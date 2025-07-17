use std::collections::HashMap;

use crate::{
    color::Color,
    game::{board::GameBoard, game::GameHistory, mov::PieceMoveType},
    piece::PieceType,
    pos::Pos,
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
                if Some(game_move.mov.to.clone()) == pos.try_rel_idx(0, -1) {
                    if let Some(capture_pos) = pos.try_rel_idx(1, -1) {
                        result.insert(
                            Pos { col: capture_pos.col, row: pos.row + 1 },
                            PieceMoveType::EnPassant,
                        );
                    }
                }
                if Some(game_move.mov.to.clone()) == pos.try_rel_idx(0, 1) {
                    if let Some(capture_pos) = pos.try_rel_idx(1, 1) {
                        result.insert(
                            Pos { col: capture_pos.col, row: pos.row + 1 },
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
                if Some(game_move.mov.to.clone()) == pos.try_rel_idx(0, -1) {
                    if let Some(capture_pos) = pos.try_rel_idx(-1, -1) {
                        result.insert(
                            Pos { col: capture_pos.col, row: pos.row - 1 },
                            PieceMoveType::EnPassant,
                        );
                    }
                }
                if Some(game_move.mov.to.clone()) == pos.try_rel_idx(0, 1) {
                    if let Some(capture_pos) = pos.try_rel_idx(-1, 1) {
                        result.insert(
                            Pos { col: capture_pos.col, row: pos.row - 1 },
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
        pos::Pos,
    };

    use super::en_passant_moves;

    #[test]
    fn pawn_moved_a7_to_a5() {
        let board = [(Pos::of("A5"), Piece::of('♟')), (Pos::of("B5"), Piece::of('♙'))].into();
        let history = vec![GameMove::default_of('♟', "A7", "A5")];
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of("B5")),
            [(Pos::of("A6"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_b7_to_b5() {
        let board = [(Pos::of("B5"), Piece::of('♟')), (Pos::of("A5"), Piece::of('♙'))].into();
        let history = vec![GameMove::default_of('♟', "B7", "B5")];
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of("A5")),
            [(Pos::of("B6"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_g7_to_g5() {
        let board = [(Pos::of("G5"), Piece::of('♟')), (Pos::of("H5"), Piece::of('♙'))].into();
        let history = vec![GameMove::default_of('♟', "G7", "G5")];
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of("H5")),
            [(Pos::of("G6"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_h7_to_h5() {
        let board = [(Pos::of("H5"), Piece::of('♟')), (Pos::of("G5"), Piece::of('♙'))].into();
        let history = vec![GameMove::default_of('♟', "H7", "H5")];
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of("G5")),
            [(Pos::of("H6"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_a2_to_a4() {
        let board = [(Pos::of("A4"), Piece::of('♙')), (Pos::of("B4"), Piece::of('♟'))].into();
        let history = vec![GameMove::default_of('♙', "A2", "A4")];
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of("B4")),
            [(Pos::of("A3"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_b2_to_b4() {
        let board = [(Pos::of("B4"), Piece::of('♙')), (Pos::of("A4"), Piece::of('♟'))].into();
        let history = vec![GameMove::default_of('♙', "B2", "B4")];
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of("A4")),
            [(Pos::of("B3"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_g2_to_g4() {
        let board = [(Pos::of("G4"), Piece::of('♙')), (Pos::of("H4"), Piece::of('♟'))].into();
        let history = vec![GameMove::default_of('♙', "G2", "G4")];
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of("H4")),
            [(Pos::of("G3"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_h2_to_h4() {
        let board = [(Pos::of("H4"), Piece::of('♙')), (Pos::of("G4"), Piece::of('♟'))].into();
        let history = vec![GameMove::default_of('♙', "H2", "H4")];
        assert_eq!(
            en_passant_moves(&board, &history, &Pos::of("G4")),
            [(Pos::of("H3"), PieceMoveType::EnPassant)].into()
        );
    }

    #[test]
    fn pawn_moved_e6_to_e5() {
        let board = [(Pos::of("E5"), Piece::of('♟')), (Pos::of("D5"), Piece::of('♙'))].into();
        let history = vec![GameMove::default_of('♟', "E6", "E5")];
        assert_eq!(en_passant_moves(&board, &history, &Pos::of("D5")), HashMap::new());
    }

    #[test]
    fn pawn_moved_d3_to_d4() {
        let board = [(Pos::of("D4"), Piece::of('♙')), (Pos::of("E4"), Piece::of('♟'))].into();
        let history = vec![GameMove::default_of('♙', "D3", "D4")];
        assert_eq!(en_passant_moves(&board, &history, &Pos::of("E4")), HashMap::new());
    }
}
