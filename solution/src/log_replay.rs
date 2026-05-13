//! Parses the verbose output of `game_engine` (a saved `game.log`) into a
//! sequence of board frames suitable for replay UIs.
//!
//! The engine emits, per turn, a block that starts with `Anfield W H:` followed
//! by an optional column-index line and then `H` row lines (each prefixed with
//! a 3-digit row index). We collect those into a `Frame` and skip everything
//! else (`Piece ...`, `-> Answer (...)`, `seed:`, `Player1 ... won!`, ...).

use std::io::BufRead;

#[derive(Clone, Debug)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub rows: Vec<String>,
}

pub fn parse_all_frames<R: BufRead>(mut reader: R) -> Vec<Frame> {
    let mut frames = Vec::new();
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }
        let trimmed = line.trim_end_matches(&['\n', '\r'][..]);
        if !trimmed.starts_with("Anfield ") {
            continue;
        }

        let dims = trimmed
            .trim_start_matches("Anfield ")
            .trim_end_matches(':');
        let parts: Vec<&str> = dims.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let width: usize = match parts[0].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let height: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let mut peek = String::new();
        if reader.read_line(&mut peek).unwrap_or(0) == 0 {
            break;
        }
        let peek_trim = peek.trim_end_matches(&['\n', '\r'][..]).trim_start();
        let column_header =
            !peek_trim.is_empty() && peek_trim.chars().all(|c| c.is_ascii_digit());

        let mut rows: Vec<String> = Vec::with_capacity(height);
        if !column_header {
            rows.push(strip_row_prefix(peek.trim_end_matches(&['\n', '\r'][..])).to_string());
        }
        while rows.len() < height {
            let mut row = String::new();
            if reader.read_line(&mut row).unwrap_or(0) == 0 {
                break;
            }
            let r = row.trim_end_matches(&['\n', '\r'][..]);
            rows.push(strip_row_prefix(r).to_string());
        }
        if rows.len() == height {
            frames.push(Frame {
                width,
                height,
                rows,
            });
        }
    }

    frames
}

fn strip_row_prefix(line: &str) -> &str {
    let mut chars = line.chars();
    let a = chars.next();
    let b = chars.next();
    let c = chars.next();
    let d = chars.next();
    if let (Some(a), Some(b), Some(c), Some(d)) = (a, b, c, d) {
        if a.is_ascii_digit() && b.is_ascii_digit() && c.is_ascii_digit() && d == ' ' {
            return chars.as_str();
        }
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SAMPLE: &str = "$$$ exec p1 : [x]\n\
Anfield 3 2:\n    \
012\n\
000 .@.\n\
001 ...\n\
Piece 1 1:\n\
#\n\
-> Answer (@): 0 0\n\
\n\
Anfield 3 2:\n    \
012\n\
000 .@.\n\
001 .a.\n\
Piece 1 1:\n\
#\n\
seed: 42\n\
Player1 (x): 2\n\
Player2 (y): 0\n\
Player1 won!\n";

    #[test]
    fn parses_two_consecutive_frames_and_skips_non_anfield_lines() {
        let frames = parse_all_frames(Cursor::new(SAMPLE));
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].width, 3);
        assert_eq!(frames[0].height, 2);
        assert_eq!(frames[0].rows, vec![".@.", "..."]);
        assert_eq!(frames[1].rows[1], ".a.");
    }

    #[test]
    fn handles_log_without_column_header() {
        // The engine always uses the 3-digit row prefix `NNN `, but some captures
        // strip the column-index header line. We must still parse rows correctly.
        let s = "Anfield 2 2:\n000 .@\n001 ..\nPiece 1 1:\n#\n";
        let frames = parse_all_frames(Cursor::new(s));
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].rows, vec![".@", ".."]);
    }
}
