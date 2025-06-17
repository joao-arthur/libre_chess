use std::collections::HashMap;

use crate::{
    color::Color,
    game::{board::Board, movement::Movement, player::Player},
    geometry::poligon::rect::RectU8,
};

#[derive(Debug, PartialEq)]
pub struct Game {
    pub board: Board,
    pub bounds: RectU8,
    pub players: HashMap<Color, Player>,
    pub history: Vec<Movement>,
}
