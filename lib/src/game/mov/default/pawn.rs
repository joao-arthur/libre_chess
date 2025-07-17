use std::collections::HashMap;

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
                    vec![pos.rel_idx(1, 0), pos.rel_idx(2, 0)]
                } else {
                    vec![pos.rel_idx(1, 0)]
                }
            }
            Color::Black => {
                if pos.row == 6 {
                    vec![pos.rel_idx(-1, 0), pos.rel_idx(-2, 0)]
                } else {
                    vec![pos.rel_idx(-1, 0)]
                }
            }
        };
        let capture_base = match &piece.color {
            Color::White => [pos.try_rel_idx(1, -1), pos.try_rel_idx(1, 1)],
            Color::Black => [pos.try_rel_idx(-1, -1), pos.try_rel_idx(-1, 1)],
        };
        for curr_pos in move_base {
            if board.get(&curr_pos).is_none() {
                result.insert(curr_pos, PieceMoveType::Default);
            }
        }
        for curr_pos in capture_base {
            if let Some(curr_pos) = curr_pos {
                if curr_pos.col < bounds.x1
                    || curr_pos.col > bounds.x2
                    || curr_pos.row < bounds.y1
                    || curr_pos.row > bounds.y2
                {
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
        pos::Pos,
    };

    use super::pawn_moves;

    #[test]
    fn pawn_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(pawn_moves(&board_empty(), &mode.bounds, &Pos::of("A1")), HashMap::new());
    }

    #[test]
    fn pawn_moves_lonely_white_pawn() {
        let mode = standard_chess();
        let board = [(Pos::of("C5"), Piece::of('♙'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("C5")),
            [(Pos::of("C6"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_lonely_black_pawn() {
        let mode = standard_chess();
        let board = [(Pos::of("C5"), Piece::of('♟'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("C5")),
            [(Pos::of("C4"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_first_move_white_pawn() {
        let mode = standard_chess();
        let board = [(Pos::of("A2"), Piece::of('♙'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("A2")),
            [(Pos::of("A3"), PieceMoveType::Default), (Pos::of("A4"), PieceMoveType::Default)]
                .into()
        );
    }

    #[test]
    fn pawn_moves_first_move_black_pawn() {
        let mode = standard_chess();
        let board = [(Pos::of("H7"), Piece::of('♟'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("H7")),
            [(Pos::of("H6"), PieceMoveType::Default), (Pos::of("H5"), PieceMoveType::Default)]
                .into()
        );
    }

    #[test]
    fn pawn_moves_blocked_white_pawn() {
        let mode = standard_chess();
        let board = [(Pos::of("C5"), Piece::of('♙')), (Pos::of("C6"), Piece::of('♟'))].into();
        assert_eq!(pawn_moves(&board, &mode.bounds, &Pos::of("C5")), HashMap::new());
    }

    #[test]
    fn pawn_moves_blocked_black_pawn() {
        let mode = standard_chess();
        let board = [(Pos::of("C5"), Piece::of('♟')), (Pos::of("C4"), Piece::of('♙'))].into();
        assert_eq!(pawn_moves(&board, &mode.bounds, &Pos::of("C5")), HashMap::new());
    }

    #[test]
    fn pawn_moves_left_bounds_white_pawn() {
        let mode = standard_chess();
        let board = [(Pos::of("A3"), Piece::of('♙'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("A3")),
            [(Pos::of("A4"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_right_bounds_white_pawn() {
        let mode = standard_chess();
        let board = [(Pos::of("H3"), Piece::of('♙'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("H3")),
            [(Pos::of("H4"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_left_bounds_black_pawn() {
        let mode = standard_chess();
        let board = [(Pos::of("A6"), Piece::of('♟'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("A6")),
            [(Pos::of("A5"), PieceMoveType::Default)].into()
        );
    }

    #[test]
    fn pawn_moves_right_bounds_black_pawn() {
        let mode = standard_chess();
        let board = [(Pos::of("H6"), Piece::of('♟'))].into();
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("H6")),
            [(Pos::of("H5"), PieceMoveType::Default)].into()
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
            pawn_moves(&board, &mode.bounds, &Pos::of("C5")),
            [(Pos::of("C6"), PieceMoveType::Default), (Pos::of("B6"), PieceMoveType::Default)]
                .into()
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
            pawn_moves(&board, &mode.bounds, &Pos::of("C5")),
            [(Pos::of("C4"), PieceMoveType::Default), (Pos::of("D4"), PieceMoveType::Default)]
                .into()
        );
    }
}
