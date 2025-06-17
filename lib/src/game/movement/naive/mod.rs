use std::collections::HashSet;

use crate::{board::pos::Pos, color::Color, game::board::Board, piece::Type};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub fn movements_of_piece(board: &Board, pos: &Pos) -> Vec<Pos> {
    if let Some(piece) = board.get(pos) {
        return match piece.t {
            Type::Rook => rook::movements(board, pos),
            Type::Knight => knight::movements(board, pos),
            Type::Bishop => bishop::movements(board, pos),
            Type::Queen => queen::movements(board, pos),
            Type::King => king::movements(board, pos),
            Type::Pawn => pawn::movements(board, pos),
        };
    }
    Vec::new()
}

pub fn movements_of_player(board: &Board, color: &Color) -> HashSet<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    for (pos, piece) in board.iter() {
        if &piece.color == color {
            result.append(&mut movements_of_piece(board, pos));
        }
    }
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::{Pos, pos_of_str_slice},
        game::{board, piece},
    };

    use super::movements_of_piece;

    #[test]
    fn movements_of_piece_rook() {
        assert_eq!(
            movements_of_piece(&HashMap::from([piece::of_str("D4", "♜")]), &Pos::of_str("D4")),
            pos_of_str_slice([
                "E4", "F4", "G4", "H4", "D3", "D2", "D1", "C4", "B4", "A4", "D5", "D6", "D7", "D8",
            ])
        );
    }

    #[test]
    fn movements_of_piece_knight() {
        assert_eq!(
            movements_of_piece(&HashMap::from([piece::of_str("D4", "♞")]), &Pos::of_str("D4")),
            pos_of_str_slice(["E6", "F5", "F3", "E2", "C2", "B3", "B5", "C6"])
        );
    }

    #[test]
    fn movements_of_piece_bishop() {
        assert_eq!(
            movements_of_piece(&HashMap::from([piece::of_str("C5", "♝")]), &Pos::of_str("C5")),
            pos_of_str_slice(["D6", "E7", "F8", "D4", "E3", "F2", "G1", "B4", "A3", "B6", "A7"])
        );
    }

    #[test]
    fn movements_of_piece_queen() {
        assert_eq!(
            movements_of_piece(&HashMap::from([piece::of_str("C5", "♛")]), &Pos::of_str("C5")),
            pos_of_str_slice([
                "D6", "E7", "F8", "D4", "E3", "F2", "G1", "B4", "A3", "B6", "A7", "D5", "E5", "F5",
                "G5", "H5", "C4", "C3", "C2", "C1", "B5", "A5", "C6", "C7", "C8",
            ])
        );
    }

    #[test]
    fn movements_of_piece_king() {
        assert_eq!(
            movements_of_piece(&HashMap::from([piece::of_str("D4", "♚")]), &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "E3", "D3", "C3", "C4", "C5", "D5"])
        );
    }

    #[test]
    fn movements_of_piece_pawn() {
        assert_eq!(
            movements_of_piece(&HashMap::from([piece::of_str("C5", "♙")]), &Pos::of_str("C5")),
            [Pos::of_str("C6")]
        );
    }

    #[test]
    fn movements_of_piece_empty_board() {
        assert_eq!(movements_of_piece(&board::empty(), &Pos::of_str("A1")), []);
    }
}
