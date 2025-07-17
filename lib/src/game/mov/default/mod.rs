use std::collections::HashMap;

use crate::{
    game::{board::GameBoard, game::GameBounds, mov::PieceMoveType},
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

pub fn default_moves(
    board: &GameBoard,
    bounds: &GameBounds,
    pos: &Pos,
) -> HashMap<Pos, PieceMoveType> {
    if let Some(piece) = board.get(pos) {
        match piece.typ {
            PieceType::Rook => rook_moves(board, bounds, pos),
            PieceType::Knight => knight_moves(board, bounds, pos),
            PieceType::Bishop => bishop_moves(board, bounds, pos),
            PieceType::Queen => queen_moves(board, bounds, pos),
            PieceType::King => king_moves(board, bounds, pos),
            PieceType::Pawn => pawn_moves(board, bounds, pos),
        }
    } else {
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        game::{
            board::board_empty, mode::standard_chess, mov::PieceMoveType, piece::game_piece_of,
        },
        pos::Pos,
    };

    use super::default_moves;

    #[test]
    fn default_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(default_moves(&board_empty(), &mode.bounds, &Pos::of("A1")), HashMap::new());
    }

    #[test]
    fn default_moves_rook() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♜')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of("D4")),
            HashMap::from([
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("F4"), PieceMoveType::Default),
                (Pos::of("G4"), PieceMoveType::Default),
                (Pos::of("H4"), PieceMoveType::Default),
                (Pos::of("D3"), PieceMoveType::Default),
                (Pos::of("D2"), PieceMoveType::Default),
                (Pos::of("D1"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A4"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("D7"), PieceMoveType::Default),
                (Pos::of("D8"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn default_moves_knight() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♞')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of("D4")),
            HashMap::from([
                (Pos::of("E6"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("F3"), PieceMoveType::Default),
                (Pos::of("E2"), PieceMoveType::Default),
                (Pos::of("C2"), PieceMoveType::Default),
                (Pos::of("B3"), PieceMoveType::Default),
                (Pos::of("B5"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn default_moves_bishop() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♝')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of("C5")),
            HashMap::from([
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("E7"), PieceMoveType::Default),
                (Pos::of("F8"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("E3"), PieceMoveType::Default),
                (Pos::of("F2"), PieceMoveType::Default),
                (Pos::of("G1"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("B6"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn default_moves_queen() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♛')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of("C5")),
            HashMap::from([
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("E7"), PieceMoveType::Default),
                (Pos::of("F8"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("E3"), PieceMoveType::Default),
                (Pos::of("F2"), PieceMoveType::Default),
                (Pos::of("G1"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("B6"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("G5"), PieceMoveType::Default),
                (Pos::of("H5"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("C2"), PieceMoveType::Default),
                (Pos::of("C1"), PieceMoveType::Default),
                (Pos::of("B5"), PieceMoveType::Default),
                (Pos::of("A5"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
                (Pos::of("C7"), PieceMoveType::Default),
                (Pos::of("C8"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn default_moves_king() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♚')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of("D4")),
            HashMap::from([
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("E3"), PieceMoveType::Default),
                (Pos::of("D3"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("C5"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn default_moves_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♙')]);
        assert_eq!(
            default_moves(&board, &mode.bounds, &Pos::of("C5")),
            HashMap::from([(Pos::of("C6"), PieceMoveType::Default),])
        );
    }
}
