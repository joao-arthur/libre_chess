use std::collections::HashSet;

use crate::{board::pos::Pos, color::Color, piece::Piece};

#[derive(Debug, PartialEq)]
pub struct Player {
    pub color: Color,
    pub captured_pieces: Vec<Piece>,
    pub possible_movements: HashSet<Pos>,
}
