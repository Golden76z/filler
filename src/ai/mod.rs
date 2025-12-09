/// AI module for intelligent move selection
///
/// Provides evaluation and strategy selection for game moves.
/// Uses multiple heuristics to rank placements and select best moves.

pub mod evaluator;
pub mod strategies;

use crate::game_state::GameState;
use crate::placement::Placement;
use evaluator::select_best_placement as evaluator_select;
use strategies::balanced;

/// Strategy type enumeration
/// 
/// Determines how the AI selects moves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIStrategy {
    /// Maximize territory expansion
    GreedyExpansion,
    /// Balance expansion and stability
    Balanced,
    /// Use evaluation heuristics
    Evaluator,
    /// Default (currently Evaluator)
    Default,
}

impl Default for AIStrategy {
    fn default() -> Self {
        AIStrategy::Evaluator
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
        AIStrategy::GreedyExpansion => strategies::greedy_expansion(placements),
        AIStrategy::Balanced => balanced(placements),
        AIStrategy::Evaluator => evaluator_select(placements, game_state),
        AIStrategy::Default => evaluator_select(placements, game_state),
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
}
