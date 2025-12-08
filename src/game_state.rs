/// Game state representation module
/// 
/// This module provides the core data structures for representing
/// the game state during a Filler game.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Empty,      // Empty cell (.)
    Player1,    // Player 1 territory (@)
    Player2,    // Player 2 territory ($)
    Player1Last, // Last piece placed by Player 1 (a)
    Player2Last, // Last piece placed by Player 2 (s)
}

impl From<char> for CellState {
    fn from(c: char) -> Self {
        match c {
            '.' => CellState::Empty,
            '@' => CellState::Player1,
            '$' => CellState::Player2,
            'a' => CellState::Player1Last,
            's' => CellState::Player2Last,
            _ => CellState::Empty, // Default to empty for unknown chars
        }
    }
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            CellState::Empty => '.',
            CellState::Player1 => '@',
            CellState::Player2 => '$',
            CellState::Player1Last => 'a',
            CellState::Player2Last => 's',
        };
        write!(f, "{}", c)
    }
}

/// Represents a position on the Anfield
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

/// Represents the Anfield grid with cell states
#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<CellState>>,
}

impl Grid {
    /// Create a new grid from raw character data
    pub fn from_chars(width: usize, height: usize, raw: Vec<Vec<char>>) -> Self {
        let cells = raw
            .into_iter()
            .map(|row| row.into_iter().map(CellState::from).collect())
            .collect();

        Grid {
            width,
            height,
            cells,
        }
    }

    /// Get cell state at position
    pub fn get(&self, pos: Position) -> Option<CellState> {
        if pos.x < self.width && pos.y < self.height {
            Some(self.cells[pos.y][pos.x])
        } else {
            None
        }
    }

    /// Set cell state at position
    pub fn set(&mut self, pos: Position, state: CellState) -> bool {
        if pos.x < self.width && pos.y < self.height {
            self.cells[pos.y][pos.x] = state;
            true
        } else {
            false
        }
    }

    /// Check if a position is within bounds
    pub fn is_valid(&self, pos: Position) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    /// Get all positions occupied by player territory (including last piece)
    pub fn get_player_positions(&self, player_num: u8) -> Vec<Position> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let state = self.cells[y][x];
                let is_player = match player_num {
                    1 => state == CellState::Player1 || state == CellState::Player1Last,
                    2 => state == CellState::Player2 || state == CellState::Player2Last,
                    _ => false,
                };
                if is_player {
                    positions.push(Position::new(x, y));
                }
            }
        }
        positions
    }

    /// Get all empty positions
    pub fn get_empty_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x] == CellState::Empty {
                    positions.push(Position::new(x, y));
                }
            }
        }
        positions
    }

    /// Count territory for a player
    pub fn count_territory(&self, player_num: u8) -> usize {
        self.get_player_positions(player_num).len()
    }

    /// Print the grid for debugging
    pub fn print(&self) {
        eprintln!("=== Grid: {} x {} ===", self.width, self.height);
        for (y, row) in self.cells.iter().enumerate() {
            eprint!("{:03} ", y);
            for cell in row {
                eprint!("{}", cell);
            }
            eprintln!();
        }
    }
}

/// Represents a piece shape
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>, // true = filled, false = empty
}

impl Shape {
    /// Create a new shape from raw character data
    pub fn from_chars(width: usize, height: usize, raw: Vec<Vec<char>>) -> Self {
        let cells = raw
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| c != '.')
                    .collect()
            })
            .collect();

        Shape {
            width,
            height,
            cells,
        }
    }

    /// Get all filled cell positions relative to top-left (0, 0)
    pub fn get_filled_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x] {
                    positions.push(Position::new(x, y));
                }
            }
        }
        positions
    }

    /// Check if the shape has any filled cells
    pub fn is_empty(&self) -> bool {
        self.get_filled_positions().is_empty()
    }

    /// Get bounding box of the filled cells
    pub fn bounding_box(&self) -> Option<(usize, usize, usize, usize)> {
        let positions = self.get_filled_positions();
        if positions.is_empty() {
            return None;
        }

        let min_x = positions.iter().map(|p| p.x).min().unwrap();
        let max_x = positions.iter().map(|p| p.x).max().unwrap();
        let min_y = positions.iter().map(|p| p.y).min().unwrap();
        let max_y = positions.iter().map(|p| p.y).max().unwrap();

        Some((min_x, min_y, max_x - min_x + 1, max_y - min_y + 1))
    }

    /// Print the shape for debugging
    pub fn print(&self) {
        eprintln!("=== Shape: {} x {} ===", self.width, self.height);
        for row in &self.cells {
            for &filled in row {
                eprint!("{}", if filled { '#' } else { '.' });
            }
            eprintln!();
        }
    }
}

