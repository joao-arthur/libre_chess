use std::collections::HashMap;

use crate::{
    color::Color,
    game::{board::GameBoard, player::GamePlayer},
    geometry::poligon::rect::RectU8,
    movement::Movement,
};

pub type GameBounds = RectU8;
pub type GamePlayers = HashMap<Color, GamePlayer>;
pub type GameHistory = Vec<Movement>;

#[derive(Debug, PartialEq)]
pub struct Game {
    pub board: GameBoard,
    pub bounds: GameBounds,
    pub players: GamePlayers,
    pub history: GameHistory,
}
