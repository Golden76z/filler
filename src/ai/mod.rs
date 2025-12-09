/// AI module for intelligent move selection
///
/// Provides evaluation and strategy selection for game moves.
/// Uses multiple heuristics to rank placements and select best moves.

pub mod evaluator;
pub mod strategies;
pub mod heuristics;
pub mod advanced_strategies;

use crate::game_state::GameState;
use crate::placement::Placement;
use evaluator::select_best_placement as evaluator_select;
use strategies::balanced;
use advanced_strategies::{
    aggressive_expansion, opportunistic, defensive, strategic_blocking,
    advanced_balanced, territorial_control
};

/// Strategy type enumeration
/// 
/// Determines how the AI selects moves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIStrategy {
    /// Maximize territory expansion (Phase 1)
    GreedyExpansion,
    /// Balance expansion and stability (Phase 1)
    Balanced,
    /// Use evaluation heuristics (Phase 1)
    Evaluator,
    /// Default (Phase 5: AdvancedBalanced)
    Default,
    /// Aggressive territory expansion (Phase 5)
    AggressiveExpansion,
    /// Attack weak opponent positions (Phase 5)
    Opportunistic,
    /// Consolidate territory (Phase 5)
    Defensive,
    /// Block opponent territory (Phase 5)
    StrategicBlocking,
    /// Advanced balanced with all heuristics (Phase 5)
    AdvancedBalanced,
    /// Territorial control strategy (Phase 5)
    TerritorialControl,
}

impl Default for AIStrategy {
    fn default() -> Self {
        AIStrategy::AdvancedBalanced
    }
}

/// Main AI interface for move selection
/// 
/// Selects the best move from available placements
/// using the specified strategy
pub fn select_move(
    placements: &[Placement],
    game_state: &GameState,
    strategy: AIStrategy,
) -> Option<Placement> {
    match strategy {
        // Phase 1 strategies
        AIStrategy::GreedyExpansion => strategies::greedy_expansion(placements),
        AIStrategy::Balanced => balanced(placements),
        AIStrategy::Evaluator => evaluator_select(placements, game_state),
        // Phase 5 strategies
        AIStrategy::AggressiveExpansion => aggressive_expansion(placements, game_state),
        AIStrategy::Opportunistic => opportunistic(placements, game_state),
        AIStrategy::Defensive => defensive(placements, game_state),
        AIStrategy::StrategicBlocking => strategic_blocking(placements, game_state),
        AIStrategy::AdvancedBalanced => advanced_balanced(placements, game_state),
        AIStrategy::TerritorialControl => territorial_control(placements, game_state),
        // Default is now AdvancedBalanced
        AIStrategy::Default => advanced_balanced(placements, game_state),
    }
}

/// Select move using default strategy (Evaluator)
pub fn select_move_default(
    placements: &[Placement],
    game_state: &GameState,
) -> Option<Placement> {
    select_move(placements, game_state, AIStrategy::Default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{Grid, Shape, Position};

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

    fn create_placements() -> Vec<Placement> {
        let shape = Shape::from_chars(
            1, 1,
            vec![vec!['#']],
        );
        
        vec![
            Placement {
                position: Position { x: 4, y: 5 },
                shape: shape.clone(),
                cells_added: 2,
                territory_touches: 1,
            },
            Placement {
                position: Position { x: 5, y: 6 },
                shape: shape,
                cells_added: 3,
                territory_touches: 1,
            },
        ]
    }

    #[test]
    fn test_select_move_greedy() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move(&placements, &game_state, AIStrategy::GreedyExpansion);
        
        assert!(result.is_some());
        assert_eq!(result.unwrap().cells_added, 3);
    }

    #[test]
    fn test_select_move_balanced() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move(&placements, &game_state, AIStrategy::Balanced);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_select_move_evaluator() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move(&placements, &game_state, AIStrategy::Evaluator);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_select_move_default() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move_default(&placements, &game_state);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_select_move_empty() {
        let placements: Vec<Placement> = vec![];
        let game_state = create_test_game_state();
        
        let result = select_move(&placements, &game_state, AIStrategy::Default);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_select_move_aggressive_expansion() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move(&placements, &game_state, AIStrategy::AggressiveExpansion);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_select_move_opportunistic() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move(&placements, &game_state, AIStrategy::Opportunistic);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_select_move_defensive() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move(&placements, &game_state, AIStrategy::Defensive);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_select_move_strategic_blocking() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move(&placements, &game_state, AIStrategy::StrategicBlocking);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_select_move_advanced_balanced() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move(&placements, &game_state, AIStrategy::AdvancedBalanced);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_select_move_territorial_control() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move(&placements, &game_state, AIStrategy::TerritorialControl);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_default_strategy_is_advanced_balanced() {
        let placements = create_placements();
        let game_state = create_test_game_state();
        
        let result = select_move_default(&placements, &game_state);
        
        assert!(result.is_some());
        // Default is now AdvancedBalanced instead of Evaluator
    }
}
