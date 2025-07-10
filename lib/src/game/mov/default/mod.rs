use crate::{
    game::{board::GameBoard, game::GameBounds, mov::GameMovOld},
    piece::Type,
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

pub fn default_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMovOld> {
    if let Some(piece) = board.get(pos) {
        return match piece.t {
            Type::Rook => rook_moves(board, bounds, pos),
            Type::Knight => knight_moves(board, bounds, pos),
            Type::Bishop => bishop_moves(board, bounds, pos),
            Type::Queen => queen_moves(board, bounds, pos),
            Type::King => king_moves(board, bounds, pos),
            Type::Pawn => pawn_moves(board, bounds, pos),
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
            mov::{DefaultMovOld, GameMovOld, MenaceMovOld},
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
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "F4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "G4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "H4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "C4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "B4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "A4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D8"))),
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
                GameMovOld::from(DefaultMovOld::from(Mov::of('♞', "D4", "E6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♞', "D4", "F5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♞', "D4", "F3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♞', "D4", "E2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♞', "D4", "C2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♞', "D4", "B3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♞', "D4", "B5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♞', "D4", "C6"))),
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
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "D6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "E7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "F8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "D4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "E3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "F2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "G1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "B4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "A3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "B6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "A7"))),
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
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "D6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "E7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "F8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "D4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "E3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "F2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "G1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "B4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "A3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "B6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "A7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "D5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "E5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "F5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "G5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "H5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "C4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "C3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "C2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "C1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "B5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "A5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "C6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "C7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♛', "C5", "C8"))),
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
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "E5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "E3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "D3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "C3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "C4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "C5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "D5"))),
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
                GameMovOld::from(DefaultMovOld::from(Mov::of('♙', "C5", "C6"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♙', "C5", "B6"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♙', "C5", "D6"))),
            ]
        );
    }
}
