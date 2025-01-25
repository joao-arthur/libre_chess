use std::collections::HashMap;

use movement::Movement;

use crate::{
    board::{pos::Pos, Board},
    piece::Piece,
};

mod movement;

#[derive(Debug, PartialEq)]
pub struct Player {
    captured_pieces: Vec<Piece>,
    possible_movements: HashMap<Piece, Vec<Pos>>,
}

#[derive(Debug, PartialEq)]
pub struct Play {
    pub board: Board,
    pub movements: Vec<Movement>,
    pub white_player: Player,
    pub black_player: Player,
}

impl Default for Play {
    fn default() -> Self {
        Play {
            board: Board::default(),
            movements: Vec::new(),
            white_player: Player {
                captured_pieces: Vec::new(),
                possible_movements: HashMap::new(),
            },
            black_player: Player {
                captured_pieces: Vec::new(),
                possible_movements: HashMap::new(),
            },
        }
    }
}
