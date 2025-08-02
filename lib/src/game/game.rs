use std::collections::HashMap;

use manfredo::matrix::rect::rect_u8::RectU8;

use crate::{
    color::Color,
    game::{board::GameBoard, mov::GameMove, player::GamePlayer},
};

pub type GameBounds = RectU8;
pub type GamePlayers = HashMap<Color, GamePlayer>;
pub type GameHistory = Vec<GameMove>;

#[derive(Debug, PartialEq)]
pub struct Game {
    pub board: GameBoard,
    pub bounds: GameBounds,
    pub players: GamePlayers,
    pub history: GameHistory,
}

pub fn empty_players() -> GamePlayers {
    [(Color::Black, GamePlayer::from(Color::Black)), (Color::White, GamePlayer::from(Color::White))]
        .into()
}
