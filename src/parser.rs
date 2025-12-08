/// Input parser module for Filler game
/// 
/// This module handles parsing input from the game engine in the following format:
/// $$$ exec p<player_num> : [<player_path>]
/// Anfield W H:
///     [column indices]
/// [row_num] [grid row]
/// ...
/// Piece W H:
/// [piece grid]

use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct GameInput {
    pub player_number: u8,
    pub anfield: Anfield,
    pub piece: Piece,
}

#[derive(Debug, Clone)]
pub struct Anfield {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub shape: Vec<Vec<char>>,
}

impl Anfield {
    /// Print the anfield grid for debugging
    pub fn print(&self) {
        for row in &self.grid {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

impl Piece {
    /// Print the piece shape for debugging
    pub fn print(&self) {
        for row in &self.shape {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

/// Parse a single game input from stdin
/// 
/// # Returns
/// - `Ok(GameInput)` if parsing succeeds
/// - `Err(String)` if parsing fails with error message
pub fn parse_game_input() -> Result<GameInput, String> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();

    // Parse player identification line: $$$ exec p<number> : [<player_path>]
    reader
        .read_line(&mut line)
        .map_err(|e| format!("Failed to read player line: {}", e))?;
    
    let player_number = parse_player_line(&line)?;

    // Parse Anfield section
    let anfield = parse_anfield(&mut reader)?;

    // Parse Piece section
    let piece = parse_piece(&mut reader)?;

    Ok(GameInput {
        player_number,
        anfield,
        piece,
    })
}

/// Extract player number from the first line
/// Expected format: $$$ exec p<number> : [<player_path>]
fn parse_player_line(line: &str) -> Result<u8, String> {
    let trimmed = line.trim();
    
    // Find 'p' character and extract number after it
    if let Some(p_idx) = trimmed.find('p') {
        let after_p = &trimmed[p_idx + 1..];
        let number_str = after_p
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>();
        
        number_str
            .parse::<u8>()
            .map_err(|e| format!("Failed to parse player number: {}", e))
    } else {
        Err("Player line missing 'p' character".to_string())
    }
}

/// Parse the Anfield section
/// Expected format:
/// Anfield W H:
///     [column indices]
/// [row_num] [grid row]
/// ...
fn parse_anfield(reader: &mut dyn BufRead) -> Result<Anfield, String> {
    let mut line = String::new();

    // Read "Anfield W H:" line
    reader
        .read_line(&mut line)
        .map_err(|e| format!("Failed to read Anfield header: {}", e))?;

    let (width, height) = parse_anfield_dimensions(&line)?;

    // Read column indices line (we can skip it)
    line.clear();
    reader
        .read_line(&mut line)
        .map_err(|e| format!("Failed to read column indices: {}", e))?;

    // Read grid rows
    let mut grid = Vec::new();
    for _ in 0..height {
        line.clear();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("Failed to read grid row: {}", e))?;

        let row = parse_grid_row(&line, width)?;
        grid.push(row);
    }

    Ok(Anfield { width, height, grid })
}

/// Parse anfield dimensions from "Anfield W H:" line
fn parse_anfield_dimensions(line: &str) -> Result<(usize, usize), String> {
    let trimmed = line.trim();
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    if parts.len() < 3 {
        return Err("Invalid Anfield header format".to_string());
    }

    let width = parts[1]
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse width: {}", e))?;

    let height = parts[2]
        .trim_end_matches(':')
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse height: {}", e))?;

    Ok((width, height))
}

/// Parse a single grid row
/// Format: [row_num] [grid content]
fn parse_grid_row(line: &str, width: usize) -> Result<Vec<char>, String> {
    let trimmed = line.trim();
    
    // Find where the actual grid content starts (after row number and space)
    let grid_start = trimmed
        .find(' ')
        .ok_or("Invalid grid row format")?
        + 1;

    let grid_content = &trimmed[grid_start..];
    let row: Vec<char> = grid_content.chars().take(width).collect();

    if row.len() != width {
        return Err(format!(
            "Grid row has {} chars, expected {}",
            row.len(),
            width
        ));
    }

    Ok(row)
}

/// Parse the Piece section
/// Expected format:
/// Piece W H:
/// [piece grid]
fn parse_piece(reader: &mut dyn BufRead) -> Result<Piece, String> {
    let mut line = String::new();

    // Read "Piece W H:" line
    reader
        .read_line(&mut line)
        .map_err(|e| format!("Failed to read Piece header: {}", e))?;

    let (width, height) = parse_piece_dimensions(&line)?;

    // Read piece shape rows
    let mut shape = Vec::new();
    for _ in 0..height {
        line.clear();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("Failed to read piece row: {}", e))?;

        let row = parse_piece_row(&line, width)?;
        shape.push(row);
    }

    Ok(Piece {
        width,
        height,
        shape,
    })
}

/// Parse piece dimensions from "Piece W H:" line
fn parse_piece_dimensions(line: &str) -> Result<(usize, usize), String> {
    let trimmed = line.trim();
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    if parts.len() < 3 {
        return Err("Invalid Piece header format".to_string());
    }

    let width = parts[1]
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse piece width: {}", e))?;

    let height = parts[2]
        .trim_end_matches(':')
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse piece height: {}", e))?;

    Ok((width, height))
}

/// Parse a single piece row
fn parse_piece_row(line: &str, width: usize) -> Result<Vec<char>, String> {
    let trimmed = line.trim();
    let row: Vec<char> = trimmed.chars().take(width).collect();

    if row.len() != width {
        return Err(format!(
            "Piece row has {} chars, expected {}",
            row.len(),
            width
        ));
    }

    Ok(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_player_line() {
        let line = "$$$ exec p1 : [robots/bender]";
        assert_eq!(parse_player_line(line).unwrap(), 1);

        let line2 = "$$$ exec p2 : [robots/terminator]";
        assert_eq!(parse_player_line(line2).unwrap(), 2);
    }

    #[test]
    fn test_parse_anfield_dimensions() {
        let line = "Anfield 20 15:";
        let (w, h) = parse_anfield_dimensions(line).unwrap();
        assert_eq!(w, 20);
        assert_eq!(h, 15);
    }

    #[test]
    fn test_parse_piece_dimensions() {
        let line = "Piece 4 1:";
        let (w, h) = parse_piece_dimensions(line).unwrap();
        assert_eq!(w, 4);
        assert_eq!(h, 1);
    }

    #[test]
    fn test_parse_grid_row() {
        let line = "002 .........@..........";
        let row = parse_grid_row(line, 20).unwrap();
        assert_eq!(row.len(), 20);
        assert_eq!(row[9], '@');
    }

    #[test]
    fn test_parse_piece_row() {
        let line = ".OO.";
        let row = parse_piece_row(line, 4).unwrap();
        assert_eq!(row.len(), 4);
        assert_eq!(row[1], 'O');
        assert_eq!(row[2], 'O');
    }
}
