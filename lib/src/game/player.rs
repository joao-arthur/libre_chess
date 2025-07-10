use std::collections::HashMap;

use crate::{color::PieceColor, game::mov::GameMovOld, pos::Pos};

use super::capture::GameCapture;

#[derive(Debug, PartialEq)]
pub struct GamePlayer {
    pub color: PieceColor,
    pub captures: Vec<GameCapture>,
    pub moves: HashMap<Pos, Vec<GameMovOld>>,
}