/// Represents the complete game state
#[derive(Debug, Clone)]
pub struct GameState {
    pub player_number: u8,
    pub grid: Grid,
    pub current_piece: Shape,
}

impl GameState {
    /// Create a new game state
    pub fn new(player_number: u8, grid: Grid, current_piece: Shape) -> Self {
        GameState {
            player_number,
            grid,
            current_piece,
        }
    }

    /// Get all positions belonging to the current player
    pub fn get_my_positions(&self) -> Vec<Position> {
        self.grid.get_player_positions(self.player_number)
    }

    /// Get all positions belonging to the opponent
    pub fn get_opponent_positions(&self) -> Vec<Position> {
        let opponent = if self.player_number == 1 { 2 } else { 1 };
        self.grid.get_player_positions(opponent)
    }

    /// Get current territory size for current player
    pub fn get_my_territory_size(&self) -> usize {
        self.grid.count_territory(self.player_number)
    }

    /// Get opponent territory size
    pub fn get_opponent_territory_size(&self) -> usize {
        let opponent = if self.player_number == 1 { 2 } else { 1 };
        self.grid.count_territory(opponent)
    }

    /// Print game state for debugging
    pub fn print(&self) {
        eprintln!("\n=== Game State ===");
        eprintln!("Player: {}", self.player_number);
        eprintln!(
            "My Territory: {} | Opponent Territory: {}",
            self.get_my_territory_size(),
            self.get_opponent_territory_size()
        );
        self.grid.print();
        eprintln!();
        self.current_piece.print();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_state_from_char() {
        assert_eq!(CellState::from('.'), CellState::Empty);
        assert_eq!(CellState::from('@'), CellState::Player1);
        assert_eq!(CellState::from('$'), CellState::Player2);
        assert_eq!(CellState::from('a'), CellState::Player1Last);
        assert_eq!(CellState::from('s'), CellState::Player2Last);
    }

    #[test]
    fn test_position() {
        let p = Position::new(5, 10);
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 10);
    }

    #[test]
    fn test_grid_creation() {
        let raw = vec![
            vec!['.', '@', '.'],
            vec!['.', '.', '.'],
            vec!['.', '$', '.'],
        ];
        let grid = Grid::from_chars(3, 3, raw);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
    }

    #[test]
    fn test_grid_get_set() {
        let raw = vec![vec!['.'; 3]; 3];
        let mut grid = Grid::from_chars(3, 3, raw);

        let pos = Position::new(1, 1);
        grid.set(pos, CellState::Player1);
        assert_eq!(grid.get(pos), Some(CellState::Player1));
    }

    #[test]
    fn test_grid_bounds() {
        let raw = vec![vec!['.'; 3]; 3];
        let grid = Grid::from_chars(3, 3, raw);

        assert!(grid.is_valid(Position::new(0, 0)));
        assert!(grid.is_valid(Position::new(2, 2)));
        assert!(!grid.is_valid(Position::new(3, 3)));
    }

    #[test]
    fn test_shape_from_chars() {
        let raw = vec![vec!['.', '#'], vec!['#', '.']];
        let shape = Shape::from_chars(2, 2, raw);
        let filled = shape.get_filled_positions();
        assert_eq!(filled.len(), 2);
    }

    #[test]
    fn test_shape_bounding_box() {
        let raw = vec![
            vec!['.', '#', '.'],
            vec!['#', '#', '.'],
            vec!['.', '.', '.'],
        ];
        let shape = Shape::from_chars(3, 3, raw);
        let bbox = shape.bounding_box().unwrap();
        assert_eq!(bbox, (0, 0, 2, 2)); // (min_x, min_y, width, height)
    }
}
