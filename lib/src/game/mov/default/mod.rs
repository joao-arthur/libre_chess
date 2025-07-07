use crate::{
    game::{board::GameBoard, game::GameBounds, mov::GameMov},
    piece::Type,
    pos::Pos,
};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub fn moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMov> {
    if let Some(piece) = board.get(pos) {
        return match piece.t {
            Type::Rook => rook::moves(board, bounds, pos),
            Type::Knight => knight::moves(board, bounds, pos),
            Type::Bishop => bishop::moves(board, bounds, pos),
            Type::Queen => queen::moves(board, bounds, pos),
            Type::King => king::moves(board, bounds, pos),
            Type::Pawn => pawn::moves(board, bounds, pos),
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
            mov::{DefaultMov, GameMov, MenaceMov},
            piece::piece_of_str,
        },
        mov::Mov,
        pos::Pos,
    };

    use super::moves;

    #[test]
    fn movements_empty_board() {
        let mode = standard_chess();
        assert_eq!(moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_rook() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("D4", '♜')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "E4"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "F4"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "G4"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "H4"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "D3"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "D2"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "D1"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "C4"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "B4"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "A4"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "D5"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "D6"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "D7"))),
                GameMov::from(DefaultMov::from(Mov::of('♜', "D4", "D8"))),
            ]
        );
    }

    #[test]
    fn movements_knight() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("D4", '♞')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "E6"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "F5"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "F3"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "E2"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "C2"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "B3"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "B5"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "C6"))),
            ]
        );
    }

    #[test]
    fn movements_bishop() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♝')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "D6"))),
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "E7"))),
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "F8"))),
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "D4"))),
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "E3"))),
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "F2"))),
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "G1"))),
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "B4"))),
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "A3"))),
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "B6"))),
                GameMov::from(DefaultMov::from(Mov::of('♝', "C5", "A7"))),
            ]
        );
    }

    #[test]
    fn movements_queen() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♛')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "D6"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "E7"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "F8"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "D4"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "E3"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "F2"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "G1"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "B4"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "A3"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "B6"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "A7"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "D5"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "E5"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "F5"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "G5"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "H5"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "C4"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "C3"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "C2"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "C1"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "B5"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "A5"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "C6"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "C7"))),
                GameMov::from(DefaultMov::from(Mov::of('♛', "C5", "C8"))),
            ]
        );
    }

    #[test]
    fn movements_king() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("D4", '♚')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "E5"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "E4"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "E3"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "D3"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "C3"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "C4"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "C5"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "D5"))),
            ]
        );
    }

    #[test]
    fn movements_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♙')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♙', "C5", "C6"))),
                GameMov::from(MenaceMov::from(Mov::of('♙', "C5", "B6"))),
                GameMov::from(MenaceMov::from(Mov::of('♙', "C5", "D6"))),
            ]
        );
    }
}
