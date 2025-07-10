use crate::{color::PieceColor, game::game::GameHistory};

pub fn evaluate_turn(history: &GameHistory) -> PieceColor {
    if history.len() % 2 == 0 { PieceColor::White } else { PieceColor::Black }
}

#[cfg(test)]
mod tests {
    use crate::{color::PieceColor, mov::Mov};

    use super::evaluate_turn;

    #[test]
    fn test_get_turn() {
        assert_eq!(evaluate_turn(&Vec::new()), PieceColor::White);
        assert_eq!(evaluate_turn(&Vec::from([Mov::of('♙', "D2", "D4")])), PieceColor::Black);
        assert_eq!(
            evaluate_turn(&Vec::from([Mov::of('♙', "D2", "D4"), Mov::of('♟', "A7", "A5")])),
            PieceColor::White
        );
    }
}
