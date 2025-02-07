use crate::board::{pos::Pos, Board};

use super::{bishop::naive_movements_bishop, rook::naive_movements_rook};

pub fn naive_movements_queen(board: &Board, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    result.append(&mut naive_movements_bishop(board, pos));
    result.append(&mut naive_movements_rook(board, pos));
    result
}

#[cfg(test)]
mod test {
    use crate::board;

    use super::*;

    #[test]
    fn test_naive_movements_queen_empty_board() {
        assert_eq!(
            naive_movements_queen(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♛     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            [
                Pos::of_str("D6"),
                Pos::of_str("E7"),
                Pos::of_str("F8"),
                //
                Pos::of_str("D4"),
                Pos::of_str("E3"),
                Pos::of_str("F2"),
                Pos::of_str("G1"),
                //
                Pos::of_str("B4"),
                Pos::of_str("A3"),
                //
                Pos::of_str("B6"),
                Pos::of_str("A7"),
                //
                Pos::of_str("C6"),
                Pos::of_str("C7"),
                Pos::of_str("C8"),
                //
                Pos::of_str("D5"),
                Pos::of_str("E5"),
                Pos::of_str("F5"),
                Pos::of_str("G5"),
                Pos::of_str("H5"),
                //
                Pos::of_str("C4"),
                Pos::of_str("C3"),
                Pos::of_str("C2"),
                Pos::of_str("C1"),
                //
                Pos::of_str("B5"),
                Pos::of_str("A5"),
            ]
        );
    }

    #[test]
    fn test_naive_movements_queen_edge() {
        assert_eq!(
            naive_movements_queen(
                &board::of_str([
                    "♛       ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("A8"),
            ),
            [
                Pos::of_str("B7"),
                Pos::of_str("C6"),
                Pos::of_str("D5"),
                Pos::of_str("E4"),
                Pos::of_str("F3"),
                Pos::of_str("G2"),
                Pos::of_str("H1"),
                //
                Pos::of_str("B8"),
                Pos::of_str("C8"),
                Pos::of_str("D8"),
                Pos::of_str("E8"),
                Pos::of_str("F8"),
                Pos::of_str("G8"),
                Pos::of_str("H8"),
                //
                Pos::of_str("A7"),
                Pos::of_str("A6"),
                Pos::of_str("A5"),
                Pos::of_str("A4"),
                Pos::of_str("A3"),
                Pos::of_str("A2"),
                Pos::of_str("A1"),
            ]
        );
    }

    #[test]
    fn test_naive_movements_queen_with_capture() {
        assert_eq!(
            naive_movements_queen(
                &board::of_str([
                    "        ",
                    "  ♝     ",
                    "   ♖    ",
                    "  ♛   ♗ ",
                    "        ",
                    "♖   ♜   ",
                    "  ♗     ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            [
                Pos::of_str("D6"),
                //
                Pos::of_str("D4"),
                //
                Pos::of_str("B4"),
                Pos::of_str("A3"),
                //
                Pos::of_str("B6"),
                Pos::of_str("A7"),
                //
                Pos::of_str("C6"),
                //
                Pos::of_str("D5"),
                Pos::of_str("E5"),
                Pos::of_str("F5"),
                Pos::of_str("G5"),
                //
                Pos::of_str("C4"),
                Pos::of_str("C3"),
                Pos::of_str("C2"),
                //
                Pos::of_str("B5"),
                Pos::of_str("A5"),
            ]
        );
        assert_eq!(
            naive_movements_queen(
                &board::of_str([
                    "        ",
                    "  ♗     ",
                    "   ♜    ",
                    "  ♕   ♝ ",
                    "        ",
                    "♜   ♖   ",
                    "  ♝     ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            [
                Pos::of_str("D6"),
                //
                Pos::of_str("D4"),
                //
                Pos::of_str("B4"),
                Pos::of_str("A3"),
                //
                Pos::of_str("B6"),
                Pos::of_str("A7"),
                //
                Pos::of_str("C6"),
                //
                Pos::of_str("D5"),
                Pos::of_str("E5"),
                Pos::of_str("F5"),
                Pos::of_str("G5"),
                //
                Pos::of_str("C4"),
                Pos::of_str("C3"),
                Pos::of_str("C2"),
                //
                Pos::of_str("B5"),
                Pos::of_str("A5"),
            ]
        );
    }
}
