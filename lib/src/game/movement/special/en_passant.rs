use crate::{
    board::pos::Pos,
    color::Color,
    game::{board::Board, movement::Movement},
    piece::Piece,
};

fn white_pawn_en_passant(board: &Board, history: Vec<Movement>, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if pos.row == 4 {
        if let Some(mov) = history.last() {
            if mov.piece == Piece::of_str("♟") && mov.from.row == 6 && mov.to.row == 4 {
                if Some(mov.to.clone()) == pos.try_of_rel_idx(0, -1) {
                    if let Some(capture_pos) = pos.try_of_rel_idx(1, -1) {
                        result.push(capture_pos);
                    }
                }
                if Some(mov.to.clone()) == pos.try_of_rel_idx(0, 1) {
                    if let Some(capture_pos) = pos.try_of_rel_idx(1, 1) {
                        result.push(capture_pos);
                    }
                }
            }
        }
    }
    result
}

fn black_pawn_en_passant(board: &Board, history: Vec<Movement>, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if pos.row == 3 {
        if let Some(mov) = history.last() {
            if mov.piece == Piece::of_str("♙") && mov.from.row == 1 && mov.to.row == 3 {
                if Some(mov.to.clone()) == pos.try_of_rel_idx(0, -1) {
                    if let Some(capture_pos) = pos.try_of_rel_idx(-1, -1) {
                        result.push(capture_pos);
                    }
                }
                if Some(mov.to.clone()) == pos.try_of_rel_idx(0, 1) {
                    if let Some(capture_pos) = pos.try_of_rel_idx(-1, 1) {
                        result.push(capture_pos);
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::Pos,
        game::{movement::Movement, piece},
    };

    use super::{black_pawn_en_passant, white_pawn_en_passant};

    #[test]
    fn test_white_pawn_en_passant() {
        assert_eq!(
            white_pawn_en_passant(
                &HashMap::from([piece::of_str("A5", "♟"), piece::of_str("B5", "♙")]),
                Vec::from([Movement::of_str("♟", "A7", "A5")]),
                &Pos::of_str("B5"),
            ),
            [Pos::of_str("A6")]
        );
        assert_eq!(
            white_pawn_en_passant(
                &HashMap::from([piece::of_str("H5", "♟"), piece::of_str("G5", "♙")]),
                Vec::from([Movement::of_str("♟", "H7", "H5")]),
                &Pos::of_str("G5"),
            ),
            [Pos::of_str("H6")]
        );
    }

    #[test]
    fn test_black_pawn_en_passant() {
        assert_eq!(
            black_pawn_en_passant(
                &HashMap::from([piece::of_str("A4", "♙"), piece::of_str("B4", "♟")]),
                Vec::from([Movement::of_str("♙", "A2", "A4")]),
                &Pos::of_str("B4"),
            ),
            [Pos::of_str("A3")]
        );
        assert_eq!(
            black_pawn_en_passant(
                &HashMap::from([piece::of_str("H4", "♙"), piece::of_str("G4", "♟")]),
                Vec::from([Movement::of_str("♙", "H2", "H4")]),
                &Pos::of_str("G4"),
            ),
            [Pos::of_str("H3")]
        );
    }
}
