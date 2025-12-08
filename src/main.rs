mod parser;
mod output;

use parser::parse_game_input;
use output::Move;

fn main() {
    // This is a simple game loop that reads input and outputs moves
    // For Phase 1, we'll just read input and output dummy coordinates
    
    eprintln!("Starting Filler AI...");
    
    match parse_game_input() {
        Ok(game_input) => {
            eprintln!("Player: {}", game_input.player_number);
            eprintln!("Anfield: {} x {}", game_input.anfield.width, game_input.anfield.height);
            eprintln!("Piece: {} x {}", game_input.piece.width, game_input.piece.height);
            
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
