use crate::{
    game::{board::GameBoard, game::GameBounds, mov::GameMove},
    piece::PieceType,
    pos::Pos,
};

use self::{
    bishop::bishop_moves, king::king_moves, knight::knight_moves, pawn::pawn_moves,
    queen::queen_moves, rook::rook_moves,
};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub fn default_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMove> {
    if let Some(piece) = board.get(pos) {
        return match piece.t {
            PieceType::Rook => rook_moves(board, bounds, pos),
            PieceType::Knight => knight_moves(board, bounds, pos),
            PieceType::Bishop => bishop_moves(board, bounds, pos),
            PieceType::Queen => queen_moves(board, bounds, pos),
            PieceType::King => king_moves(board, bounds, pos),
            PieceType::Pawn => pawn_moves(board, bounds, pos),
        };
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        game::{
            board::board_empty,
            mode::standard_chess,
            mov::{DefaultMovOld, GameMove, MenaceMovOld},
            piece::game_piece_of,
        },
        mov::Mov,
        pos::Pos,
    };

    use super::default_moves;

    #[test]
    fn default_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(default_moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn default_moves_rook() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♜')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMove::default_of("D4", "E4"),
                GameMove::default_of("D4", "F4"),
                GameMove::default_of("D4", "G4"),
                GameMove::default_of("D4", "H4"),
                GameMove::default_of("D4", "D3"),
                GameMove::default_of("D4", "D2"),
                GameMove::default_of("D4", "D1"),
                GameMove::default_of("D4", "C4"),
                GameMove::default_of("D4", "B4"),
                GameMove::default_of("D4", "A4"),
                GameMove::default_of("D4", "D5"),
                GameMove::default_of("D4", "D6"),
                GameMove::default_of("D4", "D7"),
                GameMove::default_of("D4", "D8"),
            ]
        );
    }

    #[test]
    fn default_moves_knight() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♞')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMove::default_of("D4", "E6"),
                GameMove::default_of("D4", "F5"),
                GameMove::default_of("D4", "F3"),
                GameMove::default_of("D4", "E2"),
                GameMove::default_of("D4", "C2"),
                GameMove::default_of("D4", "B3"),
                GameMove::default_of("D4", "B5"),
                GameMove::default_of("D4", "C6"),
            ]
        );
    }

    #[test]
    fn default_moves_bishop() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♝')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::default_of("C5", "D6"),
                GameMove::default_of("C5", "E7"),
                GameMove::default_of("C5", "F8"),
                GameMove::default_of("C5", "D4"),
                GameMove::default_of("C5", "E3"),
                GameMove::default_of("C5", "F2"),
                GameMove::default_of("C5", "G1"),
                GameMove::default_of("C5", "B4"),
                GameMove::default_of("C5", "A3"),
                GameMove::default_of("C5", "B6"),
                GameMove::default_of("C5", "A7"),
            ]
        );
    }

    #[test]
    fn default_moves_queen() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♛')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::default_of("C5", "D6"),
                GameMove::default_of("C5", "E7"),
                GameMove::default_of("C5", "F8"),
                GameMove::default_of("C5", "D4"),
                GameMove::default_of("C5", "E3"),
                GameMove::default_of("C5", "F2"),
                GameMove::default_of("C5", "G1"),
                GameMove::default_of("C5", "B4"),
                GameMove::default_of("C5", "A3"),
                GameMove::default_of("C5", "B6"),
                GameMove::default_of("C5", "A7"),
                GameMove::default_of("C5", "D5"),
                GameMove::default_of("C5", "E5"),
                GameMove::default_of("C5", "F5"),
                GameMove::default_of("C5", "G5"),
                GameMove::default_of("C5", "H5"),
                GameMove::default_of("C5", "C4"),
                GameMove::default_of("C5", "C3"),
                GameMove::default_of("C5", "C2"),
                GameMove::default_of("C5", "C1"),
                GameMove::default_of("C5", "B5"),
                GameMove::default_of("C5", "A5"),
                GameMove::default_of("C5", "C6"),
                GameMove::default_of("C5", "C7"),
                GameMove::default_of("C5", "C8"),
            ]
        );
    }

    #[test]
    fn default_moves_king() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♚')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMove::default_of("D4", "E5"),
                GameMove::default_of("D4", "E4"),
                GameMove::default_of("D4", "E3"),
                GameMove::default_of("D4", "D3"),
                GameMove::default_of("D4", "C3"),
                GameMove::default_of("D4", "C4"),
                GameMove::default_of("D4", "C5"),
                GameMove::default_of("D4", "D5"),
            ]
        );
    }

    #[test]
    fn default_moves_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♙')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::default_of("C5", "C6"),
                GameMove::menace_of("C5", "B6"),
                GameMove::menace_of("C5", "D6"),
            ]
        );
    }
}
