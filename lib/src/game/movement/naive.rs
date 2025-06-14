use std::collections::HashSet;

use crate::{board::pos::Pos, color::Color, game::board::Board, piece::Type};

use super::{
    bishop::naive_movements_bishop, king::naive_movements_king, knight::naive_movements_knight,
    pawn::naive_movements_pawn, queen::naive_movements_queen, rook::naive_movements_rook,
};

pub fn naive_movements_piece(board: &Board, pos: &Pos) -> Vec<Pos> {
    if let Some(piece) = board.get(&pos) {
        return match piece.t {
            Type::Rook => naive_movements_rook(&board, pos),
            Type::Knight => naive_movements_knight(&board, pos),
            Type::Bishop => naive_movements_bishop(&board, pos),
            Type::Queen => naive_movements_queen(&board, pos),
            Type::King => naive_movements_king(&board, pos),
            Type::Pawn => naive_movements_pawn(&board, pos),
        };
    }
    return Vec::new();
}

pub fn naive_movements_board(board: &Board, color: &Color) -> HashSet<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    for entry in board.iter() {
        if &entry.1.color == color {
            result.append(&mut naive_movements_piece(board, &entry.0));
        }
    }
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {

    use crate::{board::pos::Pos, game::board};

    use super::{naive_movements_board, naive_movements_piece};

    #[test]
    fn naive_movements_piece_some() {
        assert_eq!(
            naive_movements_piece(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "   ♜    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                //
                Pos::of_str("E4"),
                Pos::of_str("F4"),
                Pos::of_str("G4"),
                Pos::of_str("H4"),
                //
                Pos::of_str("D3"),
                Pos::of_str("D2"),
                Pos::of_str("D1"),
                //
                Pos::of_str("C4"),
                Pos::of_str("B4"),
                Pos::of_str("A4"),
                //
                Pos::of_str("D5"),
                Pos::of_str("D6"),
                Pos::of_str("D7"),
                Pos::of_str("D8"),
            ]
        );
        assert_eq!(
            naive_movements_piece(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "   ♞    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                Pos::of_str("E6"),
                Pos::of_str("F5"),
                Pos::of_str("F3"),
                Pos::of_str("E2"),
                Pos::of_str("C2"),
                Pos::of_str("B3"),
                Pos::of_str("B5"),
                Pos::of_str("C6"),
            ]
        );
        assert_eq!(
            naive_movements_piece(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♝     ",
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
            ]
        );
        assert_eq!(
            naive_movements_piece(
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
                //
                Pos::of_str("C6"),
                Pos::of_str("C7"),
                Pos::of_str("C8"),
            ]
        );
        assert_eq!(
            naive_movements_piece(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "   ♚    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                Pos::of_str("E5"),
                Pos::of_str("E4"),
                Pos::of_str("E3"),
                Pos::of_str("D3"),
                Pos::of_str("C3"),
                Pos::of_str("C4"),
                Pos::of_str("C5"),
                Pos::of_str("D5"),
            ]
        );
        assert_eq!(
            naive_movements_piece(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            [Pos::of_str("C6")]
        );
    }

    #[test]
    fn naive_movements_piece_none() {
        assert_eq!(
            naive_movements_piece(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("A1"),
            ),
            []
        );
    }
}
