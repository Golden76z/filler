#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    P1,
    P2,
}

impl Player {
    pub fn from_exec_line(line: &str) -> Option<Self> {
        if line.contains("exec p1") {
            Some(Player::P1)
        } else if line.contains("exec p2") {
            Some(Player::P2)
        } else {
            None
        }
    }

    /// Territory symbols on the board for this player (including last-move highlight).
    pub fn own_chars(self) -> [char; 2] {
        match self {
            Player::P1 => ['@', 'a'],
            Player::P2 => ['$', 's'],
        }
    }

    pub fn opp_chars(self) -> [char; 2] {
        match self {
            Player::P1 => ['$', 's'],
            Player::P2 => ['@', 'a'],
        }
    }

    pub fn is_own_cell(self, c: char) -> bool {
        let [a, b] = self.own_chars();
        c == a || c == b
    }

    pub fn is_opp_cell(self, c: char) -> bool {
        let [a, b] = self.opp_chars();
        c == a || c == b
    }
}

#[derive(Clone, Debug)]
pub struct Anfield {
    pub width: usize,
    pub height: usize,
    pub rows: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub rows: Vec<String>,
}

impl Piece {
    pub fn is_filled(&self, x: usize, y: usize) -> bool {
        self.rows[y].chars().nth(x).map(|c| c != '.').unwrap_or(false)
    }

    pub fn filled_cells(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_filled(x, y) {
                    out.push((x, y));
                }
            }
        }
        out
    }
}

#[derive(Clone, Debug)]
pub struct Turn {
    pub player: Player,
    pub anfield: Anfield,
    pub piece: Piece,
}
