use std::collections::HashMap;

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
            pos.try_rel_idx(2, 1),
            pos.try_rel_idx(1, 2),
            pos.try_rel_idx(-1, 2),
            pos.try_rel_idx(-2, 1),
            pos.try_rel_idx(-2, -1),
            pos.try_rel_idx(-1, -2),
            pos.try_rel_idx(1, -2),
            pos.try_rel_idx(2, -1),
        ];
        for curr_pos in base {
            if let Some(curr_pos) = curr_pos {
                if curr_pos.col < bounds.min.x
                    || curr_pos.col > bounds.max.x
                    || curr_pos.row < bounds.min.y
                    || curr_pos.row > bounds.max.y
                {
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
        pos::Pos,
    };

    use super::knight_moves;

    #[test]
    fn knight_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(knight_moves(&board_empty(), &mode.bounds, &Pos::of("A1")), HashMap::new());
    }

    #[test]
    fn knight_moves_lonely_piece() {
        let mode = standard_chess();
        let board = [(Pos::of("D4"), Piece::of('♞'))].into();
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of("D4")),
            [
                (Pos::of("E6"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("F3"), PieceMoveType::Default),
                (Pos::of("E2"), PieceMoveType::Default),
                (Pos::of("C2"), PieceMoveType::Default),
                (Pos::of("B3"), PieceMoveType::Default),
                (Pos::of("B5"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn knight_moves_top_right_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("H8"), Piece::of('♞'))].into();
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of("H8")),
            [(Pos::of("G6"), PieceMoveType::Default), (Pos::of("F7"), PieceMoveType::Default)]
                .into()
        );
    }

    #[test]
    fn knight_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("H1"), Piece::of('♞'))].into();
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of("H1")),
            [(Pos::of("F2"), PieceMoveType::Default), (Pos::of("G3"), PieceMoveType::Default)]
                .into()
        );
    }

    #[test]
    fn knight_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("A1"), Piece::of('♞'))].into();
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of("A1")),
            [(Pos::of("B3"), PieceMoveType::Default), (Pos::of("C2"), PieceMoveType::Default)]
                .into()
        );
    }

    #[test]
    fn knight_moves_top_left_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("A8"), Piece::of('♞'))].into();
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of("A8")),
            [(Pos::of("C7"), PieceMoveType::Default), (Pos::of("B6"), PieceMoveType::Default)]
                .into()
        );
    }

    #[test]
    fn knight_moves_small_bounds_top_right_edge() {
        let board = [(Pos::of("G7"), Piece::of('♞'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            knight_moves(&board, &bounds, &Pos::of("G7")),
            [
                (Pos::of("H5"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("E6"), PieceMoveType::Default),
                (Pos::of("E8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn knight_moves_small_bounds_bottom_right_edge() {
        let board = [(Pos::of("G5"), Piece::of('♞'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            knight_moves(&board, &bounds, &Pos::of("G5")),
            [
                (Pos::of("H7"), PieceMoveType::Default),
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("E6"), PieceMoveType::Default),
                (Pos::of("F7"), PieceMoveType::Default)
            ]
            .into()
        );
    }

    #[test]
    fn knight_moves_small_bounds_bottom_left_edge() {
        let board = [(Pos::of("E5"), Piece::of('♞'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            knight_moves(&board, &bounds, &Pos::of("E5")),
            [
                (Pos::of("F7"), PieceMoveType::Default),
                (Pos::of("G6"), PieceMoveType::Default),
                (Pos::of("G4"), PieceMoveType::Default),
                (Pos::of("D7"), PieceMoveType::Default)
            ]
            .into()
        );
    }

    #[test]
    fn knight_moves_small_bounds_top_left_edge() {
        let board = [(Pos::of("E7"), Piece::of('♞'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            knight_moves(&board, &bounds, &Pos::of("E7")),
            [
                (Pos::of("G8"), PieceMoveType::Default),
                (Pos::of("G6"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
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
            knight_moves(&board, &mode.bounds, &Pos::of("D4")),
            [
                (Pos::of("E6"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("F3"), PieceMoveType::Default),
                (Pos::of("E2"), PieceMoveType::Default),
                (Pos::of("C2"), PieceMoveType::Default),
                (Pos::of("B5"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
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
            knight_moves(&board, &mode.bounds, &Pos::of("D4")),
            [
                (Pos::of("E6"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("F3"), PieceMoveType::Default),
                (Pos::of("E2"), PieceMoveType::Default),
                (Pos::of("C2"), PieceMoveType::Default),
                (Pos::of("B5"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default)
            ]
            .into()
        );
    }
}
