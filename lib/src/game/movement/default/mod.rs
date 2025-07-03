use crate::{
    board::pos::Pos,
    game::{board::GameBoard, game::GameBounds, movement::movement::DefaultMovement},
    piece::Type,
};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub fn movements(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<DefaultMovement> {
    if let Some(piece) = board.get(pos) {
        return match piece.t {
            Type::Rook => rook::movements(board, bounds, pos),
            Type::Knight => knight::movements(board, bounds, pos),
            Type::Bishop => bishop::movements(board, bounds, pos),
            Type::Queen => queen::movements(board, bounds, pos),
            Type::King => king::movements(board, bounds, pos),
            Type::Pawn => pawn::movements(board, bounds, pos),
        };
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::Pos,
        game::{
            board::board_empty, mode::standard_chess, movement::movement::DefaultMovement,
            piece::piece_of_str,
        },
        movement::Movement,
    };

    use super::movements;

    #[test]
    fn movements_empty_board() {
        let mode = standard_chess();
        assert_eq!(movements(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_rook() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("D4", '♜')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                DefaultMovement::from(Movement::of_str('♜', "D4", "E4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "F4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "G4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "H4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D3")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D2")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D1")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "C4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "B4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "A4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D5")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D6")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D7")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D8")),
            ]
        );
    }

    #[test]
    fn movements_knight() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("D4", '♞')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                DefaultMovement::from(Movement::of_str('♞', "D4", "E6")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "F5")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "F3")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "E2")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "C2")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "B3")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "B5")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "C6")),
            ]
        );
    }

    #[test]
    fn movements_bishop() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♝')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                DefaultMovement::from(Movement::of_str('♝', "C5", "D6")),
                DefaultMovement::from(Movement::of_str('♝', "C5", "E7")),
                DefaultMovement::from(Movement::of_str('♝', "C5", "F8")),
                DefaultMovement::from(Movement::of_str('♝', "C5", "D4")),
                DefaultMovement::from(Movement::of_str('♝', "C5", "E3")),
                DefaultMovement::from(Movement::of_str('♝', "C5", "F2")),
                DefaultMovement::from(Movement::of_str('♝', "C5", "G1")),
                DefaultMovement::from(Movement::of_str('♝', "C5", "B4")),
                DefaultMovement::from(Movement::of_str('♝', "C5", "A3")),
                DefaultMovement::from(Movement::of_str('♝', "C5", "B6")),
                DefaultMovement::from(Movement::of_str('♝', "C5", "A7")),
            ]
        );
    }

    #[test]
    fn movements_queen() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♛')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                DefaultMovement::from(Movement::of_str('♛', "C5", "D6")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "E7")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "F8")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "D4")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "E3")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "F2")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "G1")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "B4")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "A3")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "B6")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "A7")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "D5")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "E5")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "F5")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "G5")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "H5")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "C4")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "C3")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "C2")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "C1")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "B5")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "A5")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "C6")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "C7")),
                DefaultMovement::from(Movement::of_str('♛', "C5", "C8")),
            ]
        );
    }

    #[test]
    fn movements_king() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("D4", '♚')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                DefaultMovement::from(Movement::of_str('♚', "D4", "E5")),
                DefaultMovement::from(Movement::of_str('♚', "D4", "E4")),
                DefaultMovement::from(Movement::of_str('♚', "D4", "E3")),
                DefaultMovement::from(Movement::of_str('♚', "D4", "D3")),
                DefaultMovement::from(Movement::of_str('♚', "D4", "C3")),
                DefaultMovement::from(Movement::of_str('♚', "D4", "C4")),
                DefaultMovement::from(Movement::of_str('♚', "D4", "C5")),
                DefaultMovement::from(Movement::of_str('♚', "D4", "D5")),
            ]
        );
    }

    #[test]
    fn movements_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♙')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [DefaultMovement::from(Movement::of_str('♙', "C5", "C6"))]
        );
    }
}
