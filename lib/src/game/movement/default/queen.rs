use crate::{
    game::{board::GameBoard, game::GameBounds, movement::movement::GameMovement},
    pos::Pos,
};

use super::{bishop, rook};

pub fn movements(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMovement> {
    let mut result: Vec<GameMovement> = Vec::new();
    result.append(&mut bishop::movements(board, bounds, pos));
    result.append(&mut rook::movements(board, bounds, pos));
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
            movement::movement::{CaptureMovement, DefaultMovement, GameMovement, MenaceMovement},
            piece::piece_of_str,
        },
        movement::Movement,
        pos::Pos,
    };

    use super::movements;

    #[test]
    fn movements_empty_board() {
        let mode = standard_chess();
        assert_eq!(movements(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♛')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "D6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "E7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "F8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "F2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "G1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "B4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "B6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "A7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "D5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "F5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "G5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "H5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "B5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "A5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C8"))),
            ]
        );
    }

    #[test]
    fn movements_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H8", '♛')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "G7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "F6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "C3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "B2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "A1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "H7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "H6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "H5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "H4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "H3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "H2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "H1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "G8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "F8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "E8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "D8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "C8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "B8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H8", "A8"))),
            ]
        );
    }

    #[test]
    fn movements_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H1", '♛')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "G2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "F3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "D5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "C6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "B7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "A8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "G1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "F1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "E1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "D1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "C1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "B1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "A1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "H2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "H3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "H4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "H5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "H6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "H7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "H1", "H8")))
            ]
        );
    }

    #[test]
    fn movements_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A1", '♛')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "B2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "C3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "F6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "G7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "H8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "B1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "C1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "D1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "E1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "F1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "G1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "H1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "A2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "A4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "A5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "A6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "A7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A1", "A8"))),
            ]
        );
    }

    #[test]
    fn movements_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A8", '♛')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "B7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "C6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "D5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "F3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "G2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "H1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "B8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "C8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "D8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "E8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "F8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "G8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "H8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "A7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "A6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "A5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "A4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "A2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "A8", "A1"))),
            ]
        );
    }

    #[test]
    fn movements_small_bounds() {
        let board = HashMap::from([piece_of_str("F6", '♛')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("F6")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "G7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "H8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "G5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "H4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "E7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "D8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "G6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "H6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "F5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "F4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "E6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "D6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "F7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "F6", "F8")))
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
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(CaptureMovement::from(Movement::of('♕', "C5", "D6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "D4"))),
                GameMovement::from(MenaceMovement::from(Movement::of('♕', "C5", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "B4"))),
                GameMovement::from(CaptureMovement::from(Movement::of('♕', "C5", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "B6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "A7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "D5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "F5"))),
                GameMovement::from(CaptureMovement::from(Movement::of('♕', "C5", "G5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "C4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "C3"))),
                GameMovement::from(CaptureMovement::from(Movement::of('♕', "C5", "C2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "B5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "A5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♕', "C5", "C6"))),
                GameMovement::from(MenaceMovement::from(Movement::of('♕', "C5", "C7"))),
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
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(CaptureMovement::from(Movement::of('♛', "C5", "D6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "D4"))),
                GameMovement::from(MenaceMovement::from(Movement::of('♛', "C5", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "B4"))),
                GameMovement::from(CaptureMovement::from(Movement::of('♛', "C5", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "B6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "A7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "D5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "F5"))),
                GameMovement::from(CaptureMovement::from(Movement::of('♛', "C5", "G5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C3"))),
                GameMovement::from(CaptureMovement::from(Movement::of('♛', "C5", "C2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "B5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "A5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C6"))),
                GameMovement::from(MenaceMovement::from(Movement::of('♛', "C5", "C7"))),
            ]
        );
    }
}
