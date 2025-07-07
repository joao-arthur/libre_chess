use std::collections::HashMap;

use crate::{color::Color, game::movement::movement::GameMovement, pos::Pos};

use super::capture::GameCapture;

#[derive(Debug, PartialEq)]
pub struct GamePlayer {
    pub color: Color,
    pub captures: Vec<GameCapture>,
    pub moves: HashMap<Pos, Vec<GameMovement>>,
}
