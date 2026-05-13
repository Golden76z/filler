use crate::placement::Placement;

/// Engine format: `X Y\n` with no extra spaces.
pub fn format_move(p: Placement) -> String {
    format!("{} {}\n", p.x, p.y)
}

pub fn format_pass() -> String {
    "0 0\n".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_format_with_trailing_newline() {
        assert_eq!(format_move(Placement { x: 7, y: 2 }), "7 2\n");
    }

    #[test]
    fn pass_format() {
        assert_eq!(format_pass(), "0 0\n");
    }
}
