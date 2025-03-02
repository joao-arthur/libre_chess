use std::collections::HashSet;

use bishop::naive_movements_bishop;
use king::naive_movements_king;
use knight::naive_movements_knight;
use pawn::naive_movements_pawn;
use queen::naive_movements_queen;
use rook::naive_movements_rook;

use crate::{
    board::{pos::Pos, Board},
    color::Color,
    piece::{Piece, Type},
};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

#[derive(Debug, PartialEq, Clone)]
pub struct Movement {
    pub piece: Piece,
    pub from: Pos,
    pub to: Pos,
}

impl Movement {
    pub fn try_of_str(piece: &str, from: &str, to: &str) -> Option<Self> {
        let piece = Piece::try_of_str(piece)?;
        let from = Pos::try_of_str(from)?;
        let to = Pos::try_of_str(to)?;
        Some(Movement { piece, from, to })
    }

    pub fn of_str(p: &str, from: &str, to: &str) -> Self {
        Self::try_of_str(p, from, to).unwrap()
    }
}

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
        if &entry.1.c == color {
            result.append(&mut naive_movements_piece(board, &entry.0));
        }
    }
    result.into_iter().collect()
}

#[cfg(test)]
mod test {
    use crate::{board, play::initial_board};

    use super::*;

    #[test]
    fn test_play_move_try_of_str_some() {
        assert_eq!(
            Movement::try_of_str("♟", "D2", "D4"),
            Some(Movement {
                piece: Piece::of_str("♟"),
                from: Pos::of_str("D2"),
                to: Pos::of_str("D4")
            })
        );
    }

    #[test]
    fn test_play_move_try_of_str_none() {
        assert_eq!(Movement::try_of_str("P", "D2", "D4"), None);
        assert_eq!(Movement::try_of_str("♟", "K9", "D4"), None);
        assert_eq!(Movement::try_of_str("♟", "D2", "K9"), None);
    }

    #[test]
    fn test_play_move_of_str() {
        assert_eq!(
            Movement::of_str("♟", "D2", "D4"),
            Movement {
                piece: Piece::of_str("♟"), from: Pos::of_str("D2"), to: Pos::of_str("D4")
            }
        );
    }

    #[test]
    fn test_naive_movements_piece_some() {
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
                Pos::of_str("D5"),
                Pos::of_str("D6"),
                Pos::of_str("D7"),
                Pos::of_str("D8"),
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
                Pos::of_str("C6"),
                Pos::of_str("B5"),
                Pos::of_str("B3"),
                Pos::of_str("C2"),
                Pos::of_str("E6"),
                Pos::of_str("F5"),
                Pos::of_str("F3"),
                Pos::of_str("E2"),
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
                Pos::of_str("C5"),
                Pos::of_str("C4"),
                Pos::of_str("C3"),
                Pos::of_str("D5"),
                Pos::of_str("D3"),
                Pos::of_str("E5"),
                Pos::of_str("E4"),
                Pos::of_str("E3"),
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
    fn test_naive_movements_piece_none() {
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

    #[test]
    fn test_naive_movements_board() {
        assert_eq!(
            naive_movements_board(&initial_board(), &Color::Black),
            HashSet::from([
                Pos::of_str("A6"),
                Pos::of_str("B6"),
                Pos::of_str("C6"),
                Pos::of_str("D6"),
                Pos::of_str("E6"),
                Pos::of_str("F6"),
                Pos::of_str("G6"),
                Pos::of_str("H6"),
                Pos::of_str("A5"),
                Pos::of_str("B5"),
                Pos::of_str("C5"),
                Pos::of_str("D5"),
                Pos::of_str("E5"),
                Pos::of_str("F5"),
                Pos::of_str("G5"),
                Pos::of_str("H5"),
            ])
        );
        assert_eq!(
            naive_movements_board(&initial_board(), &Color::White),
            HashSet::from([
                Pos::of_str("A4"),
                Pos::of_str("B4"),
                Pos::of_str("C4"),
                Pos::of_str("D4"),
                Pos::of_str("E4"),
                Pos::of_str("F4"),
                Pos::of_str("G4"),
                Pos::of_str("H4"),
                Pos::of_str("A3"),
                Pos::of_str("B3"),
                Pos::of_str("C3"),
                Pos::of_str("D3"),
                Pos::of_str("E3"),
                Pos::of_str("F3"),
                Pos::of_str("G3"),
                Pos::of_str("H3"),
            ])
        );
    }
}
