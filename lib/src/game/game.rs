use std::collections::HashMap;

use crate::{
    color::Color,
    game::{board::GameBoard, mov::GameMovOld, player::GamePlayer},
    geometry::poligon::rect::RectU8, mov::Mov,
};

pub type GameBounds = RectU8;
pub type GamePlayers = HashMap<Color, GamePlayer>;
pub type GameHistory = Vec<Mov>;
//pub type GameHistory = Vec<GameMovOld>;

#[derive(Debug, PartialEq)]
pub struct Game {
    pub board: GameBoard,
    pub bounds: GameBounds,
    pub players: GamePlayers,
    pub history: GameHistory,
}
