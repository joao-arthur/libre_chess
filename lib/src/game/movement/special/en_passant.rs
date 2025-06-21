use crate::{
    board::pos::Pos,
    color::Color,
    game::{board::GameBoard, movement::Movement},
    piece::Piece,
};

pub fn  en_passant(board: &GameBoard, history: &Vec<Movement>, pos: &Pos) -> Vec<Pos> {
    if let Some(piece) = board.get(pos) {
        return match piece.color {
            Color::White => white_pawn_en_passant(board, history, pos),
            Color::Black => black_pawn_en_passant(board, history, pos),
        };
    }
    Vec::new()
}

fn white_pawn_en_passant(board: &GameBoard, history: &Vec<Movement>, pos: &Pos) -> Vec<Pos> {
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

fn black_pawn_en_passant(board: &GameBoard, history: &Vec<Movement>, pos: &Pos) -> Vec<Pos> {
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

    use super::en_passant;

    #[test]
    fn pawn_moved_a7_to_a5() {
        let board = HashMap::from([piece::of_str("A5", "♟"), piece::of_str("B5", "♙")]);
        let history = Vec::from([Movement::of_str("♟", "A7", "A5")]);
        assert_eq!(en_passant(&board, &history, &Pos::of_str("B5")), [Pos::of_str("A6")]);
    }

    #[test]
    fn pawn_moved_b7_to_b5() {
        let board = HashMap::from([piece::of_str("B5", "♟"), piece::of_str("A5", "♙")]);
        let history = Vec::from([Movement::of_str("♟", "B7", "B5")]);
        assert_eq!(en_passant(&board, &history, &Pos::of_str("A5")), [Pos::of_str("B6")]);
    }

    #[test]
    fn pawn_moved_g7_to_g5() {
        let board = HashMap::from([piece::of_str("G5", "♟"), piece::of_str("H5", "♙")]);
        let history = Vec::from([Movement::of_str("♟", "G7", "G5")]);
        assert_eq!(en_passant(&board, &history, &Pos::of_str("H5")), [Pos::of_str("G6")]);
    }

    #[test]
    fn pawn_moved_h7_to_h5() {
        let board = HashMap::from([piece::of_str("H5", "♟"), piece::of_str("G5", "♙")]);
        let history = Vec::from([Movement::of_str("♟", "H7", "H5")]);
        assert_eq!(en_passant(&board, &history, &Pos::of_str("G5")), [Pos::of_str("H6")]);
    }

    #[test]
    fn pawn_moved_a2_to_a4() {
        let board = HashMap::from([piece::of_str("A4", "♙"), piece::of_str("B4", "♟")]);
        let history = Vec::from([Movement::of_str("♙", "A2", "A4")]);
        assert_eq!(en_passant(&board, &history, &Pos::of_str("B4")), [Pos::of_str("A3")]);
    }

    #[test]
    fn pawn_moved_b2_to_b4() {
        let board = HashMap::from([piece::of_str("B4", "♙"), piece::of_str("A4", "♟")]);
        let history = Vec::from([Movement::of_str("♙", "B2", "B4")]);
        assert_eq!(en_passant(&board, &history, &Pos::of_str("A4")), [Pos::of_str("B3")]);
    }

    #[test]
    fn pawn_moved_g2_to_g4() {
        let board = HashMap::from([piece::of_str("G4", "♙"), piece::of_str("H4", "♟")]);
        let history = Vec::from([Movement::of_str("♙", "G2", "G4")]);
        assert_eq!(en_passant(&board, &history, &Pos::of_str("H4")), [Pos::of_str("G3")]);
    }

    #[test]
    fn pawn_moved_h2_to_h4() {
        let board = HashMap::from([piece::of_str("H4", "♙"), piece::of_str("G4", "♟")]);
        let history = Vec::from([Movement::of_str("♙", "H2", "H4")]);
        assert_eq!(en_passant(&board, &history, &Pos::of_str("G4")), [Pos::of_str("H3")]);
    }

    #[test]
    fn pawn_moved_e6_to_e5() {
        let board = HashMap::from([piece::of_str("E5", "♟"), piece::of_str("D5", "♙")]);
        let history = Vec::from([Movement::of_str("♟", "E6", "E5")]);
        assert_eq!(en_passant(&board, &history, &Pos::of_str("D5")), []);
    }

    #[test]
    fn pawn_moved_d3_to_d4() {
        let board = HashMap::from([piece::of_str("D4", "♙"), piece::of_str("E4", "♟")]);
        let history = Vec::from([Movement::of_str("♙", "D3", "D4")]);
        assert_eq!(en_passant(&board, &history, &Pos::of_str("E4")), []);
    }
}
