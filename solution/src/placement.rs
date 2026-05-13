use crate::game_state::{Anfield, Piece, Player};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub x: usize,
    pub y: usize,
}

/// Top-left anchor of the piece bounding box at (x, y). Only non-`.` cells participate.
pub fn is_valid_placement(
    player: Player,
    field: &Anfield,
    piece: &Piece,
    x: usize,
    y: usize,
) -> bool {
    let mut own = 0usize;
    let mut opp = 0usize;

    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let bx = x + px;
            let by = y + py;
            if bx >= field.width || by >= field.height {
                return false;
            }
            let c = field.rows[by].chars().nth(bx).unwrap_or('.');
            if player.is_own_cell(c) {
                own += 1;
            } else if player.is_opp_cell(c) {
                opp += 1;
            }
        }
    }

    opp == 0 && own == 1
}

pub fn all_valid_placements(player: Player, field: &Anfield, piece: &Piece) -> Vec<Placement> {
    let mut out = Vec::new();
    if field.height == 0 || field.width == 0 {
        return out;
    }
    for yy in 0..field.height {
        for xx in 0..field.width {
            if is_valid_placement(player, field, piece, xx, yy) {
                out.push(Placement { x: xx, y: yy });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::Anfield;

    fn field_from(rows: &[&str]) -> Anfield {
        let height = rows.len();
        let width = rows[0].len();
        Anfield {
            width,
            height,
            rows: rows.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn rejects_out_of_bounds() {
        let f = field_from(&["..", ".."]);
        let p = Piece {
            width: 2,
            height: 1,
            rows: vec!["##".into()],
        };
        assert!(!is_valid_placement(Player::P1, &f, &p, 1, 0));
        assert!(!is_valid_placement(Player::P1, &f, &p, 0, 1));
    }

    #[test]
    fn requires_exactly_one_overlap_with_own() {
        let f = field_from(&[".@", ".."]);
        let p = Piece {
            width: 2,
            height: 1,
            rows: vec!["##".into()],
        };
        assert!(is_valid_placement(Player::P1, &f, &p, 0, 0));
        assert!(!is_valid_placement(Player::P1, &f, &p, 0, 1));
    }

    #[test]
    fn rejects_opponent_overlap() {
        let f = field_from(&[".$", ".."]);
        let p = Piece {
            width: 1,
            height: 1,
            rows: vec!["#".into()],
        };
        assert!(!is_valid_placement(Player::P1, &f, &p, 1, 0));
    }

    #[test]
    fn rejects_two_own_overlaps() {
        let f = field_from(&["@@", ".."]);
        let p = Piece {
            width: 2,
            height: 1,
            rows: vec!["##".into()],
        };
        assert!(!is_valid_placement(Player::P1, &f, &p, 0, 0));
    }

    #[test]
    fn player2_symbols() {
        let f = field_from(&[".s", ".."]);
        let p = Piece {
            width: 1,
            height: 1,
            rows: vec!["#".into()],
        };
        assert!(is_valid_placement(Player::P2, &f, &p, 1, 0));
    }

    #[test]
    fn ignores_piece_empty_cells_for_overlap_rules() {
        // Opponent at (0,0), own at (1,0). The piece's `.` at (0,0) sits over `$`
        // but must be ignored because only filled piece cells count.
        let f = field_from(&["$@.", "..."]);
        let p = Piece {
            width: 3,
            height: 1,
            rows: vec![".##".into()],
        };
        assert!(is_valid_placement(Player::P1, &f, &p, 0, 0));
    }
}
