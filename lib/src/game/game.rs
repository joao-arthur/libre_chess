use std::collections::HashMap;

use crate::{
    color::Color,
    game::{board::GameBoard, player::GamePlayer},
    geometry::poligon::rect::RectU8,
    movement::Movement,
};

#[derive(Debug, PartialEq)]
pub struct Game {
    pub board: GameBoard,
    pub bounds: RectU8,
    pub players: HashMap<Color, GamePlayer>,
    pub history: Vec<Movement>,
}
