use std::collections::{HashMap, HashSet};

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

impl Default for Game {
    fn default() -> Self {
        Game {
            board: Board::default(),
            players: HashMap::from([
                (
                    Color::White,
                    Player {
                        color: Color::White,
                        captured_pieces: Vec::new(),
                        possible_movements: HashSet::new(),
                    },
                ),
                (
                    Color::Black,
                    Player {
                        color: Color::Black,
                        captured_pieces: Vec::new(),
                        possible_movements: HashSet::new(),
                    },
                ),
            ]),
            history: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{color::Color, game::board::Board, game::player::Player};

    use super::Game;

    #[test]
    fn play() {
        assert_eq!(
            Game::default(),
            Game {
                board: Board::default(),
                players: HashMap::from([
                    (
                        Color::White,
                        Player {
                            color: Color::White,
                            captured_pieces: Vec::new(),
                            possible_movements: HashSet::new(),
                        },
                    ),
                    (
                        Color::Black,
                        Player {
                            color: Color::Black,
                            captured_pieces: Vec::new(),
                            possible_movements: HashSet::new(),
                        },
                    ),
                ]),
                history: Vec::new(),
            }
        );
    }
}
