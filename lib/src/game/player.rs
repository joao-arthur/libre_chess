use std::collections::HashMap;

use crate::{color::Color, game::mov::GameMove, pos::Pos};

use super::capture::GameCapture;

#[derive(Debug, PartialEq, Clone)]
pub struct GamePlayer {
    pub color: Color,
    pub captures: Vec<GameCapture>,
    pub moves: HashMap<Pos, Vec<GameMove>>,
}

impl From<Color> for GamePlayer {
    fn from(color: Color) -> Self {
        GamePlayer { color, captures: Vec::new(), moves: HashMap::new() }
    }
}
