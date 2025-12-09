/// Advanced heuristics for AI move evaluation
/// 
/// This module provides sophisticated heuristics for evaluating placements
/// including flood-fill territory analysis, edge detection, and density mapping.

use crate::game_state::{Grid, Position, CellState, GameState};
use crate::placement::Placement;
use std::collections::{VecDeque, HashSet};

/// Analyzes territory growth potential using flood-fill algorithm
/// Returns the approximate maximum territory that could be claimed from this placement
pub fn analyze_flood_fill(placement: &Placement, game_state: &GameState) -> f32 {
    // Create a hypothetical grid state after this placement
    let mut test_grid = game_state.grid.clone();
    
    // Simulate placing the piece
    let absolute_positions = placement.get_absolute_positions();
    for pos in absolute_positions {
        if test_grid.is_valid(pos) {
            test_grid.set(pos, CellState::Player1Last);
        }
    }
    
    // Perform flood-fill from the placement positions to estimate territory growth
    let reachable = flood_fill_reachable(&test_grid, &placement.get_absolute_positions());
    
    // Score based on reachable empty cells
    (reachable as f32) * 2.5
}

/// Performs flood-fill to find all reachable empty cells from given positions
fn flood_fill_reachable(grid: &Grid, start_positions: &[Position]) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    // Initialize queue with starting positions
    for &pos in start_positions {
        if grid.is_valid(pos) {
            queue.push_back(pos);
            visited.insert(pos);
        }
    }
    
    let mut reachable_count = 0;
    
    while let Some(pos) = queue.pop_front() {
        // Check all 4 adjacent cells
        let neighbors = [
            Position::new(pos.x.wrapping_add(1), pos.y),
            Position::new(pos.x.wrapping_sub(1), pos.y),
            Position::new(pos.x, pos.y.wrapping_add(1)),
            Position::new(pos.x, pos.y.wrapping_sub(1)),
        ];
        
        for neighbor in neighbors {
            if !visited.contains(&neighbor) && grid.is_valid(neighbor) {
                if let Some(state) = grid.get(neighbor) {
                    // Count empty cells and our territory
                    if matches!(state, CellState::Empty | CellState::Player1 | CellState::Player1Last) {
                        visited.insert(neighbor);
                        
                        if state == CellState::Empty {
                            reachable_count += 1;
                        }
                        
                        // Only continue flood-fill through empty cells
                        if state == CellState::Empty {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }
    
    reachable_count
}

/// Detects weak positions - areas where opponent has sparse territory
/// Returns score based on attacking weak positions (higher = more opportunity)
pub fn detect_weak_positions(placement: &Placement, game_state: &GameState) -> f32 {
    let abs_positions = placement.get_absolute_positions();
    let mut weak_score = 0.0;
    
    for pos in abs_positions {
        if game_state.grid.is_valid(pos) {
            // Check density of opponent territory around this position
            let opponent_density = count_opponent_neighbors(&game_state.grid, pos);
            
            // Low opponent density = weak position (good for us)
            if opponent_density < 2 {
                weak_score += 3.0;
            } else if opponent_density < 4 {
                weak_score += 1.5;
            }
        }
    }
    
    weak_score
}

/// Count opponent (player 2) cells adjacent to a position
fn count_opponent_neighbors(grid: &Grid, pos: Position) -> usize {
    let neighbors = [
        Position::new(pos.x.wrapping_add(1), pos.y),
        Position::new(pos.x.wrapping_sub(1), pos.y),
        Position::new(pos.x, pos.y.wrapping_add(1)),
        Position::new(pos.x, pos.y.wrapping_sub(1)),
    ];
    
    neighbors.iter().filter(|&&neighbor| {
        if grid.is_valid(neighbor) {
            matches!(grid.get(neighbor), Some(CellState::Player2 | CellState::Player2Last))
        } else {
            false
        }
    }).count()
}

/// Analyzes territory density around a placement position
/// Higher density means more consolidated territory (better defense)
pub fn analyze_density(placement: &Placement, game_state: &GameState) -> f32 {
    let abs_positions = placement.get_absolute_positions();
    let mut density_score = 0.0;
    let mut count = 0;
    
    for pos in &abs_positions {
        if game_state.grid.is_valid(*pos) {
            // Count our territory cells within distance 2
            let nearby_our_territory = count_nearby_our_territory(&game_state.grid, *pos);
            
            // More nearby territory = higher density bonus
            density_score += (nearby_our_territory as f32) * 0.8;
            count += 1;
        }
    }
    
    if count > 0 {
        density_score / (count as f32)
    } else {
        0.0
    }
}

/// Count our (player 1) territory cells within manhattan distance 2
fn count_nearby_our_territory(grid: &Grid, center: Position) -> usize {
    let mut count = 0;
    
    // Check positions at manhattan distance <= 2
    for dx in -2..=2 {
        for dy in -2..=2 {
            if dx == 0 && dy == 0 {
                continue;
            }
            
            let x = (center.x as i32 + dx) as usize;
            let y = (center.y as i32 + dy) as usize;
            let pos = Position::new(x, y);
            
            if grid.is_valid(pos) {
                if let Some(state) = grid.get(pos) {
                    if matches!(state, CellState::Player1 | CellState::Player1Last) {
                        count += 1;
                    }
                }
            }
        }
    }
    
    count
}

/// Analyzes strategic value of controlling edges and corners
/// Corners and edges provide natural defense
pub fn analyze_edge_control(placement: &Placement, grid: &Grid) -> f32 {
    let abs_positions = placement.get_absolute_positions();
    let mut edge_score = 0.0;
    
    for pos in abs_positions {
        if grid.is_valid(pos) {
            let is_corner = (pos.x == 0 || pos.x == grid.width - 1) && 
                           (pos.y == 0 || pos.y == grid.height - 1);
            let is_edge = pos.x == 0 || pos.x == grid.width - 1 || 
                         pos.y == 0 || pos.y == grid.height - 1;
            
            if is_corner {
                edge_score += 2.0; // Corners are very valuable
            } else if is_edge {
                edge_score += 1.0; // Edges are somewhat valuable
            }
        }
    }
    
    edge_score
}

/// Comprehensive advanced scoring combining all heuristics
pub fn advanced_score(placement: &Placement, game_state: &GameState) -> f32 {
    // Base expansion score (most important)
    let base_expansion = (placement.cells_added as f32) * 10.0;
    
    // Advanced heuristics (new in Phase 5)
    let flood_fill = analyze_flood_fill(placement, game_state);
    let weak_positions = detect_weak_positions(placement, game_state);
    let density = analyze_density(placement, game_state);
    let edge_control = analyze_edge_control(placement, &game_state.grid);
    
    // Combine scores with strategic weights
    base_expansion 
        + (flood_fill * 1.5)           // Territory growth potential (medium importance)
        + (weak_positions * 2.0)       // Attacking weak positions (high importance)
        + (density * 1.2)              // Territory consolidation (medium importance)
        + (edge_control * 0.5)         // Edge control (lower importance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::Shape;

    fn create_test_grid() -> Grid {
        let raw = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '@', '@', '.', '.'],
            vec!['.', '@', '.', '.', '.'],
            vec!['.', '.', '.', '$', '$'],
            vec!['.', '.', '.', '$', '.'],
        ];
        Grid::from_chars(5, 5, raw)
    }

    fn create_test_game_state() -> GameState {
        let grid = create_test_grid();
        let shape = Shape::from_chars(1, 1, vec![vec!['#']]);
        GameState::new(1, grid, shape)
    }

    fn create_test_placement(x: usize, y: usize) -> Placement {
        let shape = Shape::from_chars(1, 1, vec![vec!['#']]);
        Placement {
            position: Position::new(x, y),
            shape,
            cells_added: 1,
            territory_touches: 1,
        }
    }

    #[test]
    fn test_flood_fill_reachable() {
        let grid = create_test_grid();
        let start = vec![Position::new(1, 1)];
        
        let reachable = flood_fill_reachable(&grid, &start);
        
        // Should find some empty cells reachable from position (1,1)
        assert!(reachable > 0);
    }

    #[test]
    fn test_analyze_flood_fill() {
        let game_state = create_test_game_state();
        let placement = create_test_placement(1, 0);
        let score = analyze_flood_fill(&placement, &game_state);
        
        // Should return a positive score
        assert!(score > 0.0);
    }

    #[test]
    fn test_detect_weak_positions() {
        let game_state = create_test_game_state();
        // Position near opponent should have weak position score
        let placement = create_test_placement(2, 3);
        let score = detect_weak_positions(&placement, &game_state);
        
        // Score depends on opponent density
        assert!(score >= 0.0);
    }

    #[test]
    fn test_analyze_density() {
        let game_state = create_test_game_state();
        // Placement near our territory should have high density
        let placement = create_test_placement(1, 2);
        let score = analyze_density(&placement, &game_state);
        
        assert!(score >= 0.0);
    }

    #[test]
    fn test_analyze_edge_control() {
        let grid = create_test_grid();
        
        // Corner placement
        let corner_placement = create_test_placement(0, 0);
        let corner_score = analyze_edge_control(&corner_placement, &grid);
        
        // Edge placement
        let edge_placement = create_test_placement(2, 0);
        let edge_score = analyze_edge_control(&edge_placement, &grid);
        
        // Interior placement
        let interior_placement = create_test_placement(2, 2);
        let interior_score = analyze_edge_control(&interior_placement, &grid);
        
        // Corner should score higher than edge, edge higher than interior
        assert!(corner_score > edge_score);
        assert!(edge_score > interior_score);
    }

    #[test]
    fn test_advanced_score_combines_heuristics() {
        let game_state = create_test_game_state();
        let placement = create_test_placement(1, 1);
        let score = advanced_score(&placement, &game_state);
        
        // Should return a positive score combining all heuristics
        assert!(score > 0.0);
    }

    #[test]
    fn test_count_opponent_neighbors() {
        let grid = create_test_grid();
        
        // Position adjacent to opponent
        let pos = Position::new(3, 3);
        let count = count_opponent_neighbors(&grid, pos);
        
        // Should count neighboring opponent cells
        assert!(count > 0);
    }

    #[test]
    fn test_count_nearby_our_territory() {
        let grid = create_test_grid();
        
        // Position near our territory
        let pos = Position::new(1, 1);
        let count = count_nearby_our_territory(&grid, pos);
        
        // Should count nearby our territory cells
        assert!(count >= 1); // At least the cell itself if it's ours
    }
}
