use std::collections::HashSet;

use crate::{
    board::pos::Pos, color::Color, game::board::GameBoard, geometry::poligon::rect::RectU8,
    piece::Type,
};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub fn menace_of_piece(board: &GameBoard, bounds: &RectU8, pos: &Pos) -> Vec<Pos> {
    if let Some(piece) = board.get(pos) {
        return match piece.t {
            Type::Rook => rook::menace(board, bounds, pos),
            Type::Knight => knight::menace(board, bounds, pos),
            Type::Bishop => bishop::menace(board, bounds, pos),
            Type::Queen => queen::menace(board, bounds, pos),
            Type::King => king::menace(board, bounds, pos),
            Type::Pawn => pawn::menace(board, bounds, pos),
        };
    }
    Vec::new()
}

pub fn menace_of_player(board: &GameBoard, bounds: &RectU8, color: &Color) -> HashSet<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    for (pos, piece) in board.iter() {
        if &piece.color == color {
            result.append(&mut menace_of_piece(board, bounds, pos));
        }
    }
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::{Pos, pos_of_str_slice},
        game::{board, mode::standard_chess, piece},
    };

    use super::menace_of_piece;

    #[test]
    fn menace_of_piece_empty_board() {
        let bounds = standard_chess().bounds;
        assert_eq!(menace_of_piece(&board::empty(), &bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn menace_of_piece_rook() {
        let board = HashMap::from([piece::of_str("D4", '♜')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace_of_piece(&board, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice([
                "E4", "F4", "G4", "H4", "D3", "D2", "D1", "C4", "B4", "A4", "D5", "D6", "D7", "D8",
            ])
        );
    }

    #[test]
    fn menace_of_piece_knight() {
        let board = HashMap::from([piece::of_str("D4", '♞')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace_of_piece(&board, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E6", "F5", "F3", "E2", "C2", "B3", "B5", "C6"])
        );
    }

    #[test]
    fn menace_of_piece_bishop() {
        let board = HashMap::from([piece::of_str("C5", '♝')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace_of_piece(&board, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice(["D6", "E7", "F8", "D4", "E3", "F2", "G1", "B4", "A3", "B6", "A7"])
        );
    }

    #[test]
    fn menace_of_piece_queen() {
        let board = HashMap::from([piece::of_str("C5", '♛')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace_of_piece(&board, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice([
                "D6", "E7", "F8", "D4", "E3", "F2", "G1", "B4", "A3", "B6", "A7", "D5", "E5", "F5",
                "G5", "H5", "C4", "C3", "C2", "C1", "B5", "A5", "C6", "C7", "C8",
            ])
        );
    }

    #[test]
    fn menace_of_piece_king() {
        let board = HashMap::from([piece::of_str("D4", '♚')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace_of_piece(&board, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "E3", "D3", "C3", "C4", "C5", "D5"])
        );
    }

    #[test]
    fn menace_of_piece_pawn() {
        let board = HashMap::from([piece::of_str("C5", '♙')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace_of_piece(&board, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice(["B6", "D6"])
        );
    }
}
