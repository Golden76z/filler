/// Utility functions for Filler AI
/// 
/// Common helper functions used across modules

use crate::game_state::Position;

/// Calculate Manhattan distance between two positions
pub fn manhattan_distance(a: Position, b: Position) -> usize {
    let dx = (a.x as i32 - b.x as i32).abs() as usize;
    let dy = (a.y as i32 - b.y as i32).abs() as usize;
    dx + dy
}

/// Calculate Chebyshev distance (max of absolute differences)
pub fn chebyshev_distance(a: Position, b: Position) -> usize {
    let dx = (a.x as i32 - b.x as i32).abs() as usize;
    let dy = (a.y as i32 - b.y as i32).abs() as usize;
    dx.max(dy)
}

/// Check if two positions are adjacent (4-connected)
pub fn are_adjacent_4(a: Position, b: Position) -> bool {
    manhattan_distance(a, b) == 1
}

/// Check if two positions are adjacent (8-connected)
pub fn are_adjacent_8(a: Position, b: Position) -> bool {
    chebyshev_distance(a, b) == 1
}

/// Clamp a value between min and max
pub fn clamp<T: std::cmp::PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let a = Position::new(0, 0);
        let b = Position::new(3, 4);
        assert_eq!(manhattan_distance(a, b), 7);
    }

    #[test]
    fn test_chebyshev_distance() {
        let a = Position::new(0, 0);
        let b = Position::new(3, 4);
        assert_eq!(chebyshev_distance(a, b), 4);
    }

    #[test]
    fn test_are_adjacent_4() {
        let a = Position::new(2, 2);
        
        assert!(are_adjacent_4(a, Position::new(2, 1))); // up
        assert!(are_adjacent_4(a, Position::new(2, 3))); // down
        assert!(are_adjacent_4(a, Position::new(1, 2))); // left
        assert!(are_adjacent_4(a, Position::new(3, 2))); // right
        
        assert!(!are_adjacent_4(a, Position::new(3, 3))); // diagonal
        assert!(!are_adjacent_4(a, Position::new(2, 0))); // too far
    }

    #[test]
    fn test_are_adjacent_8() {
        let a = Position::new(2, 2);
        
        assert!(are_adjacent_8(a, Position::new(2, 1))); // up
        assert!(are_adjacent_8(a, Position::new(3, 3))); // diagonal
        assert!(are_adjacent_8(a, Position::new(1, 2))); // left
        
        assert!(!are_adjacent_8(a, Position::new(4, 4))); // too far
        assert!(!are_adjacent_8(a, Position::new(2, 0))); // too far
    }
}
