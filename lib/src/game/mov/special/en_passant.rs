use crate::{
    color::PieceColor,
    game::{
        board::GameBoard,
        game::GameHistory,
        mov::{EnPassantMove, GameMove, GameMoveType},
    },
    mov::Mov,
    piece::PieceType,
    pos::Pos,
};

pub fn en_passant_moves(board: &GameBoard, history: &GameHistory, pos: &Pos) -> Vec<GameMove> {
    if let Some(piece) = board.get(pos) {
        return match piece.color {
            PieceColor::White => white_pawn_en_passant(board, history, pos),
            PieceColor::Black => black_pawn_en_passant(board, history, pos),
        };
    }
    Vec::new()
}

fn white_pawn_en_passant(board: &GameBoard, history: &GameHistory, pos: &Pos) -> Vec<GameMove> {
    let mut result: Vec<GameMove> = Vec::new();
    if let Some(piece) = board.get(pos) {
        if pos.row == 4 {
            if let Some(mov) = history.last() {
                if mov.piece.t == PieceType::Pawn
                    && mov.piece.color == PieceColor::Black
                    && mov.from.row == 6
                    && mov.to.row == 4
                {
                    if Some(mov.to.clone()) == pos.try_of_rel_idx(0, -1) {
                        if let Some(capture_pos) = pos.try_of_rel_idx(1, -1) {
                            result.push(GameMove {
                                from: pos.clone(),
                                to: Pos { col: capture_pos.col, row: pos.row + 1 },
                                t: GameMoveType::EnPassant(EnPassantMove),
                            });
                        }
                    }
                    if Some(mov.to.clone()) == pos.try_of_rel_idx(0, 1) {
                        if let Some(capture_pos) = pos.try_of_rel_idx(1, 1) {
                            result.push(GameMove {
                                from: pos.clone(),
                                to: Pos { col: capture_pos.col, row: pos.row + 1 },
                                t: GameMoveType::EnPassant(EnPassantMove),
                            });
                        }
                    }
                }
            }
        }
    }
    result
}

fn black_pawn_en_passant(board: &GameBoard, history: &GameHistory, pos: &Pos) -> Vec<GameMove> {
    let mut result: Vec<GameMove> = Vec::new();
    if let Some(piece) = board.get(pos) {
        if pos.row == 3 {
            if let Some(mov) = history.last() {
                if mov.piece.t == PieceType::Pawn
                    && mov.piece.color == PieceColor::White
                    && mov.from.row == 1
                    && mov.to.row == 3
                {
                    if Some(mov.to.clone()) == pos.try_of_rel_idx(0, -1) {
                        if let Some(capture_pos) = pos.try_of_rel_idx(-1, -1) {
                            result.push(GameMove {
                                from: pos.clone(),
                                to: Pos { col: capture_pos.col, row: pos.row - 1 },
                                t: GameMoveType::EnPassant(EnPassantMove),
                            });
                        }
                    }
                    if Some(mov.to.clone()) == pos.try_of_rel_idx(0, 1) {
                        if let Some(capture_pos) = pos.try_of_rel_idx(-1, 1) {
                            result.push(GameMove {
                                from: pos.clone(),
                                to: Pos { col: capture_pos.col, row: pos.row - 1 },
                                t: GameMoveType::EnPassant(EnPassantMove),
                            });
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
        game::{mov::GameMove, piece::game_piece_of},
        mov::Mov,
        pos::Pos,
    };

    use super::en_passant_moves;

    #[test]
    fn pawn_moved_a7_to_a5() {
        let board = HashMap::from([game_piece_of("A5", '♟'), game_piece_of("B5", '♙')]);
        let history = Vec::from([Mov::of('♟', "A7", "A5")]);
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of_str("B5")),
            [GameMove::en_passant_of("B5", "A6")]
        );
    }

    #[test]
    fn pawn_moved_b7_to_b5() {
        let board = HashMap::from([game_piece_of("B5", '♟'), game_piece_of("A5", '♙')]);
        let history = Vec::from([Mov::of('♟', "B7", "B5")]);
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of_str("A5")),
            [GameMove::en_passant_of("A5", "B6")]
        );
    }

    #[test]
    fn pawn_moved_g7_to_g5() {
        let board = HashMap::from([game_piece_of("G5", '♟'), game_piece_of("H5", '♙')]);
        let history = Vec::from([Mov::of('♟', "G7", "G5")]);
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of_str("H5")),
            [GameMove::en_passant_of("H5", "G6")]
        );
    }

    #[test]
    fn pawn_moved_h7_to_h5() {
        let board = HashMap::from([game_piece_of("H5", '♟'), game_piece_of("G5", '♙')]);
        let history = Vec::from([Mov::of('♟', "H7", "H5")]);
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of_str("G5")),
            [GameMove::en_passant_of("G5", "H6")]
        );
    }

    #[test]
    fn pawn_moved_a2_to_a4() {
        let board = HashMap::from([game_piece_of("A4", '♙'), game_piece_of("B4", '♟')]);
        let history = Vec::from([Mov::of('♙', "A2", "A4")]);
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of_str("B4")),
            [GameMove::en_passant_of("B4", "A3")]
        );
    }

    #[test]
    fn pawn_moved_b2_to_b4() {
        let board = HashMap::from([game_piece_of("B4", '♙'), game_piece_of("A4", '♟')]);
        let history = Vec::from([Mov::of('♙', "B2", "B4")]);
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of_str("A4")),
            [GameMove::en_passant_of("A4", "B3")]
        );
    }

    #[test]
    fn pawn_moved_g2_to_g4() {
        let board = HashMap::from([game_piece_of("G4", '♙'), game_piece_of("H4", '♟')]);
        let history = Vec::from([Mov::of('♙', "G2", "G4")]);
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of_str("H4")),
            [GameMove::en_passant_of("H4", "G3")]
        );
    }

    #[test]
    fn pawn_moved_h2_to_h4() {
        let board = HashMap::from([game_piece_of("H4", '♙'), game_piece_of("G4", '♟')]);
        let history = Vec::from([Mov::of('♙', "H2", "H4")]);
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of_str("G4")),
            [GameMove::en_passant_of("G4", "H3")]
        );
    }

    #[test]
    fn pawn_moved_e6_to_e5() {
        let board = HashMap::from([game_piece_of("E5", '♟'), game_piece_of("D5", '♙')]);
        let history = Vec::from([Mov::of('♟', "E6", "E5")]);
        assert_eq!(en_passant_moves(&board, &history, &Pos::of_str("D5")), []);
    }

    #[test]
    fn pawn_moved_d3_to_d4() {
        let board = HashMap::from([game_piece_of("D4", '♙'), game_piece_of("E4", '♟')]);
        let history = Vec::from([Mov::of('♙', "D3", "D4")]);
        assert_eq!(en_passant_moves(&board, &history, &Pos::of_str("E4")), []);
    }
}
