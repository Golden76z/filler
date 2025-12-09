/// Optimized AI evaluator using cached calculations
/// 
/// This module provides high-performance move evaluation by leveraging
/// caching to avoid redundant heuristic calculations.

use crate::game_state::GameState;
use crate::placement::Placement;
use super::optimization::BatchScorer;
use super::heuristics::advanced_score;

/// Optimized move selection using cached batch scoring
/// 
/// For evaluating multiple placements, this is significantly faster
/// than individual scoring due to cache reuse.
pub fn select_best_placement_optimized(
    placements: &[Placement],
    game_state: &GameState,
) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }

    let mut scorer = BatchScorer::new();
    let scored = scorer.score_all(placements, game_state);

    scored
        .into_iter()
        .max_by(|a, b| {
            a.1.partial_cmp(&b.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(placement, _score)| placement)
}

/// Fast scoring for a single placement without cache overhead
/// 
/// For single placements, avoids cache initialization overhead
pub fn score_single_fast(placement: &Placement, game_state: &GameState) -> f32 {
    advanced_score(placement, game_state)
}

/// Rank placements by score using cached batch scoring
pub fn rank_placements_optimized(
    placements: &[Placement],
    game_state: &GameState,
) -> Vec<(Placement, f32)> {
    if placements.is_empty() {
        return Vec::new();
    }

    let mut scorer = BatchScorer::new();
    let mut scored = scorer.score_all(placements, game_state);
    
    // Sort by score descending
    scored.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    scored
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{Grid, Shape};

    fn create_test_game_state() -> GameState {
        let raw = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '@', '@', '.', '.'],
            vec!['.', '@', '.', '.', '.'],
            vec!['.', '.', '.', '$', '$'],
            vec!['.', '.', '.', '$', '.'],
        ];
        let grid = Grid::from_chars(5, 5, raw);
        let shape = Shape::from_chars(1, 1, vec![vec!['#']]);
        GameState::new(1, grid, shape)
    }

    fn create_test_placements() -> Vec<Placement> {
        let shape = Shape::from_chars(1, 1, vec![vec!['#']]);
        vec![
            Placement {
                position: crate::game_state::Position::new(1, 0),
                shape: shape.clone(),
                cells_added: 2,
                territory_touches: 1,
            },
            Placement {
                position: crate::game_state::Position::new(2, 0),
                shape: shape.clone(),
                cells_added: 3,
                territory_touches: 1,
            },
            Placement {
                position: crate::game_state::Position::new(0, 1),
                shape: shape,
                cells_added: 1,
                territory_touches: 2,
            },
        ]
    }

    #[test]
    fn test_select_best_placement_optimized() {
        let placements = create_test_placements();
        let game_state = create_test_game_state();

        let best = select_best_placement_optimized(&placements, &game_state);

        assert!(best.is_some());
        // Should select one of the placements
        let selected = best.unwrap();
        assert!(selected.cells_added >= 1);
    }

    #[test]
    fn test_select_best_placement_optimized_empty() {
        let placements = vec![];
        let game_state = create_test_game_state();

        let best = select_best_placement_optimized(&placements, &game_state);

        assert!(best.is_none());
    }

    #[test]
    fn test_score_single_fast() {
        let shape = Shape::from_chars(1, 1, vec![vec!['#']]);
        let placement = Placement {
            position: crate::game_state::Position::new(1, 0),
            shape,
            cells_added: 2,
            territory_touches: 1,
        };
        let game_state = create_test_game_state();

        let score = score_single_fast(&placement, &game_state);

        // Should return a positive score
        assert!(score > 0.0);
    }

    #[test]
    fn test_rank_placements_optimized() {
        let placements = create_test_placements();
        let game_state = create_test_game_state();

        let ranked = rank_placements_optimized(&placements, &game_state);

        assert_eq!(ranked.len(), placements.len());
        
        // Scores should be in descending order
        for i in 0..ranked.len() - 1 {
            assert!(ranked[i].1 >= ranked[i + 1].1);
        }
    }

    #[test]
    fn test_rank_placements_optimized_single() {
        let shape = Shape::from_chars(1, 1, vec![vec!['#']]);
        let placements = vec![Placement {
            position: crate::game_state::Position::new(1, 0),
            shape,
            cells_added: 2,
            territory_touches: 1,
        }];
        let game_state = create_test_game_state();

        let ranked = rank_placements_optimized(&placements, &game_state);

        assert_eq!(ranked.len(), 1);
    }

    #[test]
    fn test_rank_placements_optimized_empty() {
        let placements = vec![];
        let game_state = create_test_game_state();

        let ranked = rank_placements_optimized(&placements, &game_state);

        assert_eq!(ranked.len(), 0);
    }
}
