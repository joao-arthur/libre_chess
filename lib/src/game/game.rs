use std::collections::HashMap;

use crate::{
    color::Color,
    game::board::Board,
    game::{movement::Movement, player::Player},
};

#[derive(Debug, PartialEq)]
pub struct Game {
    pub board: Board,
    pub players: HashMap<Color, Player>,
    pub history: Vec<Movement>,
}
