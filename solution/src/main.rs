use std::io::{self, BufRead, Write};

use filler::game_state::Player;
use filler::output::{format_move, format_pass};
use filler::parser::parse_turn;
use filler::strategy::choose_placement;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut iter = stdin.lock().lines().filter_map(|r| r.ok());

    let mut player_hint: Option<Player> = None;

    loop {
        let turn = match parse_turn(&mut iter, player_hint) {
            Some(t) => t,
            None => break,
        };
        player_hint = Some(turn.player);

        let reply = match choose_placement(turn.player, &turn.anfield, &turn.piece) {
            Some(p) => format_move(p),
            None => format_pass(),
        };

        if out.write_all(reply.as_bytes()).is_err() {
            break;
        }
        if out.flush().is_err() {
            break;
        }
    }
}
