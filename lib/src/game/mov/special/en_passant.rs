use crate::{
    color::Color,
    game::{board::GameBoard, game::GameHistory, mov::EnPassantMov},
    mov::Mov,
    piece::Type,
    pos::Pos,
};

pub fn moves(board: &GameBoard, history: &GameHistory, pos: &Pos) -> Vec<EnPassantMov> {
    if let Some(piece) = board.get(pos) {
        return match piece.color {
            Color::White => white_pawn_en_passant(board, history, pos),
            Color::Black => black_pawn_en_passant(board, history, pos),
        };
    }
    Vec::new()
}

fn white_pawn_en_passant(
    board: &GameBoard,
    history: &GameHistory,
    pos: &Pos,
) -> Vec<EnPassantMov> {
    let mut result: Vec<EnPassantMov> = Vec::new();
    if let Some(piece) = board.get(pos) {
        if pos.row == 4 {
            if let Some(mov) = history.last() {
                if mov.piece.t == Type::Pawn
                    && mov.piece.color == Color::Black
                    && mov.from.row == 6
                    && mov.to.row == 4
                {
                    if Some(mov.to.clone()) == pos.try_of_rel_idx(0, -1) {
                        if let Some(capture_pos) = pos.try_of_rel_idx(1, -1) {
                            result.push(EnPassantMov::from(Mov {
                                piece: *piece,
                                from: pos.clone(),
                                to: Pos { col: capture_pos.col, row: pos.row + 1 },
                            }));
                        }
                    }
                    if Some(mov.to.clone()) == pos.try_of_rel_idx(0, 1) {
                        if let Some(capture_pos) = pos.try_of_rel_idx(1, 1) {
                            result.push(EnPassantMov::from(Mov {
                                piece: *piece,
                                from: pos.clone(),
                                to: Pos { col: capture_pos.col, row: pos.row + 1 },
                            }));
                        }
                    }
                }
            }
        }
    }
    result
}

fn black_pawn_en_passant(
    board: &GameBoard,
    history: &GameHistory,
    pos: &Pos,
) -> Vec<EnPassantMov> {
    let mut result: Vec<EnPassantMov> = Vec::new();
    if let Some(piece) = board.get(pos) {
        if pos.row == 3 {
            if let Some(mov) = history.last() {
                if mov.piece.t == Type::Pawn
                    && mov.piece.color == Color::White
                    && mov.from.row == 1
                    && mov.to.row == 3
                {
                    if Some(mov.to.clone()) == pos.try_of_rel_idx(0, -1) {
                        if let Some(capture_pos) = pos.try_of_rel_idx(-1, -1) {
                            result.push(EnPassantMov::from(Mov {
                                piece: *piece,
                                from: pos.clone(),
                                to: Pos { col: capture_pos.col, row: pos.row - 1 },
                            }));
                        }
                    }
                    if Some(mov.to.clone()) == pos.try_of_rel_idx(0, 1) {
                        if let Some(capture_pos) = pos.try_of_rel_idx(-1, 1) {
                            result.push(EnPassantMov::from(Mov {
                                piece: *piece,
                                from: pos.clone(),
                                to: Pos { col: capture_pos.col, row: pos.row - 1 },
                            }));
                        }
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
        game::{mov::EnPassantMov, piece::piece_of_str},
        mov::Mov,
        pos::Pos,
    };

    use super::moves;

    #[test]
    fn pawn_moved_a7_to_a5() {
        let board = HashMap::from([piece_of_str("A5", '♟'), piece_of_str("B5", '♙')]);
        let history = Vec::from([Mov::of('♟', "A7", "A5")]);
        assert_eq!(
            moves(&board, &history, &Pos::of_str("B5")),
            [EnPassantMov::from(Mov::of('♙', "B5", "A6"))]
        );
    }

    #[test]
    fn pawn_moved_b7_to_b5() {
        let board = HashMap::from([piece_of_str("B5", '♟'), piece_of_str("A5", '♙')]);
        let history = Vec::from([Mov::of('♟', "B7", "B5")]);
        assert_eq!(
            moves(&board, &history, &Pos::of_str("A5")),
            [EnPassantMov::from(Mov::of('♙', "A5", "B6"))]
        );
    }

    #[test]
    fn pawn_moved_g7_to_g5() {
        let board = HashMap::from([piece_of_str("G5", '♟'), piece_of_str("H5", '♙')]);
        let history = Vec::from([Mov::of('♟', "G7", "G5")]);
        assert_eq!(
            moves(&board, &history, &Pos::of_str("H5")),
            [EnPassantMov::from(Mov::of('♙', "H5", "G6"))]
        );
    }

    #[test]
    fn pawn_moved_h7_to_h5() {
        let board = HashMap::from([piece_of_str("H5", '♟'), piece_of_str("G5", '♙')]);
        let history = Vec::from([Mov::of('♟', "H7", "H5")]);
        assert_eq!(
            moves(&board, &history, &Pos::of_str("G5")),
            [EnPassantMov::from(Mov::of('♙', "G5", "H6"))]
        );
    }

    #[test]
    fn pawn_moved_a2_to_a4() {
        let board = HashMap::from([piece_of_str("A4", '♙'), piece_of_str("B4", '♟')]);
        let history = Vec::from([Mov::of('♙', "A2", "A4")]);
        assert_eq!(
            moves(&board, &history, &Pos::of_str("B4")),
            [EnPassantMov::from(Mov::of('♟', "B4", "A3"))]
        );
    }

    #[test]
    fn pawn_moved_b2_to_b4() {
        let board = HashMap::from([piece_of_str("B4", '♙'), piece_of_str("A4", '♟')]);
        let history = Vec::from([Mov::of('♙', "B2", "B4")]);
        assert_eq!(
            moves(&board, &history, &Pos::of_str("A4")),
            [EnPassantMov::from(Mov::of('♟', "A4", "B3"))]
        );
    }

    #[test]
    fn pawn_moved_g2_to_g4() {
        let board = HashMap::from([piece_of_str("G4", '♙'), piece_of_str("H4", '♟')]);
        let history = Vec::from([Mov::of('♙', "G2", "G4")]);
        assert_eq!(
            moves(&board, &history, &Pos::of_str("H4")),
            [EnPassantMov::from(Mov::of('♟', "H4", "G3"))]
        );
    }

    #[test]
    fn pawn_moved_h2_to_h4() {
        let board = HashMap::from([piece_of_str("H4", '♙'), piece_of_str("G4", '♟')]);
        let history = Vec::from([Mov::of('♙', "H2", "H4")]);
        assert_eq!(
            moves(&board, &history, &Pos::of_str("G4")),
            [EnPassantMov::from(Mov::of('♟', "G4", "H3"))]
        );
    }

    #[test]
    fn pawn_moved_e6_to_e5() {
        let board = HashMap::from([piece_of_str("E5", '♟'), piece_of_str("D5", '♙')]);
        let history = Vec::from([Mov::of('♟', "E6", "E5")]);
        assert_eq!(moves(&board, &history, &Pos::of_str("D5")), []);
    }

    #[test]
    fn pawn_moved_d3_to_d4() {
        let board = HashMap::from([piece_of_str("D4", '♙'), piece_of_str("E4", '♟')]);
        let history = Vec::from([Mov::of('♙', "D3", "D4")]);
        assert_eq!(moves(&board, &history, &Pos::of_str("E4")), []);
    }
}
