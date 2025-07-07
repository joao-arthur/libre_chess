use std::collections::HashMap;

use crate::{pos::Pos, color::Color, game::movement::movement::GameMovement};

use super::capture::GameCapture;

#[derive(Debug, PartialEq)]
pub struct GamePlayer {
    pub color: Color,
    pub captures: Vec<GameCapture>,
    pub moves: HashMap<Pos, Vec<GameMovement>>,
}
