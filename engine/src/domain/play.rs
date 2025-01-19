use crate::domain::{board::BoardPos, piece::Piece};

use super::board::{get_default_board, Board};

#[derive(Debug, PartialEq, Clone)]
pub struct PlayMove {
    piece: Piece,
    from: BoardPos,
    to: BoardPos,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Play {
    board: Board,
    moves: Vec<PlayMove>,
}

impl Default for Play {
    fn default() -> Self {
        Play { board: get_default_board(), moves: Vec::new() }
    }
}
