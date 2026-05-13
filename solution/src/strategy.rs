use crate::game_state::{Anfield, Piece, Player};
use crate::placement::{all_valid_placements, Placement};

/// Multi-source BFS distance map (Manhattan-ish, 4-connected) from opponent cells.
/// Cells that are part of own territory are walkable for the BFS (distance grows through them).
/// We use 4-connectivity which gives a faithful spread; Chebyshev/8-connectivity also works.
fn opponent_distance_map(player: Player, field: &Anfield) -> Vec<Vec<u32>> {
    let h = field.height;
    let w = field.width;
    let mut dist = vec![vec![u32::MAX; w]; h];
    let mut queue: std::collections::VecDeque<(usize, usize)> = std::collections::VecDeque::new();

    for y in 0..h {
        for (x, c) in field.rows[y].chars().enumerate().take(w) {
            if player.is_opp_cell(c) {
                dist[y][x] = 0;
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let d = dist[y][x];
        // 8-connected (Chebyshev) BFS: pieces can connect diagonally on the
        // board because the "one cell overlap" rule allows any single shared
        // cell — distance via Chebyshev gives a much better aggression signal.
        let neigh: [(isize, isize); 8] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        for (dx, dy) in neigh {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if dist[ny][nx] != u32::MAX {
                continue;
            }
            dist[ny][nx] = d + 1;
            queue.push_back((nx, ny));
        }
    }

    dist
}

/// Score a placement: lower is better.
/// Primary: minimum distance from any newly-added piece cell to the opponent.
/// Secondary: sum of distances (encourages broad reach toward opponent).
/// Tertiary: small penalty for hugging the board edges, which traps us.
fn score_placement(
    placement: Placement,
    piece: &Piece,
    player: Player,
    field: &Anfield,
    dist: &[Vec<u32>],
) -> (u32, u64, i32) {
    let mut min_d = u32::MAX;
    let mut sum_d: u64 = 0;
    let mut edge_penalty = 0i32;

    for (px, py) in piece.filled_cells() {
        let bx = placement.x + px;
        let by = placement.y + py;
        let c = field.rows[by].chars().nth(bx).unwrap_or('.');
        if player.is_own_cell(c) {
            continue;
        }
        let d = dist[by][bx];
        if d < min_d {
            min_d = d;
        }
        sum_d = sum_d.saturating_add(d as u64);
        if bx == 0 || by == 0 || bx + 1 == field.width || by + 1 == field.height {
            edge_penalty += 1;
        }
    }

    (min_d, sum_d, edge_penalty)
}

pub fn choose_placement(player: Player, field: &Anfield, piece: &Piece) -> Option<Placement> {
    let valid = all_valid_placements(player, field, piece);
    if valid.is_empty() {
        return None;
    }

    let any_opp = field
        .rows
        .iter()
        .any(|r| r.chars().any(|c| player.is_opp_cell(c)));

    if !any_opp {
        return Some(valid[0]);
    }

    let dist = opponent_distance_map(player, field);

    let mut best: Option<(Placement, (u32, u64, i32))> = None;
    for p in valid {
        let s = score_placement(p, piece, player, field, &dist);
        match &best {
            None => best = Some((p, s)),
            Some((_, bs)) if s < *bs => best = Some((p, s)),
            _ => {}
        }
    }

    best.map(|(p, _)| p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::Anfield;

    fn field_from(rows: &[&str]) -> Anfield {
        Anfield {
            width: rows[0].len(),
            height: rows.len(),
            rows: rows.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn picks_move_toward_opponent() {
        // P1 at (0,0), P2 at (9,0). With a 1x1 piece, the only valid placement is
        // on top of @ but the heuristic should still return a valid placement.
        let f = field_from(&[
            "@.........",
            "..........",
            "..........",
            "..........",
            ".........$",
        ]);
        let p = Piece {
            width: 1,
            height: 1,
            rows: vec!["#".into()],
        };
        let chosen = choose_placement(Player::P1, &f, &p).expect("some");
        assert_eq!((chosen.x, chosen.y), (0, 0));
    }

    #[test]
    fn returns_none_when_no_valid_placement() {
        // Field full of opponent cells and no own cells: no placement can satisfy
        // the exactly-one-own-overlap rule.
        let f = field_from(&["$$", "$$"]);
        let p = Piece {
            width: 1,
            height: 1,
            rows: vec!["#".into()],
        };
        assert!(choose_placement(Player::P1, &f, &p).is_none());
    }

    #[test]
    fn handles_no_opponent_yet() {
        let f = field_from(&["@...", "...."]);
        let p = Piece {
            width: 1,
            height: 1,
            rows: vec!["#".into()],
        };
        let chosen = choose_placement(Player::P1, &f, &p).expect("some");
        assert_eq!((chosen.x, chosen.y), (0, 0));
    }
}
