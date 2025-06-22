
use crate::{
    board::pos::Pos,
    game::{board::GameBoard, movement::movement::GameMovementOld},
    geometry::poligon::rect::RectU8,
    piece::Type,
};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub fn movements_of_piece(board: &GameBoard, bounds: &RectU8, pos: &Pos) -> Vec<GameMovementOld> {
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

//pub fn movements_of_player(board: &GameBoard, bounds: &RectU8, color: &Color) -> HashSet<Pos> {
//    let mut result: Vec<Pos> = Vec::new();
//    for (pos, piece) in board.iter() {
//        if &piece.color == color {
//            result.append(&mut movements_of_piece(board, bounds, pos));
//        }
//    }
//    result.into_iter().collect()
//}

#[cfg(test)]
mod tests {
    //    use std::collections::HashMap;
    //
    //    use crate::{
    //        board::pos::Pos,
    //        game::{board, mode::standard_chess, piece},
    //    };
    //
    //    use super::movements_of_piece;
    //
    //    #[test]
    //    fn movements_of_piece_empty_board() {
    //        let bounds = standard_chess().bounds;
    //        assert_eq!(movements_of_piece(&board::empty(), &bounds, &Pos::of_str("A1")), []);
    //    }
    //
    //    #[test]
    //    fn movements_of_piece_rook() {
    //        let board = HashMap::from([piece::of_str("D4", "♜")]);
    //        let bounds = standard_chess().bounds;
    //        assert_eq!(
    //            movements_of_piece(&board, &bounds, &Pos::of_str("D4")),
    //            [
    //                GameMovementOld::from(Movement::of_str("E4")),
    //                GameMovementOld::from(Movement::of_str("F4")),
    //                GameMovementOld::from(Movement::of_str("G4")),
    //                GameMovementOld::from(Movement::of_str("H4")),
    //                GameMovementOld::from(Movement::of_str("D3")),
    //                GameMovementOld::from(Movement::of_str("D2")),
    //                GameMovementOld::from(Movement::of_str("D1")),
    //                GameMovementOld::from(Movement::of_str("C4")),
    //                GameMovementOld::from(Movement::of_str("B4")),
    //                GameMovementOld::from(Movement::of_str("A4")),
    //                GameMovementOld::from(Movement::of_str("D5")),
    //                GameMovementOld::from(Movement::of_str("D6")),
    //                GameMovementOld::from(Movement::of_str("D7")),
    //                GameMovementOld::from(Movement::of_str("D8")),
    //            ]
    //        );
    //    }
    //
    //    #[test]
    //    fn movements_of_piece_knight() {
    //        let board = HashMap::from([piece::of_str("D4", "♞")]);
    //        let bounds = standard_chess().bounds;
    //        assert_eq!(
    //            movements_of_piece(&board, &bounds, &Pos::of_str("D4")),
    //            [
    //                GameMovementOld::from(Movement::of_str("E6")),
    //                GameMovementOld::from(Movement::of_str("F5")),
    //                GameMovementOld::from(Movement::of_str("F3")),
    //                GameMovementOld::from(Movement::of_str("E2")),
    //                GameMovementOld::from(Movement::of_str("C2")),
    //                GameMovementOld::from(Movement::of_str("B3")),
    //                GameMovementOld::from(Movement::of_str("B5")),
    //                GameMovementOld::from(Movement::of_str("C6")),
    //            ]
    //        );
    //    }
    //
    //    #[test]
    //    fn movements_of_piece_bishop() {
    //        let board = HashMap::from([piece::of_str("C5", "♝")]);
    //        let bounds = standard_chess().bounds;
    //        assert_eq!(
    //            movements_of_piece(&board, &bounds, &Pos::of_str("C5")),
    //            [
    //                GameMovementOld::from(Movement::of_str("D6")),
    //                GameMovementOld::from(Movement::of_str("E7")),
    //                GameMovementOld::from(Movement::of_str("F8")),
    //                GameMovementOld::from(Movement::of_str("D4")),
    //                GameMovementOld::from(Movement::of_str("E3")),
    //                GameMovementOld::from(Movement::of_str("F2")),
    //                GameMovementOld::from(Movement::of_str("G1")),
    //                GameMovementOld::from(Movement::of_str("B4")),
    //                GameMovementOld::from(Movement::of_str("A3")),
    //                GameMovementOld::from(Movement::of_str("B6")),
    //                GameMovementOld::from(Movement::of_str("A7")),
    //            ]
    //        );
    //    }
    //
    //    #[test]
    //    fn movements_of_piece_queen() {
    //        let board = HashMap::from([piece::of_str("C5", "♛")]);
    //        let bounds = standard_chess().bounds;
    //        assert_eq!(
    //            movements_of_piece(&board, &bounds, &Pos::of_str("C5")),
    //            [
    //                GameMovementOld::from(Movement::of_str("D6")),
    //                GameMovementOld::from(Movement::of_str("E7")),
    //                GameMovementOld::from(Movement::of_str("F8")),
    //                GameMovementOld::from(Movement::of_str("D4")),
    //                GameMovementOld::from(Movement::of_str("E3")),
    //                GameMovementOld::from(Movement::of_str("F2")),
    //                GameMovementOld::from(Movement::of_str("G1")),
    //                GameMovementOld::from(Movement::of_str("B4")),
    //                GameMovementOld::from(Movement::of_str("A3")),
    //                GameMovementOld::from(Movement::of_str("B6")),
    //                GameMovementOld::from(Movement::of_str("A7")),
    //                GameMovementOld::from(Movement::of_str("D5")),
    //                GameMovementOld::from(Movement::of_str("E5")),
    //                GameMovementOld::from(Movement::of_str("F5")),
    //                GameMovementOld::from(Movement::of_str("G5")),
    //                GameMovementOld::from(Movement::of_str("H5")),
    //                GameMovementOld::from(Movement::of_str("C4")),
    //                GameMovementOld::from(Movement::of_str("C3")),
    //                GameMovementOld::from(Movement::of_str("C2")),
    //                GameMovementOld::from(Movement::of_str("C1")),
    //                GameMovementOld::from(Movement::of_str("B5")),
    //                GameMovementOld::from(Movement::of_str("A5")),
    //                GameMovementOld::from(Movement::of_str("C6")),
    //                GameMovementOld::from(Movement::of_str("C7")),
    //                GameMovementOld::from(Movement::of_str("C8")),
    //            ]
    //        );
    //    }
    //
    //    #[test]
    //    fn movements_of_piece_king() {
    //        let board = HashMap::from([piece::of_str("D4", "♚")]);
    //        let bounds = standard_chess().bounds;
    //        assert_eq!(
    //            movements_of_piece(&board, &bounds, &Pos::of_str("D4")),
    //            [
    //                GameMovementOld::from(Movement::of_str("E5")),
    //                GameMovementOld::from(Movement::of_str("E4")),
    //                GameMovementOld::from(Movement::of_str("E3")),
    //                GameMovementOld::from(Movement::of_str("D3")),
    //                GameMovementOld::from(Movement::of_str("C3")),
    //                GameMovementOld::from(Movement::of_str("C4")),
    //                GameMovementOld::from(Movement::of_str("C5")),
    //                GameMovementOld::from(Movement::of_str("D5")),
    //            ]
    //        );
    //    }
    //
    //    #[test]
    //    fn movements_of_piece_pawn() {
    //        let board = HashMap::from([piece::of_str("C5", "♙")]);
    //        let bounds = standard_chess().bounds;
    //        assert_eq!(
    //            movements_of_piece(&board, &bounds, &Pos::of_str("C5")),
    //            [GameMovementOld::from(Movement::of_str("C6"))]
    //        );
    //    }
}
