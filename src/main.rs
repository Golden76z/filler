mod parser;
mod output;
mod game_state;
mod placement;
mod utils;

use parser::parse_game_input;
use output::Move;
use game_state::{Grid, Shape, GameState};
use placement::{validate_placement, find_all_valid_placements};

fn main() {
    eprintln!("Starting Filler AI...");
    
    match parse_game_input() {
        Ok(game_input) => {
            eprintln!("Player: {}", game_input.player_number);
            eprintln!("Anfield: {} x {}", game_input.anfield.width, game_input.anfield.height);
            eprintln!("Piece: {} x {}", game_input.piece.width, game_input.piece.height);
            
            // Convert parsed input to internal game state representation
            let grid = Grid::from_chars(
                game_input.anfield.width,
                game_input.anfield.height,
                game_input.anfield.grid,
            );
            
            let shape = Shape::from_chars(
                game_input.piece.width,
                game_input.piece.height,
                game_input.piece.shape,
            );
            
            let game_state = GameState::new(game_input.player_number, grid, shape);
            
            // Debug output
            game_state.print();
            
            // Find all valid placements
            let valid_placements = find_all_valid_placements(&game_state);
            
            if valid_placements.is_empty() {
                eprintln!("No valid placements available!");
                if let Err(e) = Move::fallback().submit() {
                    eprintln!("Error submitting fallback move: {}", e);
                }
            } else {
                eprintln!("Found {} valid placements", valid_placements.len());
                
                // For Phase 3, just pick the first valid placement
                // TODO: Implement smarter selection in Phase 4
                let placement = &valid_placements[0];
                let game_move = Move::new(placement.position.x, placement.position.y);
                
                eprintln!(
                    "Selected placement at ({}, {}) - adds {} cells",
                    placement.position.x, placement.position.y, placement.cells_added
                );
                
                if let Err(e) = game_move.submit() {
                    eprintln!("Error submitting move: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            // Output fallback move when parsing fails
            if let Err(e) = Move::fallback().submit() {
                eprintln!("Error submitting fallback move: {}", e);
            }
        }
    }
}
