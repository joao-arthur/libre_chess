use crate::{
    board::pos::Pos,
    game::{board::GameBoard, game::GameBounds},
};

use super::{bishop, rook};

pub fn menace(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    result.append(&mut bishop::menace(board, bounds, pos));
    result.append(&mut rook::menace(board, bounds, pos));
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::{Pos, pos_of_str_slice},
        game::{board::board_of_str, game::GameBounds, mode::standard_chess, piece::piece_of_str},
    };

    use super::menace;

    #[test]
    fn menace_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♛')]);
        assert_eq!(
            menace(&board, &mode.bounds, &Pos::of_str("C5")),
            pos_of_str_slice([
                "D6", "E7", "F8", "D4", "E3", "F2", "G1", "B4", "A3", "B6", "A7", "D5", "E5", "F5",
                "G5", "H5", "C4", "C3", "C2", "C1", "B5", "A5", "C6", "C7", "C8",
            ])
        );
    }

    #[test]
    fn menace_edge() {
        let mode = standard_chess();
        let top_right = HashMap::from([piece_of_str("H8", '♛')]);
        let bottom_right = HashMap::from([piece_of_str("H1", '♛')]);
        let bottom_left = HashMap::from([piece_of_str("A1", '♛')]);
        let top_left = HashMap::from([piece_of_str("A8", '♛')]);
        assert_eq!(
            menace(&top_right, &mode.bounds, &Pos::of_str("H8")),
            pos_of_str_slice([
                "G7", "F6", "E5", "D4", "C3", "B2", "A1", "H7", "H6", "H5", "H4", "H3", "H2", "H1",
                "G8", "F8", "E8", "D8", "C8", "B8", "A8",
            ])
        );
        assert_eq!(
            menace(&bottom_right, &mode.bounds, &Pos::of_str("H1")),
            pos_of_str_slice([
                "G2", "F3", "E4", "D5", "C6", "B7", "A8", "G1", "F1", "E1", "D1", "C1", "B1", "A1",
                "H2", "H3", "H4", "H5", "H6", "H7", "H8"
            ])
        );
        assert_eq!(
            menace(&bottom_left, &mode.bounds, &Pos::of_str("A1")),
            pos_of_str_slice([
                "B2", "C3", "D4", "E5", "F6", "G7", "H8", "B1", "C1", "D1", "E1", "F1", "G1", "H1",
                "A2", "A3", "A4", "A5", "A6", "A7", "A8",
            ])
        );
        assert_eq!(
            menace(&top_left, &mode.bounds, &Pos::of_str("A8")),
            pos_of_str_slice([
                "B7", "C6", "D5", "E4", "F3", "G2", "H1", "B8", "C8", "D8", "E8", "F8", "G8", "H8",
                "A7", "A6", "A5", "A4", "A3", "A2", "A1",
            ])
        );
    }

    #[test]
    fn menace_small_bounds() {
        let board = HashMap::from([piece_of_str("F6", '♛')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            menace(&board, &bounds, &Pos::of_str("F6")),
            pos_of_str_slice([
                "G7", "H8", "G5", "H4", "E5", "D4", "E7", "D8", "G6", "H6", "F5", "F4", "E6", "D6",
                "F7", "F8"
            ])
        );
    }

    #[test]
    fn menace_with_capture() {
        let mode = standard_chess();
        let board_white_queen = board_of_str(
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
        let board_black_queen = board_of_str(
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
            menace(&board_white_queen, &mode.bounds, &Pos::of_str("C5")),
            pos_of_str_slice([
                "D6", "D4", "E3", "B4", "A3", "B6", "A7", "D5", "E5", "F5", "G5", "C4", "C3", "C2",
                "B5", "A5", "C6", "C7"
            ])
        );
        assert_eq!(
            menace(&board_black_queen, &mode.bounds, &Pos::of_str("C5")),
            pos_of_str_slice([
                "D6", "D4", "E3", "B4", "A3", "B6", "A7", "D5", "E5", "F5", "G5", "C4", "C3", "C2",
                "B5", "A5", "C6", "C7"
            ])
        );
    }
}
