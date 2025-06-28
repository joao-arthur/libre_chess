use std::collections::{HashMap, HashSet};

use crate::{
    color::Color,
    game::{board::GameBoard, mode::GameMode, player::GamePlayer},
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

impl Game {
    fn of(mode: GameMode, history: GameHistory) -> Self {
        let mut board = mode.initial_board;
        let history_iter = history.iter();
        for movement in history_iter {
            if let Some(piece) = board.remove(&movement.from) {
                board.insert(movement.to.clone(), piece);
            }
        }
        Game {
            bounds: mode.bounds,
            board,
            players: HashMap::from([
                (
                    Color::Black,
                    GamePlayer {
                        color: Color::Black,
                        captures: Vec::new(),
                        menace: HashSet::new(),
                        moves: HashMap::new(),
                    },
                ),
                (
                    Color::White,
                    GamePlayer {
                        color: Color::White,
                        captures: Vec::new(),
                        menace: HashSet::new(),
                        moves: HashMap::new(),
                    },
                ),
            ]),
            history,
        }
    }
}

#[cfg(test)]
mod tests {
    fn test_game_of() {
        
    }
}