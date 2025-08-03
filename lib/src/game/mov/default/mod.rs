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
        game::{board::board_empty, mode::standard_chess, mov::PieceMoveType},
        piece::Piece,
        pos::pos_of,
    };

    use super::default_moves;

    #[test]
    fn default_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(default_moves(&board_empty(), &mode.bounds, &pos_of("A1")), HashMap::new());
    }

    #[test]
    fn default_moves_rook() {
        let mode = standard_chess();
        let board = [(pos_of("D4"), Piece::of('♜'))].into();
        assert_eq!(
            default_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("F4"), PieceMoveType::Default),
                (pos_of("G4"), PieceMoveType::Default),
                (pos_of("H4"), PieceMoveType::Default),
                (pos_of("D3"), PieceMoveType::Default),
                (pos_of("D2"), PieceMoveType::Default),
                (pos_of("D1"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A4"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("D7"), PieceMoveType::Default),
                (pos_of("D8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn default_moves_knight() {
        let mode = standard_chess();
        let board = [(pos_of("D4"), Piece::of('♞'))].into();
        assert_eq!(
            default_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E6"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("F3"), PieceMoveType::Default),
                (pos_of("E2"), PieceMoveType::Default),
                (pos_of("C2"), PieceMoveType::Default),
                (pos_of("B3"), PieceMoveType::Default),
                (pos_of("B5"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn default_moves_bishop() {
        let mode = standard_chess();
        let board = [(pos_of("C5"), Piece::of('♝'))].into();
        assert_eq!(
            default_moves(&board, &mode.bounds, &pos_of("C5")),
            [
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("E7"), PieceMoveType::Default),
                (pos_of("F8"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("E3"), PieceMoveType::Default),
                (pos_of("F2"), PieceMoveType::Default),
                (pos_of("G1"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("B6"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn default_moves_queen() {
        let mode = standard_chess();
        let board = [(pos_of("C5"), Piece::of('♛'))].into();
        assert_eq!(
            default_moves(&board, &mode.bounds, &pos_of("C5")),
            [
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("E7"), PieceMoveType::Default),
                (pos_of("F8"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("E3"), PieceMoveType::Default),
                (pos_of("F2"), PieceMoveType::Default),
                (pos_of("G1"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("B6"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("G5"), PieceMoveType::Default),
                (pos_of("H5"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("C2"), PieceMoveType::Default),
                (pos_of("C1"), PieceMoveType::Default),
                (pos_of("B5"), PieceMoveType::Default),
                (pos_of("A5"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default),
                (pos_of("C7"), PieceMoveType::Default),
                (pos_of("C8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn default_moves_king() {
        let mode = standard_chess();
        let board = [(pos_of("D4"), Piece::of('♚'))].into();
        assert_eq!(
            default_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("E3"), PieceMoveType::Default),
                (pos_of("D3"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("C5"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn default_moves_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("C5"), Piece::of('♙'))].into();
        assert_eq!(
            default_moves(&board, &mode.bounds, &pos_of("C5")),
            [(pos_of("C6"), PieceMoveType::Default)].into()
        );
    }
}
