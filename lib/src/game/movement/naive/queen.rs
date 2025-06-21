use crate::{board::pos::Pos, game::board::GameBoard, geometry::poligon::rect::RectU8};

use super::{bishop, rook};

pub fn movements(board: &GameBoard, bounds: &RectU8, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    result.append(&mut bishop::movements(board, bounds, pos));
    result.append(&mut rook::movements(board, bounds, pos));
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::{Pos, pos_of_str_slice},
        game::{board, mode::standard_chess, piece},
        geometry::poligon::rect::RectU8,
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
            pos_of_str_slice([
                "D6", "E7", "F8", "D4", "E3", "F2", "G1", "B4", "A3", "B6", "A7", "D5", "E5", "F5",
                "G5", "H5", "C4", "C3", "C2", "C1", "B5", "A5", "C6", "C7", "C8",
            ])
        );
    }

    #[test]
    fn movements_edge() {
        let top_right = HashMap::from([piece::of_str("H8", "♛")]);
        let bottom_right = HashMap::from([piece::of_str("H1", "♛")]);
        let bottom_left = HashMap::from([piece::of_str("A1", "♛")]);
        let top_left = HashMap::from([piece::of_str("A8", "♛")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&top_right, &bounds, &Pos::of_str("H8")),
            pos_of_str_slice([
                "G7", "F6", "E5", "D4", "C3", "B2", "A1", "H7", "H6", "H5", "H4", "H3", "H2", "H1",
                "G8", "F8", "E8", "D8", "C8", "B8", "A8",
            ])
        );
        assert_eq!(
            movements(&bottom_right, &bounds, &Pos::of_str("H1")),
            pos_of_str_slice([
                "G2", "F3", "E4", "D5", "C6", "B7", "A8", "G1", "F1", "E1", "D1", "C1", "B1", "A1",
                "H2", "H3", "H4", "H5", "H6", "H7", "H8"
            ])
        );
        assert_eq!(
            movements(&bottom_left, &bounds, &Pos::of_str("A1")),
            pos_of_str_slice([
                "B2", "C3", "D4", "E5", "F6", "G7", "H8", "B1", "C1", "D1", "E1", "F1", "G1", "H1",
                "A2", "A3", "A4", "A5", "A6", "A7", "A8",
            ])
        );
        assert_eq!(
            movements(&top_left, &bounds, &Pos::of_str("A8")),
            pos_of_str_slice([
                "B7", "C6", "D5", "E4", "F3", "G2", "H1", "B8", "C8", "D8", "E8", "F8", "G8", "H8",
                "A7", "A6", "A5", "A4", "A3", "A2", "A1",
            ])
        );
    }

    #[test]
    fn movements_small_bounds() {
        let board = HashMap::from([piece::of_str("F6", "♛")]);
        let bounds = RectU8 { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("F6")),
            pos_of_str_slice([
                "G7", "H8", "G5", "H4", "E5", "D4", "E7", "D8", "G6", "H6", "F5", "F4", "E6", "D6",
                "F7", "F8"
            ])
        );
    }

    #[test]
    fn movements_with_capture() {
        let board_white_queen = &board::of_str([
            "        ",
            "  ♗     ",
            "   ♜    ",
            "  ♕   ♝ ",
            "        ",
            "♜   ♖   ",
            "  ♝     ",
            "        ",
        ]);
        let board_black_queen = &board::of_str([
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
            movements(&board_white_queen, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice([
                "D6", "D4", "B4", "A3", "B6", "A7", "D5", "E5", "F5", "G5", "C4", "C3", "C2", "B5",
                "A5", "C6",
            ])
        );
        assert_eq!(
            movements(&board_black_queen, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice([
                "D6", "D4", "B4", "A3", "B6", "A7", "D5", "E5", "F5", "G5", "C4", "C3", "C2", "B5",
                "A5", "C6",
            ])
        );
    }
}
