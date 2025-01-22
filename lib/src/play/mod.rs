use movement::Movement;

use crate::board::Board;

mod movement;

#[derive(Debug, PartialEq)]
pub struct Play {
    pub board: Board,
    pub movements: Vec<Movement>,
}

impl Default for Play {
    fn default() -> Self {
        Play { board: Board::default(), movements: Vec::new() }
    }
}
