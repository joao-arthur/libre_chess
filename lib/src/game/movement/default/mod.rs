use crate::{
    game::{board::GameBoard, game::GameBounds, movement::movement::GameMovement},
    piece::Type,
    pos::Pos,
};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub fn moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMovement> {
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
            movement::movement::{DefaultMovement, GameMovement, MenaceMovement},
            piece::piece_of_str,
        },
        movement::Movement,
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
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "F4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "G4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "H4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "D3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "D2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "D1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "C4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "B4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "A4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "D5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "D6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "D7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♜', "D4", "D8"))),
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
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "E6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "F5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "F3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "E2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "C2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "B3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "B5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "C6"))),
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
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "D6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "E7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "F8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "F2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "G1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "B4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "B6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "A7"))),
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
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "D6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "E7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "F8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "F2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "G1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "B4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "B6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "A7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "D5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "F5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "G5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "H5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "B5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "A5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♛', "C5", "C8"))),
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
                GameMovement::from(DefaultMovement::from(Movement::of('♚', "D4", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♚', "D4", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♚', "D4", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♚', "D4", "D3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♚', "D4", "C3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♚', "D4", "C4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♚', "D4", "C5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♚', "D4", "D5"))),
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
                GameMovement::from(DefaultMovement::from(Movement::of('♙', "C5", "C6"))),
                GameMovement::from(MenaceMovement::from(Movement::of('♙', "C5", "B6"))),
                GameMovement::from(MenaceMovement::from(Movement::of('♙', "C5", "D6"))),
            ]
        );
    }
}
