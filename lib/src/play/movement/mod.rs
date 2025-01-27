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

pub fn get_naive_movements_piece(board: &Board, pos: &Pos, piece: &Piece) -> Vec<Pos> {
    match piece.t {
        Type::Rook => naive_movements_rook(&board, pos, &piece.c),
        Type::Knight => naive_movements_knight(&board, pos, &piece.c),
        Type::Bishop => naive_movements_bishop(&board, pos, &piece.c),
        Type::Queen => naive_movements_queen(&board, pos, &piece.c),
        Type::King => naive_movements_king(&board, pos, &piece.c),
        Type::Pawn => naive_movements_pawn(&board, pos, &piece.c),
    }
}

pub fn get_naive_movements(board: &Board, color: &Color) -> HashSet<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    for row in 0..8 {
        for col in 0..8 {
            if let Some(pos) = Pos::try_of_idx(row, col) {
                if let Some(piece) = board[pos.clone()] {
                    if &piece.c == color {
                        result.append(&mut get_naive_movements_piece(board, &pos, &piece));
                    }
                }
            }
        }
    }
    result.into_iter().collect()
}

#[cfg(test)]
mod test {
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
}
