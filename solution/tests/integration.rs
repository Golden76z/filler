use filler::game_state::Player;
use filler::output::format_move;
use filler::parser::parse_turn;
use filler::placement::{is_valid_placement, Placement};
use filler::strategy::choose_placement;

const SAMPLE_P1: &str = "$$$ exec p1 : [robots/bender]\n\
Anfield 20 15:\n    \
01234567890123456789\n\
000 ....................\n\
001 ....................\n\
002 .........@..........\n\
003 ....................\n\
004 ....................\n\
005 ....................\n\
006 ....................\n\
007 ....................\n\
008 ....................\n\
009 ....................\n\
010 ....................\n\
011 ....................\n\
012 .........$..........\n\
013 ....................\n\
014 ....................\n\
Piece 4 1:\n\
.OO.\n";

#[test]
fn full_round_trip_first_turn() {
    let mut it = SAMPLE_P1.lines().map(|s| s.to_string());
    let turn = parse_turn(&mut it, None).expect("turn parses");

    assert_eq!(turn.player, Player::P1);
    assert_eq!(turn.anfield.width, 20);
    assert_eq!(turn.anfield.height, 15);
    assert_eq!(turn.piece.width, 4);
    assert_eq!(turn.piece.height, 1);

    let placement = choose_placement(turn.player, &turn.anfield, &turn.piece).expect("placement");
    assert!(is_valid_placement(
        turn.player,
        &turn.anfield,
        &turn.piece,
        placement.x,
        placement.y,
    ));

    let line = format_move(placement);
    assert!(line.ends_with('\n'));
    let trimmed = line.trim_end();
    let parts: Vec<&str> = trimmed.split(' ').collect();
    assert_eq!(parts.len(), 2);
    assert!(parts[0].parse::<usize>().is_ok());
    assert!(parts[1].parse::<usize>().is_ok());
}

#[test]
fn placement_never_extends_beyond_grid() {
    let mut it = SAMPLE_P1.lines().map(|s| s.to_string());
    let turn = parse_turn(&mut it, None).unwrap();

    let p = choose_placement(turn.player, &turn.anfield, &turn.piece).unwrap();

    for (px, py) in turn.piece.filled_cells() {
        assert!(p.x + px < turn.anfield.width);
        assert!(p.y + py < turn.anfield.height);
    }
}

#[test]
fn rejects_placements_outside_bounds_explicitly() {
    let mut it = SAMPLE_P1.lines().map(|s| s.to_string());
    let turn = parse_turn(&mut it, None).unwrap();

    assert!(!is_valid_placement(
        turn.player,
        &turn.anfield,
        &turn.piece,
        turn.anfield.width,
        0
    ));
    assert!(!is_valid_placement(
        turn.player,
        &turn.anfield,
        &turn.piece,
        0,
        turn.anfield.height
    ));
}

#[test]
fn prefers_moves_pointing_toward_opponent() {
    // @ has 2 candidate anchors. The placement aimed at $ side should be preferred.
    let board = "$$$ exec p1 : [x]\n\
Anfield 7 1:\n    \
0123456\n\
000 ..@...$\n\
Piece 2 1:\n\
##\n";
    let mut it = board.lines().map(|s| s.to_string());
    let turn = parse_turn(&mut it, None).unwrap();
    let p: Placement = choose_placement(turn.player, &turn.anfield, &turn.piece).unwrap();
    // Two valid placements: x=1 (## over .@) and x=2 (## over @.).
    // x=2 puts a new cell at (3,0) which is closer to $ at (6,0).
    assert_eq!(p, Placement { x: 2, y: 0 });
}
