use std::collections::{HashMap, HashSet};

use crate::{
    color::Color,
    game::{movement::Movement, player::Player},
    game_board::Board,
};

#[derive(Debug, PartialEq)]
pub struct Play {
    pub board: Board,
    pub players: HashMap<Color, Player>,
    pub history: Vec<Movement>,
}

impl Default for Play {
    fn default() -> Self {
        Play {
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

    use crate::{color::Color, game::player::Player, game_board::Board};

    use super::Play;

    #[test]
    fn play() {
        assert_eq!(
            Play::default(),
            Play {
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
