use std::collections::HashMap;

use crate::{
    color::Color,
    game::{board::GameBoard, game::GameHistory, mov::PieceMoveType},
    piece::PieceType,
    pos::{pos_try_rel_idx, Pos},
};

pub fn en_passant_moves(
    board: &GameBoard,
    history: &GameHistory,
    pos: &Pos,
) -> HashMap<Pos, PieceMoveType> {
    if let Some(piece) = board.get(pos) {
        match piece.color {
            Color::White => white_pawn_en_passant(history, pos),
            Color::Black => black_pawn_en_passant(history, pos),
        }
    } else {
        HashMap::new()
    }
}

fn white_pawn_en_passant(history: &GameHistory, pos: &Pos) -> HashMap<Pos, PieceMoveType> {
    let mut result = HashMap::new();
    if pos.row == 4 {
        if let Some(game_move) = history.last() {
            if game_move.mov.piece.typ == PieceType::Pawn
                && game_move.mov.piece.color == Color::Black
                && game_move.mov.from.row == 6
                && game_move.mov.to.row == 4
            {
                if Some(game_move.mov.to.clone()) == pos_try_rel_idx(pos, 0, -1) {
                    if let Some(capture_pos) = pos_try_rel_idx(pos, 1, -1) {
                        result.insert(
                            Pos::of(pos.row + 1, capture_pos.col),
                            PieceMoveType::EnPassant,
                        );
                    }
                }
                if Some(game_move.mov.to.clone()) == pos_try_rel_idx(pos, 0, 1) {
                    if let Some(capture_pos) = pos_try_rel_idx(pos, 1, 1) {
                        result.insert(
                            Pos::of(pos.row + 1, capture_pos.col),
                            PieceMoveType::EnPassant,
                        );
                    }
                }
            }
        }
    }
    result
}

fn black_pawn_en_passant(history: &GameHistory, pos: &Pos) -> HashMap<Pos, PieceMoveType> {
    let mut result = HashMap::new();
    if pos.row == 3 {
        if let Some(game_move) = history.last() {
            if game_move.mov.piece.typ == PieceType::Pawn
                && game_move.mov.piece.color == Color::White
                && game_move.mov.from.row == 1
                && game_move.mov.to.row == 3
            {
                if Some(game_move.mov.to.clone()) == pos_try_rel_idx(pos, 0, -1) {
                    if let Some(capture_pos) = pos_try_rel_idx(pos, -1, -1) {
                        result.insert(
                            Pos::of(pos.row - 1, capture_pos.col),
                            PieceMoveType::EnPassant,
                        );
                    }
                }
                if Some(game_move.mov.to.clone()) == pos_try_rel_idx(pos, 0, 1) {
                    if let Some(capture_pos) = pos_try_rel_idx(pos, -1, 1) {
                        result.insert(
                            Pos::of(pos.row - 1, capture_pos.col),
                            PieceMoveType::EnPassant,
                        );
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
        game::mov::{GameMove, PieceMoveType},
        piece::Piece,
        pos::pos_of,
    };

    use super::en_passant_moves;

    #[test]
    fn pawn_moved_a7_to_a5() {
        let board = [(pos_of("A5"), Piece::of('♟')), (pos_of("B5"), Piece::of('♙'))].into();
        let history = vec![GameMove::default_of('♟', "A7", "A5")];
        assert_eq!(
            en_passant_moves(&board, &history, &pos_of("B5")),
            [(pos_of("A6"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_b7_to_b5() {
        let board = [(pos_of("B5"), Piece::of('♟')), (pos_of("A5"), Piece::of('♙'))].into();
        let history = vec![GameMove::default_of('♟', "B7", "B5")];
        assert_eq!(
            en_passant_moves(&board, &history, &pos_of("A5")),
            [(pos_of("B6"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_g7_to_g5() {
        let board = [(pos_of("G5"), Piece::of('♟')), (pos_of("H5"), Piece::of('♙'))].into();
        let history = vec![GameMove::default_of('♟', "G7", "G5")];
        assert_eq!(
            en_passant_moves(&board, &history, &pos_of("H5")),
            [(pos_of("G6"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_h7_to_h5() {
        let board = [(pos_of("H5"), Piece::of('♟')), (pos_of("G5"), Piece::of('♙'))].into();
        let history = vec![GameMove::default_of('♟', "H7", "H5")];
        assert_eq!(
            en_passant_moves(&board, &history, &pos_of("G5")),
            [(pos_of("H6"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_a2_to_a4() {
        let board = [(pos_of("A4"), Piece::of('♙')), (pos_of("B4"), Piece::of('♟'))].into();
        let history = vec![GameMove::default_of('♙', "A2", "A4")];
        assert_eq!(
            en_passant_moves(&board, &history, &pos_of("B4")),
            [(pos_of("A3"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_b2_to_b4() {
        let board = [(pos_of("B4"), Piece::of('♙')), (pos_of("A4"), Piece::of('♟'))].into();
        let history = vec![GameMove::default_of('♙', "B2", "B4")];
        assert_eq!(
            en_passant_moves(&board, &history, &pos_of("A4")),
            [(pos_of("B3"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_g2_to_g4() {
        let board = [(pos_of("G4"), Piece::of('♙')), (pos_of("H4"), Piece::of('♟'))].into();
        let history = vec![GameMove::default_of('♙', "G2", "G4")];
        assert_eq!(
            en_passant_moves(&board, &history, &pos_of("H4")),
            [(pos_of("G3"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_h2_to_h4() {
        let board = [(pos_of("H4"), Piece::of('♙')), (pos_of("G4"), Piece::of('♟'))].into();
        let history = vec![GameMove::default_of('♙', "H2", "H4")];
        assert_eq!(
            en_passant_moves(&board, &history, &pos_of("G4")),
            [(pos_of("H3"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_e6_to_e5() {
        let board = [(pos_of("E5"), Piece::of('♟')), (pos_of("D5"), Piece::of('♙'))].into();
        let history = vec![GameMove::default_of('♟', "E6", "E5")];
        assert_eq!(en_passant_moves(&board, &history, &pos_of("D5")), HashMap::new());
    }

    #[test]
    fn pawn_moved_d3_to_d4() {
        let board = [(pos_of("D4"), Piece::of('♙')), (pos_of("E4"), Piece::of('♟'))].into();
        let history = vec![GameMove::default_of('♙', "D3", "D4")];
        assert_eq!(en_passant_moves(&board, &history, &pos_of("E4")), HashMap::new());
    }
}
