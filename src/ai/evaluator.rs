/// Move evaluation and scoring module
/// 
/// Evaluates piece placements based on various heuristics:
/// - Territory expansion (primary score)
/// - Center of mass bias (secondary)
/// - Adjacency count (secondary)

use crate::game_state::{GameState, Position, Grid, Shape, CellState};
use crate::placement::Placement;
use crate::utils::manhattan_distance;

/// Score a single placement
/// 
/// Scoring factors:
/// 1. Territory expansion (cells_added) - PRIMARY
/// 2. Position centrality (distance from board center) - SECONDARY
/// 3. Adjacency count (touching own territory) - TERTIARY
pub fn evaluate_placement(placement: &Placement, game_state: &GameState) -> f32 {
    // Primary score: Territory expansion
    // Each cell added is worth base points
    let expansion_score = placement.cells_added as f32 * 10.0;
    
    // Secondary score: Centrality bonus
    // Placements near board center get slight bonus
    let center_x = game_state.grid.width as i32 / 2;
    let center_y = game_state.grid.height as i32 / 2;
    let distance_to_center = manhattan_distance(
        placement.position,
        Position { x: center_x as usize, y: center_y as usize },
    );
    let centrality_bonus = if distance_to_center < 15 {
        (15 - distance_to_center) as f32 * 0.5
    } else {
        0.0
    };
    
    // Tertiary score: Adjacency bonus
    // Placements touching more of own territory get bonus
    let adjacency_bonus = (placement.territory_touches as f32) * 1.0;
    
    // Total score combines all factors
    let total_score = expansion_score + centrality_bonus + adjacency_bonus;
    
    total_score
}

/// Rank placements by their evaluation score
/// 
/// Returns placements sorted from highest to lowest score
pub fn rank_placements(
    placements: &[Placement],
    game_state: &GameState,
) -> Vec<(Placement, f32)> {
    let mut scored: Vec<(Placement, f32)> = placements
        .iter()
        .map(|p| (p.clone(), evaluate_placement(p, game_state)))
        .collect();
    
    // Sort by score descending
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    scored
}

/// Select the best placement based on evaluation
/// 
/// Returns the highest-scoring placement, or the first valid placement
/// if all scores are equal
pub fn select_best_placement(
    placements: &[Placement],
    game_state: &GameState,
) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    let ranked = rank_placements(placements, game_state);
    ranked.first().map(|(placement, _)| placement.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_game_state() -> GameState {
        let grid = Grid::from_chars(
            10, 10,
            (0..10).map(|_| (0..10).map(|_| '.').collect()).collect(),
        );
        
        let shape = Shape::from_chars(
            2, 2,
            vec![vec!['.', '#'], vec!['#', '.']],
        );
        
        GameState::new(1, grid, shape)
    }

    #[test]
    fn test_evaluate_placement_expansion() {
        let game_state = create_test_game_state();
        let placement = Placement {
            position: Position { x: 4, y: 5 },
            shape: game_state.current_piece.clone(),
            cells_added: 2,
            territory_touches: 1,
        };
        
        let score = evaluate_placement(&placement, &game_state);
        // Should be significant due to cells_added
        assert!(score > 15.0);
    }

    #[test]
    fn test_evaluate_placement_near_center() {
        let game_state = create_test_game_state();
        
        let placement_center = Placement {
            position: Position { x: 5, y: 5 },
            shape: game_state.current_piece.clone(),
            cells_added: 1,
            territory_touches: 1,
        };
        
        let placement_edge = Placement {
            position: Position { x: 0, y: 0 },
            shape: game_state.current_piece.clone(),
            cells_added: 1,
            territory_touches: 1,
        };
        
        let score_center = evaluate_placement(&placement_center, &game_state);
        let score_edge = evaluate_placement(&placement_edge, &game_state);
        
        // Center placement should score higher
        assert!(score_center > score_edge);
    }

    #[test]
    fn test_rank_placements_order() {
        let game_state = create_test_game_state();
        
        let placements = vec![
            Placement {
                position: Position { x: 4, y: 5 },
                shape: game_state.current_piece.clone(),
                cells_added: 1,
                territory_touches: 1,
            },
            Placement {
                position: Position { x: 4, y: 6 },
                shape: game_state.current_piece.clone(),
                cells_added: 3,
                territory_touches: 1,
            },
            Placement {
                position: Position { x: 6, y: 5 },
                shape: game_state.current_piece.clone(),
                cells_added: 2,
                territory_touches: 1,
            },
        ];
        
        let ranked = rank_placements(&placements, &game_state);
        
        // Should be ordered by score
        assert_eq!(ranked.len(), 3);
        assert!(ranked[0].1 >= ranked[1].1);
        assert!(ranked[1].1 >= ranked[2].1);
    }

    #[test]
    fn test_select_best_placement() {
        let game_state = create_test_game_state();
        
        let placements = vec![
            Placement {
                position: Position { x: 4, y: 5 },
                shape: game_state.current_piece.clone(),
                cells_added: 1,
                territory_touches: 1,
            },
            Placement {
                position: Position { x: 4, y: 6 },
                shape: game_state.current_piece.clone(),
                cells_added: 3,
                territory_touches: 1,
            },
        ];
        
        let best = select_best_placement(&placements, &game_state);
        
        assert!(best.is_some());
        let best_placement = best.unwrap();
        // Best should have more cells_added
        assert_eq!(best_placement.cells_added, 3);
    }

    #[test]
    fn test_select_best_placement_empty() {
        let game_state = create_test_game_state();
        let placements: Vec<Placement> = vec![];
        
        let best = select_best_placement(&placements, &game_state);
        assert!(best.is_none());
    }
}
