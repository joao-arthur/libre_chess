use crate::{color::Color, game::Game};

pub fn evaluate_turn(play: &Game) -> Color {
    if play.history.len() % 2 == 0 { Color::White } else { Color::Black }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        color::Color,
        game::{Game, board::Board, movement::Movement},
    };

    use super::evaluate_turn;

    #[test]
    fn test_get_turn() {
        assert_eq!(
            evaluate_turn(&Game {
                board: Board::default(),
                players: HashMap::new(),
                history: Vec::new(),
            }),
            Color::White
        );
        assert_eq!(
            evaluate_turn(&Game {
                board: Board::default(),
                players: HashMap::new(),
                history: Vec::from([Movement::of_str("♙", "D2", "D4")]),
            }),
            Color::Black
        );
        assert_eq!(
            evaluate_turn(&Game {
                board: Board::default(),
                players: HashMap::new(),
                history: Vec::from([
                    Movement::of_str("♙", "D2", "D4"),
                    Movement::of_str("♟", "A7", "A5")
                ]),
            }),
            Color::White
        );
    }
}
