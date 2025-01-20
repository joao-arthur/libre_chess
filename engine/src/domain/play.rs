use crate::domain::{board::BoardPos, piece::Piece};

use super::board::{get_initial_white_board, Board};

#[derive(Debug, PartialEq, Clone)]
pub struct PlayMove {
    piece: Piece,
    from: BoardPos,
    to: BoardPos,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Play {
    pub b: Board,
    pub moves: Vec<PlayMove>,
}

impl Default for Play {
    fn default() -> Self {
        Play { b: get_initial_white_board(), moves: Vec::new() }
    }
}
