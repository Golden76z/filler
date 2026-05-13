use crate::game_state::{Anfield, Piece, Player, Turn};

fn strip_row_prefix(line: &str) -> Option<&str> {
    let mut chars = line.chars();
    let a = chars.next()?;
    let b = chars.next()?;
    let c = chars.next()?;
    if !a.is_ascii_digit() || !b.is_ascii_digit() || !c.is_ascii_digit() {
        return None;
    }
    if chars.next()? != ' ' {
        return None;
    }
    Some(chars.as_str())
}

fn parse_anfield_header(line: &str) -> Option<(usize, usize)> {
    let rest = line.strip_prefix("Anfield ")?;
    let (dims, _) = rest.split_once(':')?;
    let (w, h) = dims.split_once(' ')?;
    let width: usize = w.trim().parse().ok()?;
    let height: usize = h.trim().parse().ok()?;
    Some((width, height))
}

fn parse_piece_header(line: &str) -> Option<(usize, usize)> {
    let rest = line.strip_prefix("Piece ")?;
    let (dims, _) = rest.split_once(':')?;
    let (w, h) = dims.split_once(' ')?;
    let width: usize = w.trim().parse().ok()?;
    let height: usize = h.trim().parse().ok()?;
    Some((width, height))
}

fn is_column_index_line(line: &str) -> bool {
    let t = line.trim_start();
    !t.is_empty() && t.chars().all(|c| c.is_ascii_digit())
}

/// Parses one engine message: optional `$$$` line, Anfield block, Piece block.
/// `player_hint` is used when the stream does not repeat the `$$$` line on later turns.
pub fn parse_turn<I>(lines: &mut I, mut player_hint: Option<Player>) -> Option<Turn>
where
    I: Iterator<Item = String>,
{
    let mut buf: Vec<String> = Vec::new();

    while let Some(line) = lines.next() {
        if line.starts_with("$$$") {
            player_hint = Player::from_exec_line(&line).or(player_hint);
            continue;
        }
        if line.starts_with("Anfield ") {
            buf.push(line);
            break;
        }
    }

    let header = buf.pop()?;
    let (width, height) = parse_anfield_header(&header)?;

    let first = lines.next()?;
    let mut grid_lines: Vec<String> = Vec::new();

    if is_column_index_line(&first) {
        for _ in 0..height {
            grid_lines.push(lines.next()?);
        }
    } else {
        grid_lines.push(first);
        for _ in 1..height {
            grid_lines.push(lines.next()?);
        }
    }

    let mut rows: Vec<String> = Vec::with_capacity(height);
    for gl in grid_lines.into_iter().take(height) {
        let row = strip_row_prefix(&gl).unwrap_or(&gl);
        rows.push(row.to_string());
    }

    let mut piece_header: Option<String> = None;
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("Piece ") {
            piece_header = Some(line);
            break;
        }
    }
    let ph = piece_header?;

    let (pw, phh) = parse_piece_header(&ph)?;
    let mut piece_rows: Vec<String> = Vec::with_capacity(phh);
    for _ in 0..phh {
        piece_rows.push(lines.next()?);
    }

    let player = player_hint?;

    Some(Turn {
        player,
        anfield: Anfield {
            width,
            height,
            rows,
        },
        piece: Piece {
            width: pw,
            height: phh,
            rows: piece_rows,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_P1: &str = r#"$$$ exec p1 : [robots/bender]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 ....................
004 ....................
005 ....................
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 ....................
012 .........$..........
013 ....................
014 ....................
Piece 4 1:
.OO.
"#;

    fn lines_from(s: &str) -> impl Iterator<Item = String> + '_ {
        s.lines().map(|l| l.to_string())
    }

    #[test]
    fn parses_player_anfield_dimensions_and_piece() {
        let mut it = lines_from(SAMPLE_P1);
        let t = parse_turn(&mut it, None).expect("turn");
        assert_eq!(t.player, Player::P1);
        assert_eq!(t.anfield.width, 20);
        assert_eq!(t.anfield.height, 15);
        assert_eq!(t.anfield.rows[2].chars().nth(9), Some('@'));
        assert_eq!(t.piece.width, 4);
        assert_eq!(t.piece.height, 1);
        assert_eq!(t.piece.rows[0], ".OO.");
    }

    #[test]
    fn parses_without_column_header_line() {
        let s = r#"$$$ exec p2 : [x]
Anfield 3 2:
00 ...
01 .$.
Piece 2 1:
##
"#;
        let mut it = lines_from(s);
        let t = parse_turn(&mut it, None).unwrap();
        assert_eq!(t.player, Player::P2);
        assert_eq!(t.anfield.width, 3);
        assert_eq!(t.anfield.height, 2);
        assert_eq!(t.piece.rows[0], "##");
    }

    #[test]
    fn second_turn_reuses_player_hint() {
        let s = r#"Anfield 3 2:
00 ...
01 .@.
Piece 1 1:
#
"#;
        let mut it = lines_from(s);
        let t = parse_turn(&mut it, Some(Player::P1)).unwrap();
        assert_eq!(t.player, Player::P1);
    }
}
