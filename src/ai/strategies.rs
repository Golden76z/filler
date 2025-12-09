/// AI Strategy implementations
/// 
/// Different approaches to selecting moves:
/// - Greedy expansion: Maximize territory gain
/// - Conservative: Prioritize stable positions
/// - Aggressive: Attack opponent weaknesses

use crate::placement::Placement;

/// Greedy expansion strategy
/// 
/// Prioritizes maximum territory expansion regardless of risk.
/// Best for early game where board is large and options plentiful.
pub fn greedy_expansion(placements: &[Placement]) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    // Find placement with maximum cells_added
    placements
        .iter()
        .max_by_key(|p| p.cells_added)
        .cloned()
}

/// Conservative strategy
/// 
/// Prefers placements that touch more of own territory.
/// Safer but less aggressive.
pub fn conservative(placements: &[Placement]) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    // First by territory touches, then by cells_added
    placements
        .iter()
        .max_by(|a, b| {
            match a.territory_touches.cmp(&b.territory_touches) {
                std::cmp::Ordering::Equal => a.cells_added.cmp(&b.cells_added),
                other => other,
            }
        })
        .cloned()
}

/// Edge avoidance strategy
/// 
/// Avoids placements too close to board edges.
/// Better for defending against opponent encroachment.
pub fn edge_avoidance(placements: &[Placement], grid_width: usize, grid_height: usize) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    let edge_distance = 2; // Minimum distance from edge
    
    // Filter placements away from edges
    let safe_placements: Vec<_> = placements
        .iter()
        .filter(|p| {
            p.position.x >= edge_distance
                && p.position.x < grid_width - edge_distance
                && p.position.y >= edge_distance
                && p.position.y < grid_height - edge_distance
        })
        .collect();
    
    if safe_placements.is_empty() {
        // Fallback to greedy if no safe placements
        greedy_expansion(placements)
    } else {
        safe_placements
            .iter()
            .max_by_key(|p| p.cells_added)
            .map(|p| (*p).clone())
    }
}

/// Balanced strategy (DEFAULT)
/// 
/// Combines expansion and stability.
/// Good general-purpose strategy for all phases of game.
pub fn balanced(placements: &[Placement]) -> Option<Placement> {
    if placements.is_empty() {
        return None;
    }
    
    // Score by combined metric: (cells_added * 2) + territory_touches
    placements
        .iter()
        .max_by_key(|p| (p.cells_added * 2) + p.territory_touches)
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{Position, Shape};

    fn create_placements() -> Vec<Placement> {
        let shape = Shape::from_chars(
            1, 1,
            vec![vec!['#']],
        );
        
        vec![
            Placement {
                position: Position { x: 0, y: 0 },
                shape: shape.clone(),
                cells_added: 3,
                territory_touches: 1,
            },
            Placement {
                position: Position { x: 5, y: 5 },
                shape: shape.clone(),
                cells_added: 2,
                territory_touches: 2,
            },
            Placement {
                position: Position { x: 9, y: 9 },
                shape: shape.clone(),
                cells_added: 4,
                territory_touches: 1,
            },
        ]
    }

    #[test]
    fn test_greedy_expansion_selects_max_cells() {
        let placements = create_placements();
        let result = greedy_expansion(&placements);
        
        assert!(result.is_some());
        let selected = result.unwrap();
        assert_eq!(selected.cells_added, 4);
        assert_eq!(selected.position.x, 9);
    }

    #[test]
    fn test_greedy_expansion_empty() {
        let placements: Vec<Placement> = vec![];
        let result = greedy_expansion(&placements);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_conservative_prefers_contacts() {
        let shape = Shape::from_chars(
            1, 1,
            vec![vec!['#']],
        );
        
        let placements = vec![
            Placement {
                position: Position { x: 0, y: 0 },
                shape: shape.clone(),
                cells_added: 5,
                territory_touches: 1,
            },
            Placement {
                position: Position { x: 5, y: 5 },
                shape: shape,
                cells_added: 1,
                territory_touches: 3,  // Clearly higher
            },
        ];
        
        let result = conservative(&placements);
        
        assert!(result.is_some());
        let selected = result.unwrap();
        // Should prefer placement with territory_touches = 3
        assert_eq!(selected.territory_touches, 3);
    }

    #[test]
    fn test_edge_avoidance_filters_edges() {
        let shape = Shape::from_chars(
            1, 1,
            vec![vec!['#']],
        );
        
        let placements = vec![
            Placement {
                position: Position { x: 0, y: 0 },
                shape: shape.clone(),
                cells_added: 5,
                territory_touches: 1,
            },
            Placement {
                position: Position { x: 5, y: 5 },
                shape: shape.clone(),
                cells_added: 4,
                territory_touches: 1,
            },
        ];
        
        let result = edge_avoidance(&placements, 10, 10);
        
        assert!(result.is_some());
        let selected = result.unwrap();
        // Should select the safe placement away from edges
        assert_eq!(selected.position.x, 5);
    }

    #[test]
    fn test_edge_avoidance_fallback() {
        let shape = Shape::from_chars(
            1, 1,
            vec![vec!['#']],
        );
        
        let placements = vec![
            Placement {
                position: Position { x: 0, y: 0 },
                shape: shape,
                cells_added: 5,
                territory_touches: 1,
            },
        ];
        
        let result = edge_avoidance(&placements, 10, 10);
        
        // Should fallback to greedy since no safe placements
        assert!(result.is_some());
        assert_eq!(result.unwrap().cells_added, 5);
    }

    #[test]
    fn test_balanced_strategy() {
        let shape = Shape::from_chars(
            1, 1,
            vec![vec!['#']],
        );
        
        let placements = vec![
            Placement {
                position: Position { x: 0, y: 0 },
                shape: shape.clone(),
                cells_added: 3,
                territory_touches: 1,
            },
            Placement {
                position: Position { x: 5, y: 5 },
                shape: shape,
                cells_added: 2,
                territory_touches: 3,
            },
        ];
        
        let result = balanced(&placements);
        
        assert!(result.is_some());
        // First: (3*2) + 1 = 7
        // Second: (2*2) + 3 = 7
        // Should pick one of them (tie case)
        assert!(result.is_some());
    }
}
