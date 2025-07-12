use crate::{color::Color, game::game::GameHistory};

pub fn evaluate_turn(history: &GameHistory) -> Color {
    if history.len() % 2 == 0 { Color::White } else { Color::Black }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, game::mov::GameMove};

    use super::evaluate_turn;

    #[test]
    fn test_get_turn() {
        assert_eq!(evaluate_turn(&Vec::new()), Color::White);
        assert_eq!(
            evaluate_turn(&Vec::from([GameMove::default_of('♙', "D2", "D4")])),
            Color::Black
        );
        assert_eq!(
            evaluate_turn(&Vec::from([
                GameMove::default_of('♙', "D2", "D4"),
                GameMove::default_of('♟', "A7", "A5")
            ])),
            Color::White
        );
    }
}
