/// Output/Move submission module for Filler game
/// 
/// Handles outputting moves in the format expected by the game engine: X Y\n

use std::io::{self, Write};

/// Represents a move to be submitted to the game engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub x: usize,
    pub y: usize,
}

impl Move {
    /// Create a new move with coordinates
    pub fn new(x: usize, y: usize) -> Self {
        Move { x, y }
    }

    /// Create a fallback move (0, 0) for when no valid placement exists
    pub fn fallback() -> Self {
        Move { x: 0, y: 0 }
    }

    /// Submit the move to stdout in the format expected by game engine
    pub fn submit(&self) -> io::Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{} {}", self.x, self.y)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_creation() {
        let m = Move::new(5, 5);
        assert_eq!(m.x, 5);
        assert_eq!(m.y, 5);
    }

    #[test]
    fn test_fallback_move() {
        let m = Move::fallback();
        assert_eq!(m.x, 0);
        assert_eq!(m.y, 0);
    }

    #[test]
    fn test_move_equality() {
        let m1 = Move::new(5, 5);
        let m2 = Move::new(5, 5);
        assert_eq!(m1, m2);
    }
}
