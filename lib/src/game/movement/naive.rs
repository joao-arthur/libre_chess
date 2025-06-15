use std::collections::HashSet;

use crate::{board::pos::Pos, color::Color, game::board::Board, piece::Type};

use super::{
    bishop::naive_movements_bishop, king::naive_movements_king, knight::naive_movements_knight,
    pawn::naive_movements_pawn, queen::naive_movements_queen, rook::naive_movements_rook,
};

pub fn naive_movements_piece(board: &Board, pos: &Pos) -> Vec<Pos> {
    if let Some(piece) = board.get(&pos) {
        return match piece.t {
            Type::Rook => naive_movements_rook(&board, pos),
            Type::Knight => naive_movements_knight(&board, pos),
            Type::Bishop => naive_movements_bishop(&board, pos),
            Type::Queen => naive_movements_queen(&board, pos),
            Type::King => naive_movements_king(&board, pos),
            Type::Pawn => naive_movements_pawn(&board, pos),
        };
    }
    return Vec::new();
}

pub fn naive_movements_board(board: &Board, color: &Color) -> HashSet<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    for entry in board.iter() {
        if &entry.1.color == color {
            result.append(&mut naive_movements_piece(board, &entry.0));
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

    use super::{naive_movements_board, naive_movements_piece};

    #[test]
    fn naive_movements_piece_rook() {
        assert_eq!(
            naive_movements_piece(&HashMap::from([piece::of_str("D4", "♜")]), &Pos::of_str("D4")),
            pos_of_str_slice([
                "E4", "F4", "G4", "H4", "D3", "D2", "D1", "C4", "B4", "A4", "D5", "D6", "D7", "D8",
            ])
        );
    }

    #[test]
    fn naive_movements_piece_knight() {
        assert_eq!(
            naive_movements_piece(&HashMap::from([piece::of_str("D4", "♞")]), &Pos::of_str("D4")),
            pos_of_str_slice(["E6", "F5", "F3", "E2", "C2", "B3", "B5", "C6"])
        );
    }

    #[test]
    fn naive_movements_piece_bishop() {
        assert_eq!(
            naive_movements_piece(&HashMap::from([piece::of_str("C5", "♝")]), &Pos::of_str("C5")),
            pos_of_str_slice(["D6", "E7", "F8", "D4", "E3", "F2", "G1", "B4", "A3", "B6", "A7"])
        );
    }

    #[test]
    fn naive_movements_piece_queen() {
        assert_eq!(
            naive_movements_piece(&HashMap::from([piece::of_str("C5", "♛")]), &Pos::of_str("C5")),
            pos_of_str_slice([
                "D6", "E7", "F8", "D4", "E3", "F2", "G1", "B4", "A3", "B6", "A7", "D5", "E5", "F5",
                "G5", "H5", "C4", "C3", "C2", "C1", "B5", "A5", "C6", "C7", "C8",
            ])
        );
    }

    #[test]
    fn naive_movements_piece_king() {
        assert_eq!(
            naive_movements_piece(&HashMap::from([piece::of_str("D4", "♚")]), &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "E3", "D3", "C3", "C4", "C5", "D5"])
        );
    }

    #[test]
    fn naive_movements_piece_pawn() {
        assert_eq!(
            naive_movements_piece(&HashMap::from([piece::of_str("C5", "♙")]), &Pos::of_str("C5")),
            [Pos::of_str("C6")]
        );
    }

    #[test]
    fn naive_movements_piece_empty_board() {
        assert_eq!(naive_movements_piece(&board::empty(), &Pos::of_str("A1")), []);
    }
}
