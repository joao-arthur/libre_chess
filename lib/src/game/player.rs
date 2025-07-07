use std::collections::HashMap;

use crate::{color::Color, game::mov::GameMov, pos::Pos};

use super::capture::GameCapture;

#[derive(Debug, PartialEq)]
pub struct GamePlayer {
    pub color: Color,
    pub captures: Vec<GameCapture>,
    pub moves: HashMap<Pos, Vec<GameMov>>,
}
