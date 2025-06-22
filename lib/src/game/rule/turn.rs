use crate::{color::Color, game::Game};

pub fn evaluate_turn(game: &Game) -> Color {
    if game.history.len() % 2 == 0 { Color::White } else { Color::Black }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        color::Color,
        movement::Movement,
        game::{Game, board::GameBoard},
        geometry::poligon::rect::RectU8,
    };

    use super::evaluate_turn;

    #[test]
    fn test_get_turn() {
        assert_eq!(
            evaluate_turn(&Game {
                board: GameBoard::default(),
                bounds: RectU8 { x1: 0, y1: 0, x2: 7, y2: 7 },
                players: HashMap::new(),
                history: Vec::new(),
            }),
            Color::White
        );
        assert_eq!(
            evaluate_turn(&Game {
                board: GameBoard::default(),
                bounds: RectU8 { x1: 0, y1: 0, x2: 7, y2: 7 },
                players: HashMap::new(),
                history: Vec::from([Movement::of_str("♙", "D2", "D4")]),
            }),
            Color::Black
        );
        assert_eq!(
            evaluate_turn(&Game {
                board: GameBoard::default(),
                bounds: RectU8 { x1: 0, y1: 0, x2: 7, y2: 7 },
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
