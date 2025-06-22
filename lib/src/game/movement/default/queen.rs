use crate::{
    board::pos::Pos,
    game::{board::GameBoard, movement::movement::DefaultMovement},
    geometry::poligon::rect::RectU8,
};

use super::{bishop, rook};

pub fn movements(board: &GameBoard, bounds: &RectU8, pos: &Pos) -> Vec<DefaultMovement> {
    let mut result: Vec<DefaultMovement> = Vec::new();
    result.append(&mut bishop::movements(board, bounds, pos));
    result.append(&mut rook::movements(board, bounds, pos));
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::Pos,
        game::{board, mode::standard_chess, movement::movement::DefaultMovement, piece},
        geometry::poligon::rect::RectU8,
        movement::Movement,
    };

    use super::movements;

    #[test]
    fn movements_empty_board() {
        let bounds = standard_chess().bounds;
        assert_eq!(movements(&board::empty(), &bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_piece() {
        let board = HashMap::from([piece::of_str("C5", "♛")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("C5")),
            [
                DefaultMovement::from(Movement::of_str("♛", "C5", "D6")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "E7")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "F8")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "D4")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "E3")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "F2")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "G1")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "B4")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "A3")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "B6")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "A7")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "D5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "E5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "F5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "G5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "H5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C4")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C3")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C2")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C1")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "B5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "A5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C6")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C7")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C8")),
            ]
        );
    }

    #[test]
    fn movements_top_right_edge() {
        let board = HashMap::from([piece::of_str("H8", "♛")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("H8")),
            [
                DefaultMovement::from(Movement::of_str("♛", "H8", "G7")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "F6")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "E5")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "D4")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "C3")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "B2")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "A1")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "H7")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "H6")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "H5")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "H4")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "H3")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "H2")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "H1")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "G8")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "F8")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "E8")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "D8")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "C8")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "B8")),
                DefaultMovement::from(Movement::of_str("♛", "H8", "A8")),
            ]
        );
    }

    #[test]
    fn movements_bottom_right_edge() {
        let board = HashMap::from([piece::of_str("H1", "♛")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("H1")),
            [
                DefaultMovement::from(Movement::of_str("♛", "H1", "G2")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "F3")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "E4")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "D5")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "C6")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "B7")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "A8")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "G1")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "F1")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "E1")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "D1")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "C1")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "B1")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "A1")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "H2")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "H3")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "H4")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "H5")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "H6")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "H7")),
                DefaultMovement::from(Movement::of_str("♛", "H1", "H8"))
            ]
        );
    }

    #[test]
    fn movements_bottom_left_edge() {
        let board = HashMap::from([piece::of_str("A1", "♛")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("A1")),
            [
                DefaultMovement::from(Movement::of_str("♛", "A1", "B2")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "C3")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "D4")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "E5")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "F6")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "G7")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "H8")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "B1")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "C1")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "D1")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "E1")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "F1")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "G1")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "H1")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "A2")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "A3")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "A4")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "A5")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "A6")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "A7")),
                DefaultMovement::from(Movement::of_str("♛", "A1", "A8")),
            ]
        );
    }

    #[test]
    fn movements_top_left_edge() {
        let board = HashMap::from([piece::of_str("A8", "♛")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("A8")),
            [
                DefaultMovement::from(Movement::of_str("♛", "A8", "B7")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "C6")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "D5")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "E4")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "F3")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "G2")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "H1")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "B8")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "C8")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "D8")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "E8")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "F8")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "G8")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "H8")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "A7")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "A6")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "A5")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "A4")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "A3")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "A2")),
                DefaultMovement::from(Movement::of_str("♛", "A8", "A1")),
            ]
        );
    }

    #[test]
    fn movements_small_bounds() {
        let board = HashMap::from([piece::of_str("F6", "♛")]);
        let bounds = RectU8 { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("F6")),
            [
                DefaultMovement::from(Movement::of_str("♛", "F6", "G7")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "H8")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "G5")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "H4")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "E5")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "D4")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "E7")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "D8")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "G6")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "H6")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "F5")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "F4")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "E6")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "D6")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "F7")),
                DefaultMovement::from(Movement::of_str("♛", "F6", "F8"))
            ]
        );
    }

    #[test]
    fn movements_white_capture() {
        let board = board::of_str([
            "        ",
            "  ♗     ",
            "   ♜    ",
            "  ♕   ♝ ",
            "        ",
            "♜   ♖   ",
            "  ♝     ",
            "        ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("C5")),
            [
                DefaultMovement::from(Movement::of_str("♕", "C5", "D6")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "D4")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "B4")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "A3")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "B6")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "A7")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "D5")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "E5")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "F5")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "G5")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "C4")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "C3")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "C2")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "B5")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "A5")),
                DefaultMovement::from(Movement::of_str("♕", "C5", "C6")),
            ]
        );
    }

    #[test]
    fn movements_black_capture() {
        let board = board::of_str([
            "        ",
            "  ♝     ",
            "   ♖    ",
            "  ♛   ♗ ",
            "        ",
            "♖   ♜   ",
            "  ♗     ",
            "        ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("C5")),
            [
                DefaultMovement::from(Movement::of_str("♛", "C5", "D6")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "D4")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "B4")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "A3")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "B6")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "A7")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "D5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "E5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "F5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "G5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C4")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C3")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C2")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "B5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "A5")),
                DefaultMovement::from(Movement::of_str("♛", "C5", "C6")),
            ]
        );
    }
}
