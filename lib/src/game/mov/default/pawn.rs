use std::collections::HashMap;

use manfredo::matrix::{
    point::point_i8::PointI8, point::point_u8::checked_translated, rect::rect_u8::contains,
};

use crate::{
    color::Color,
    game::{board::GameBoard, game::GameBounds, mov::PieceMoveType},
    pos::Pos,
};

pub fn pawn_moves(
    board: &GameBoard,
    bounds: &GameBounds,
    pos: &Pos,
) -> HashMap<Pos, PieceMoveType> {
    let mut result = HashMap::new();
    if let Some(piece) = board.get(pos) {
        let move_base = match &piece.color {
            Color::White => {
                if pos.row == 1 {
                    vec![
                        checked_translated(pos, &PointI8::of(1, 0)).unwrap(),
                        checked_translated(pos, &PointI8::of(2, 0)).unwrap(),
                    ]
                } else {
                    vec![checked_translated(pos, &PointI8::of(1, 0)).unwrap()]
                }
            }
            Color::Black => {
                if pos.row == 6 {
                    vec![
                        checked_translated(pos, &PointI8::of(-1, 0)).unwrap(),
                        checked_translated(pos, &PointI8::of(-2, 0)).unwrap(),
                    ]
                } else {
                    vec![checked_translated(pos, &PointI8::of(-1, 0)).unwrap()]
                }
            }
        };
        let capture_base = match &piece.color {
            Color::White => [
                checked_translated(pos, &PointI8::of(1, -1)),
                checked_translated(pos, &PointI8::of(1, 1)),
            ],
            Color::Black => [
                checked_translated(pos, &PointI8::of(-1, -1)),
                checked_translated(pos, &PointI8::of(-1, 1)),
            ],
        };
        for curr_pos in move_base {
            if board.get(&curr_pos).is_none() {
                result.insert(curr_pos, PieceMoveType::Default);
            }
        }
        for curr_pos in capture_base {
            if let Some(curr_pos) = curr_pos {
                if !contains(bounds, &curr_pos) {
                    continue;
                }
                if let Some(curr_piece) = board.get(&curr_pos) {
                    if curr_piece.color != piece.color {
                        result.insert(curr_pos, PieceMoveType::Default);
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        game::{
            board::{board_empty, board_of_str},
            mode::standard_chess,
            mov::PieceMoveType,
        },
        piece::Piece,
        pos::pos_of,
    };

    use super::pawn_moves;

    #[test]
    fn pawn_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(pawn_moves(&board_empty(), &mode.bounds, &pos_of("A1")), HashMap::new());
    }

    #[test]
    fn pawn_moves_lonely_white_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("C5"), Piece::of('♙'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &pos_of("C5")),
            [(pos_of("C6"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_lonely_black_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("C5"), Piece::of('♟'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &pos_of("C5")),
            [(pos_of("C4"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_first_move_white_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("A2"), Piece::of('♙'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &pos_of("A2")),
            [(pos_of("A3"), PieceMoveType::Default), (pos_of("A4"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_first_move_black_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("H7"), Piece::of('♟'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &pos_of("H7")),
            [(pos_of("H6"), PieceMoveType::Default), (pos_of("H5"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_blocked_white_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("C5"), Piece::of('♙')), (pos_of("C6"), Piece::of('♟'))].into();
        assert_eq!(pawn_moves(&board, &mode.bounds, &pos_of("C5")), HashMap::new());
    }

    #[test]
    fn pawn_moves_blocked_black_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("C5"), Piece::of('♟')), (pos_of("C4"), Piece::of('♙'))].into();
        assert_eq!(pawn_moves(&board, &mode.bounds, &pos_of("C5")), HashMap::new());
    }

    #[test]
    fn pawn_moves_left_bounds_white_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("A3"), Piece::of('♙'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &pos_of("A3")),
            [(pos_of("A4"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_right_bounds_white_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("H3"), Piece::of('♙'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &pos_of("H3")),
            [(pos_of("H4"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_left_bounds_black_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("A6"), Piece::of('♟'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &pos_of("A6")),
            [(pos_of("A5"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_right_bounds_black_pawn() {
        let mode = standard_chess();
        let board = [(pos_of("H6"), Piece::of('♟'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &pos_of("H6")),
            [(pos_of("H5"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_white_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "        ",
                " ♟ ♙    ",
                "  ♙     ",
                "        ",
                "        ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &pos_of("C5")),
            [(pos_of("C6"), PieceMoveType::Default), (pos_of("B6"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_black_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "        ",
                "        ",
                "  ♟     ",
                " ♟ ♙    ",
                "        ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &pos_of("C5")),
            [(pos_of("C4"), PieceMoveType::Default), (pos_of("D4"), PieceMoveType::Default)].into()
        );
    }
}
