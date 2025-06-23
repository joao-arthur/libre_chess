use std::collections::{HashMap, HashSet};

use crate::{board::pos::Pos, color::Color, game::movement::movement::GameMovement};

use super::capture::GameCapture;

#[derive(Debug, PartialEq)]
pub struct GamePlayer {
    pub color: Color,
    pub captures: Vec<GameCapture>,
    pub menace: HashSet<Pos>,
    pub moves: HashMap<Pos, Vec<GameMovement>>,
}
