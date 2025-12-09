/// Advanced AI strategies using complex heuristics
/// 
/// This module provides advanced move selection strategies that use
/// sophisticated analysis including predictive blocking, territory control,
/// and opponent modeling.

use crate::game_state::GameState;
use crate::placement::Placement;
use super::heuristics::{
    analyze_flood_fill, detect_weak_positions, analyze_density, 
    analyze_edge_control, advanced_score
};

/// Aggressive expansion strategy that prioritizes growth potential
pub fn aggressive_expansion(placements: &[Placement], game_state: &GameState) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    placements
        .iter()
        .max_by(|a, b| {
            let score_a = (a.cells_added as f32) * 10.0 
                + analyze_flood_fill(a, game_state) * 2.0;
            let score_b = (b.cells_added as f32) * 10.0 
                + analyze_flood_fill(b, game_state) * 2.0;
            
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}

/// Opportunistic strategy that attacks weak opponent positions
pub fn opportunistic(placements: &[Placement], game_state: &GameState) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    placements
        .iter()
        .max_by(|a, b| {
            let score_a = detect_weak_positions(a, game_state) * 2.5
                + (a.cells_added as f32) * 5.0;
            let score_b = detect_weak_positions(b, game_state) * 2.5
                + (b.cells_added as f32) * 5.0;
            
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}

/// Defensive strategy that consolidates territory and maximizes density
pub fn defensive(placements: &[Placement], game_state: &GameState) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    placements
        .iter()
        .max_by(|a, b| {
            let score_a = analyze_density(a, game_state) * 2.0
                + (a.territory_touches as f32) * 2.0
                + analyze_edge_control(a, &game_state.grid) * 1.5;
            let score_b = analyze_density(b, game_state) * 2.0
                + (b.territory_touches as f32) * 2.0
                + analyze_edge_control(b, &game_state.grid) * 1.5;
            
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}

/// Strategic blocking strategy that tries to deny opponent territory
pub fn strategic_blocking(placements: &[Placement], game_state: &GameState) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    placements
        .iter()
        .max_by(|a, b| {
            // Prioritize positions that block opponent from expanding
            // by maximizing weak position detection (offensive blocking)
            // combined with territory touch count (defensive blocking)
            let score_a = detect_weak_positions(a, game_state) * 1.8
                + (a.territory_touches as f32) * 3.0
                + (a.cells_added as f32) * 3.0;
            let score_b = detect_weak_positions(b, game_state) * 1.8
                + (b.territory_touches as f32) * 3.0
                + (b.cells_added as f32) * 3.0;
            
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}

/// Advanced balanced strategy using all heuristics
/// This is the new default strategy for Phase 5
pub fn advanced_balanced(placements: &[Placement], game_state: &GameState) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    placements
        .iter()
        .max_by(|a, b| {
            let score_a = advanced_score(a, game_state);
            let score_b = advanced_score(b, game_state);
            
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}

/// Territorial control strategy that balances multiple objectives
pub fn territorial_control(placements: &[Placement], game_state: &GameState) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    placements
        .iter()
        .max_by(|a, b| {
            let score_a = (a.cells_added as f32) * 8.0
                + analyze_flood_fill(a, game_state) * 1.5
                + (a.territory_touches as f32) * 1.5
                + analyze_edge_control(a, &game_state.grid) * 0.8;
            let score_b = (b.cells_added as f32) * 8.0
                + analyze_flood_fill(b, game_state) * 1.5
                + (b.territory_touches as f32) * 1.5
                + analyze_edge_control(b, &game_state.grid) * 0.8;
            
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::Shape;

    fn create_test_placement(x: usize, y: usize, cells: usize, touches: usize) -> Placement {
        Placement {
            position: crate::game_state::Position::new(x, y),
            shape: Shape::from_chars(1, 1, vec![vec!['#']]),
            cells_added: cells,
            territory_touches: touches,
        }
    }

    fn create_test_game_state() -> GameState {
        let raw = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '@', '@', '.', '.'],
            vec!['.', '@', '.', '.', '.'],
            vec!['.', '.', '.', '$', '$'],
            vec!['.', '.', '.', '$', '.'],
        ];
        let grid = crate::game_state::Grid::from_chars(5, 5, raw);
        let shape = Shape::from_chars(1, 1, vec![vec!['#']]);
        GameState::new(1, grid, shape)
    }

    #[test]
    fn test_aggressive_expansion() {
        let game_state = create_test_game_state();
        let placements = vec![
            create_test_placement(1, 1, 3, 1),
            create_test_placement(0, 0, 1, 1),
            create_test_placement(2, 2, 2, 2),
        ];
        
        let best = aggressive_expansion(&placements, &game_state);
        assert!(best.is_some());
        
        let selected = best.unwrap();
        assert_eq!(selected.cells_added, 3);
    }

    #[test]
    fn test_aggressive_expansion_empty() {
        let game_state = create_test_game_state();
        let placements = vec![];
        
        let best = aggressive_expansion(&placements, &game_state);
        assert!(best.is_none());
    }

    #[test]
    fn test_opportunistic() {
        let game_state = create_test_game_state();
        let placements = vec![
            create_test_placement(1, 1, 2, 1),
            create_test_placement(3, 3, 1, 2),
            create_test_placement(2, 2, 1, 1),
        ];
        
        let best = opportunistic(&placements, &game_state);
        assert!(best.is_some());
    }

    #[test]
    fn test_defensive() {
        let game_state = create_test_game_state();
        let placements = vec![
            create_test_placement(1, 1, 1, 2),
            create_test_placement(0, 0, 1, 1),
            create_test_placement(2, 2, 2, 1),
        ];
        
        let best = defensive(&placements, &game_state);
        assert!(best.is_some());
    }

    #[test]
    fn test_strategic_blocking() {
        let game_state = create_test_game_state();
        let placements = vec![
            create_test_placement(1, 1, 2, 2),
            create_test_placement(3, 3, 1, 3),
            create_test_placement(2, 2, 1, 1),
        ];
        
        let best = strategic_blocking(&placements, &game_state);
        assert!(best.is_some());
    }

    #[test]
    fn test_advanced_balanced() {
        let game_state = create_test_game_state();
        let placements = vec![
            create_test_placement(1, 1, 2, 1),
            create_test_placement(0, 0, 1, 1),
            create_test_placement(2, 2, 2, 2),
        ];
        
        let best = advanced_balanced(&placements, &game_state);
        assert!(best.is_some());
    }

    #[test]
    fn test_territorial_control() {
        let game_state = create_test_game_state();
        let placements = vec![
            create_test_placement(1, 1, 3, 1),
            create_test_placement(0, 0, 1, 1),
            create_test_placement(2, 2, 2, 2),
        ];
        
        let best = territorial_control(&placements, &game_state);
        assert!(best.is_some());
    }

    #[test]
    fn test_all_strategies_handle_single_placement() {
        let game_state = create_test_game_state();
        let placements = vec![create_test_placement(1, 1, 2, 1)];
        
        assert!(aggressive_expansion(&placements, &game_state).is_some());
        assert!(opportunistic(&placements, &game_state).is_some());
        assert!(defensive(&placements, &game_state).is_some());
        assert!(strategic_blocking(&placements, &game_state).is_some());
        assert!(advanced_balanced(&placements, &game_state).is_some());
        assert!(territorial_control(&placements, &game_state).is_some());
    }
}
