use std::collections::HashSet;

use crate::{board::pos::Pos, color::Color};

use super::capture::GameCapture;

#[derive(Debug, PartialEq)]
pub struct GamePlayer {
    pub color: Color,
    pub captures: Vec<GameCapture>,
    pub menace: HashSet<Pos>,
}
