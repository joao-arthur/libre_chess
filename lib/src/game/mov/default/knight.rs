use std::collections::HashMap;

use manfredo::matrix::{
    point::point_i8::PointI8, point::point_u8::checked_translated, rect::rect_u8::contains,
};

use crate::{
    game::{board::GameBoard, game::GameBounds, mov::PieceMoveType},
    pos::Pos,
};

pub fn knight_moves(
    board: &GameBoard,
    bounds: &GameBounds,
    pos: &Pos,
) -> HashMap<Pos, PieceMoveType> {
    let mut result = HashMap::new();
    if let Some(piece) = board.get(pos) {
        let base = [
            checked_translated(pos, &PointI8::of(2, 1)),
            checked_translated(pos, &PointI8::of(1, 2)),
            checked_translated(pos, &PointI8::of(-1, 2)),
            checked_translated(pos, &PointI8::of(-2, 1)),
            checked_translated(pos, &PointI8::of(-2, -1)),
            checked_translated(pos, &PointI8::of(-1, -2)),
            checked_translated(pos, &PointI8::of(1, -2)),
            checked_translated(pos, &PointI8::of(2, -1)),
        ];
        for curr_pos in base {
            if let Some(curr_pos) = curr_pos {
                if !contains(bounds, &curr_pos) {
                    continue;
                }
                if let Some(curr_piece) = board.get(&curr_pos) {
                    if curr_piece.color != piece.color {
                        result.insert(curr_pos, PieceMoveType::Default);
                    }
                } else {
                    result.insert(curr_pos, PieceMoveType::Default);
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
            game::GameBounds,
            mode::standard_chess,
            mov::PieceMoveType,
        },
        piece::Piece,
        pos::pos_of,
    };

    use super::knight_moves;

    #[test]
    fn knight_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(knight_moves(&board_empty(), &mode.bounds, &pos_of("A1")), HashMap::new());
    }

    #[test]
    fn knight_moves_lonely_piece() {
        let mode = standard_chess();
        let board = [(pos_of("D4"), Piece::of('♞'))].into();
        assert_eq!(
            knight_moves(&board, &mode.bounds, &pos_of("D4")),
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
    fn knight_moves_top_right_edge() {
        let mode = standard_chess();
        let board = [(pos_of("H8"), Piece::of('♞'))].into();
        assert_eq!(
            knight_moves(&board, &mode.bounds, &pos_of("H8")),
            [(pos_of("G6"), PieceMoveType::Default), (pos_of("F7"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn knight_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = [(pos_of("H1"), Piece::of('♞'))].into();
        assert_eq!(
            knight_moves(&board, &mode.bounds, &pos_of("H1")),
            [(pos_of("F2"), PieceMoveType::Default), (pos_of("G3"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn knight_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = [(pos_of("A1"), Piece::of('♞'))].into();
        assert_eq!(
            knight_moves(&board, &mode.bounds, &pos_of("A1")),
            [(pos_of("B3"), PieceMoveType::Default), (pos_of("C2"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn knight_moves_top_left_edge() {
        let mode = standard_chess();
        let board = [(pos_of("A8"), Piece::of('♞'))].into();
        assert_eq!(
            knight_moves(&board, &mode.bounds, &pos_of("A8")),
            [(pos_of("C7"), PieceMoveType::Default), (pos_of("B6"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn knight_moves_small_bounds_top_right_edge() {
        let board = [(pos_of("G7"), Piece::of('♞'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            knight_moves(&board, &bounds, &pos_of("G7")),
            [
                (pos_of("H5"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("E6"), PieceMoveType::Default),
                (pos_of("E8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn knight_moves_small_bounds_bottom_right_edge() {
        let board = [(pos_of("G5"), Piece::of('♞'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            knight_moves(&board, &bounds, &pos_of("G5")),
            [
                (pos_of("H7"), PieceMoveType::Default),
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("E6"), PieceMoveType::Default),
                (pos_of("F7"), PieceMoveType::Default)
            ]
            .into()
        );
    }

    #[test]
    fn knight_moves_small_bounds_bottom_left_edge() {
        let board = [(pos_of("E5"), Piece::of('♞'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            knight_moves(&board, &bounds, &pos_of("E5")),
            [
                (pos_of("F7"), PieceMoveType::Default),
                (pos_of("G6"), PieceMoveType::Default),
                (pos_of("G4"), PieceMoveType::Default),
                (pos_of("D7"), PieceMoveType::Default)
            ]
            .into()
        );
    }

    #[test]
    fn knight_moves_small_bounds_top_left_edge() {
        let board = [(pos_of("E7"), Piece::of('♞'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            knight_moves(&board, &bounds, &pos_of("E7")),
            [
                (pos_of("G8"), PieceMoveType::Default),
                (pos_of("G6"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn knight_moves_white_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "        ",
                "    ♜   ",
                "     ♜  ",
                "   ♘    ",
                " ♖      ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            knight_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E6"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("F3"), PieceMoveType::Default),
                (pos_of("E2"), PieceMoveType::Default),
                (pos_of("C2"), PieceMoveType::Default),
                (pos_of("B5"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn knight_moves_black_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "        ",
                "    ♖   ",
                "     ♖  ",
                "   ♞    ",
                " ♜      ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            knight_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E6"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("F3"), PieceMoveType::Default),
                (pos_of("E2"), PieceMoveType::Default),
                (pos_of("C2"), PieceMoveType::Default),
                (pos_of("B5"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default)
            ]
            .into()
        );
    }
}
