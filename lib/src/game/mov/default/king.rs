use std::collections::HashMap;

use manfredo::matrix::{
    point::{point_i8::PointI8, point_u8::checked_translated},
    rect::rect_u8::contains,
};

use crate::{
    game::{board::GameBoard, game::GameBounds, mov::PieceMoveType},
    pos::Pos,
};

pub fn king_moves(
    board: &GameBoard,
    bounds: &GameBounds,
    pos: &Pos,
) -> HashMap<Pos, PieceMoveType> {
    let mut result = HashMap::new();
    if let Some(piece) = board.get(pos) {
        let base = [
            checked_translated(pos, &PointI8::of(1, 1)),
            checked_translated(pos, &PointI8::of(0, 1)),
            checked_translated(pos, &PointI8::of(-1, 1)),
            checked_translated(pos, &PointI8::of(-1, 0)),
            checked_translated(pos, &PointI8::of(-1, -1)),
            checked_translated(pos, &PointI8::of(0, -1)),
            checked_translated(pos, &PointI8::of(1, -1)),
            checked_translated(pos, &PointI8::of(1, 0)),
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

    use super::king_moves;

    #[test]
    fn king_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(king_moves(&board_empty(), &mode.bounds, &pos_of("A1")), HashMap::new());
    }

    #[test]
    fn king_moves_lonely_piece() {
        let mode = standard_chess();
        let board = [(pos_of("D4"), Piece::of('♚'))].into();
        assert_eq!(
            king_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("E3"), PieceMoveType::Default),
                (pos_of("D3"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("C5"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn king_moves_top_right_edge() {
        let mode = standard_chess();
        let board = [(pos_of("H8"), Piece::of('♚'))].into();
        assert_eq!(
            king_moves(&board, &mode.bounds, &pos_of("H8")),
            [
                (pos_of("H7"), PieceMoveType::Default),
                (pos_of("G7"), PieceMoveType::Default),
                (pos_of("G8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn king_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = [(pos_of("H1"), Piece::of('♚'))].into();
        assert_eq!(
            king_moves(&board, &mode.bounds, &pos_of("H1")),
            [
                (pos_of("G1"), PieceMoveType::Default),
                (pos_of("G2"), PieceMoveType::Default),
                (pos_of("H2"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn king_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = [(pos_of("A1"), Piece::of('♚'))].into();
        assert_eq!(
            king_moves(&board, &mode.bounds, &pos_of("A1")),
            [
                (pos_of("B2"), PieceMoveType::Default),
                (pos_of("B1"), PieceMoveType::Default),
                (pos_of("A2"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn king_moves_top_left_edge() {
        let mode = standard_chess();
        let board = [(pos_of("A8"), Piece::of('♚'))].into();
        assert_eq!(
            king_moves(&board, &mode.bounds, &pos_of("A8")),
            [
                (pos_of("B8"), PieceMoveType::Default),
                (pos_of("B7"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn king_moves_small_bounds_top_right_edge() {
        let board = [(pos_of("H8"), Piece::of('♚'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            king_moves(&board, &bounds, &pos_of("H8")),
            [
                (pos_of("H7"), PieceMoveType::Default),
                (pos_of("G7"), PieceMoveType::Default),
                (pos_of("G8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn king_moves_small_bounds_bottom_right_edge() {
        let board = [(pos_of("H4"), Piece::of('♚'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            king_moves(&board, &bounds, &pos_of("H4")),
            [
                (pos_of("G4"), PieceMoveType::Default),
                (pos_of("G5"), PieceMoveType::Default),
                (pos_of("H5"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn king_moves_small_bounds_bottom_left_edge() {
        let board = [(pos_of("D4"), Piece::of('♚'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            king_moves(&board, &bounds, &pos_of("D4")),
            [
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn king_moves_small_bounds_top_left_edge() {
        let board = [(pos_of("D8"), Piece::of('♚'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            king_moves(&board, &bounds, &pos_of("D8")),
            [
                (pos_of("E8"), PieceMoveType::Default),
                (pos_of("E7"), PieceMoveType::Default),
                (pos_of("D7"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn king_moves_white_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "        ",
                "        ",
                "    ♜   ",
                "  ♝♔    ",
                "   ♖    ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            king_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("E3"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("C5"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn king_moves_black_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "        ",
                "        ",
                "    ♖   ",
                "  ♗♚    ",
                "   ♜    ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            king_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("E3"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("C5"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
            ]
            .into()
        );
    }
}
