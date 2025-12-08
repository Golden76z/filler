mod parser;
mod output;
mod game_state;

use parser::parse_game_input;
use output::Move;
use game_state::{Grid, Shape, GameState};

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
            
            // For Phase 1, output a dummy move
            // TODO: Implement actual move selection in later phases
            let game_move = Move::new(5, 5);
            
            if let Err(e) = game_move.submit() {
                eprintln!("Error submitting move: {}", e);
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
