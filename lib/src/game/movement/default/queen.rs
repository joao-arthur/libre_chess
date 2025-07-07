use crate::{
    game::{board::GameBoard, game::GameBounds, movement::movement::GameMove},
    pos::Pos,
};

use super::{bishop, rook};

pub fn moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMove> {
    let mut result: Vec<GameMove> = Vec::new();
    result.append(&mut bishop::moves(board, bounds, pos));
    result.append(&mut rook::moves(board, bounds, pos));
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        game::{
            board::{board_empty, board_of_str},
            game::GameBounds,
            mode::standard_chess,
            movement::movement::{CaptureMove, DefaultMove, GameMove, MenaceMove},
            piece::piece_of_str,
        },
        movement::Movement,
        pos::Pos,
    };

    use super::moves;

    #[test]
    fn movements_empty_board() {
        let mode = standard_chess();
        assert_eq!(moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♛')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "D6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "E7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "F8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "D4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "E3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "F2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "G1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "B4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "A3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "B6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "A7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "D5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "E5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "F5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "G5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "H5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "C4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "C3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "C2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "C1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "B5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "A5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "C6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "C7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "C8"))),
            ]
        );
    }

    #[test]
    fn movements_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H8", '♛')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "G7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "F6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "E5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "D4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "C3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "B2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "A1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "H7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "H6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "H5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "H4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "H3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "H2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "H1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "G8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "F8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "E8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "D8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "C8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "B8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H8", "A8"))),
            ]
        );
    }

    #[test]
    fn movements_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H1", '♛')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "G2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "F3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "E4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "D5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "C6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "B7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "A8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "G1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "F1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "E1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "D1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "C1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "B1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "A1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "H2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "H3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "H4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "H5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "H6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "H7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "H1", "H8")))
            ]
        );
    }

    #[test]
    fn movements_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A1", '♛')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "B2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "C3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "D4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "E5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "F6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "G7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "H8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "B1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "C1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "D1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "E1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "F1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "G1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "H1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "A2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "A3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "A4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "A5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "A6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "A7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A1", "A8"))),
            ]
        );
    }

    #[test]
    fn movements_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A8", '♛')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "B7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "C6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "D5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "E4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "F3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "G2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "H1"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "B8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "C8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "D8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "E8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "F8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "G8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "H8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "A7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "A6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "A5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "A4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "A3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "A2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "A8", "A1"))),
            ]
        );
    }

    #[test]
    fn movements_small_bounds() {
        let board = HashMap::from([piece_of_str("F6", '♛')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            moves(&board, &bounds, &Pos::of_str("F6")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "G7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "H8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "G5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "H4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "E5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "D4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "E7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "D8"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "G6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "H6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "F5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "F4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "E6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "D6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "F7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "F6", "F8")))
            ]
        );
    }

    #[test]
    fn movements_white_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "  ♗     ",
                "   ♜    ",
                "  ♕   ♝ ",
                "        ",
                "♜   ♖   ",
                "  ♝     ",
                "        ",
            ],
        );
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::from(CaptureMove::from(Movement::of('♕', "C5", "D6"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "D4"))),
                GameMove::from(MenaceMove::from(Movement::of('♕', "C5", "E3"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "B4"))),
                GameMove::from(CaptureMove::from(Movement::of('♕', "C5", "A3"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "B6"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "A7"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "D5"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "E5"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "F5"))),
                GameMove::from(CaptureMove::from(Movement::of('♕', "C5", "G5"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "C4"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "C3"))),
                GameMove::from(CaptureMove::from(Movement::of('♕', "C5", "C2"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "B5"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "A5"))),
                GameMove::from(DefaultMove::from(Movement::of('♕', "C5", "C6"))),
                GameMove::from(MenaceMove::from(Movement::of('♕', "C5", "C7"))),
            ]
        );
    }

    #[test]
    fn movements_black_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "  ♝     ",
                "   ♖    ",
                "  ♛   ♗ ",
                "        ",
                "♖   ♜   ",
                "  ♗     ",
                "        ",
            ],
        );
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::from(CaptureMove::from(Movement::of('♛', "C5", "D6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "D4"))),
                GameMove::from(MenaceMove::from(Movement::of('♛', "C5", "E3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "B4"))),
                GameMove::from(CaptureMove::from(Movement::of('♛', "C5", "A3"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "B6"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "A7"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "D5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "E5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "F5"))),
                GameMove::from(CaptureMove::from(Movement::of('♛', "C5", "G5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "C4"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "C3"))),
                GameMove::from(CaptureMove::from(Movement::of('♛', "C5", "C2"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "B5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "A5"))),
                GameMove::from(DefaultMove::from(Movement::of('♛', "C5", "C6"))),
                GameMove::from(MenaceMove::from(Movement::of('♛', "C5", "C7"))),
            ]
        );
    }
}
