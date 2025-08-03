use std::collections::HashMap;

use crate::{
    game::{board::GameBoard, game::GameBounds, mov::PieceMoveType},
    pos::Pos,
};

use super::{bishop::bishop_moves, rook::rook_moves};

pub fn queen_moves(
    board: &GameBoard,
    bounds: &GameBounds,
    pos: &Pos,
) -> HashMap<Pos, PieceMoveType> {
    let mut result = HashMap::new();
    result.extend(bishop_moves(board, bounds, pos));
    result.extend(rook_moves(board, bounds, pos));
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

    use super::queen_moves;

    #[test]
    fn queen_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(queen_moves(&board_empty(), &mode.bounds, &pos_of("A1")), HashMap::new());
    }

    #[test]
    fn queen_moves_lonely_piece() {
        let mode = standard_chess();
        let board = [(pos_of("C5"), Piece::of('♛'))].into();
        assert_eq!(
            queen_moves(&board, &mode.bounds, &pos_of("C5")),
            [
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("E7"), PieceMoveType::Default),
                (pos_of("F8"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("E3"), PieceMoveType::Default),
                (pos_of("F2"), PieceMoveType::Default),
                (pos_of("G1"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("B6"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("G5"), PieceMoveType::Default),
                (pos_of("H5"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("C2"), PieceMoveType::Default),
                (pos_of("C1"), PieceMoveType::Default),
                (pos_of("B5"), PieceMoveType::Default),
                (pos_of("A5"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default),
                (pos_of("C7"), PieceMoveType::Default),
                (pos_of("C8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn queen_moves_top_right_edge() {
        let mode = standard_chess();
        let board = [(pos_of("H8"), Piece::of('♛'))].into();
        assert_eq!(
            queen_moves(&board, &mode.bounds, &pos_of("H8")),
            [
                (pos_of("G7"), PieceMoveType::Default),
                (pos_of("F6"), PieceMoveType::Default),
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("B2"), PieceMoveType::Default),
                (pos_of("A1"), PieceMoveType::Default),
                (pos_of("H7"), PieceMoveType::Default),
                (pos_of("H6"), PieceMoveType::Default),
                (pos_of("H5"), PieceMoveType::Default),
                (pos_of("H4"), PieceMoveType::Default),
                (pos_of("H3"), PieceMoveType::Default),
                (pos_of("H2"), PieceMoveType::Default),
                (pos_of("H1"), PieceMoveType::Default),
                (pos_of("G8"), PieceMoveType::Default),
                (pos_of("F8"), PieceMoveType::Default),
                (pos_of("E8"), PieceMoveType::Default),
                (pos_of("D8"), PieceMoveType::Default),
                (pos_of("C8"), PieceMoveType::Default),
                (pos_of("B8"), PieceMoveType::Default),
                (pos_of("A8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn queen_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = [(pos_of("H1"), Piece::of('♛'))].into();
        assert_eq!(
            queen_moves(&board, &mode.bounds, &pos_of("H1")),
            [
                (pos_of("G2"), PieceMoveType::Default),
                (pos_of("F3"), PieceMoveType::Default),
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default),
                (pos_of("B7"), PieceMoveType::Default),
                (pos_of("A8"), PieceMoveType::Default),
                (pos_of("G1"), PieceMoveType::Default),
                (pos_of("F1"), PieceMoveType::Default),
                (pos_of("E1"), PieceMoveType::Default),
                (pos_of("D1"), PieceMoveType::Default),
                (pos_of("C1"), PieceMoveType::Default),
                (pos_of("B1"), PieceMoveType::Default),
                (pos_of("A1"), PieceMoveType::Default),
                (pos_of("H2"), PieceMoveType::Default),
                (pos_of("H3"), PieceMoveType::Default),
                (pos_of("H4"), PieceMoveType::Default),
                (pos_of("H5"), PieceMoveType::Default),
                (pos_of("H6"), PieceMoveType::Default),
                (pos_of("H7"), PieceMoveType::Default),
                (pos_of("H8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn queen_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = [(pos_of("A1"), Piece::of('♛'))].into();
        assert_eq!(
            queen_moves(&board, &mode.bounds, &pos_of("A1")),
            [
                (pos_of("B2"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("F6"), PieceMoveType::Default),
                (pos_of("G7"), PieceMoveType::Default),
                (pos_of("H8"), PieceMoveType::Default),
                (pos_of("B1"), PieceMoveType::Default),
                (pos_of("C1"), PieceMoveType::Default),
                (pos_of("D1"), PieceMoveType::Default),
                (pos_of("E1"), PieceMoveType::Default),
                (pos_of("F1"), PieceMoveType::Default),
                (pos_of("G1"), PieceMoveType::Default),
                (pos_of("H1"), PieceMoveType::Default),
                (pos_of("A2"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("A4"), PieceMoveType::Default),
                (pos_of("A5"), PieceMoveType::Default),
                (pos_of("A6"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
                (pos_of("A8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn queen_moves_top_left_edge() {
        let mode = standard_chess();
        let board = [(pos_of("A8"), Piece::of('♛'))].into();
        assert_eq!(
            queen_moves(&board, &mode.bounds, &pos_of("A8")),
            [
                (pos_of("B7"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("F3"), PieceMoveType::Default),
                (pos_of("G2"), PieceMoveType::Default),
                (pos_of("H1"), PieceMoveType::Default),
                (pos_of("B8"), PieceMoveType::Default),
                (pos_of("C8"), PieceMoveType::Default),
                (pos_of("D8"), PieceMoveType::Default),
                (pos_of("E8"), PieceMoveType::Default),
                (pos_of("F8"), PieceMoveType::Default),
                (pos_of("G8"), PieceMoveType::Default),
                (pos_of("H8"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
                (pos_of("A6"), PieceMoveType::Default),
                (pos_of("A5"), PieceMoveType::Default),
                (pos_of("A4"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("A2"), PieceMoveType::Default),
                (pos_of("A1"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn queen_moves_small_bounds() {
        let board = [(pos_of("F6"), Piece::of('♛'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            queen_moves(&board, &bounds, &pos_of("F6")),
            [
                (pos_of("G7"), PieceMoveType::Default),
                (pos_of("H8"), PieceMoveType::Default),
                (pos_of("G5"), PieceMoveType::Default),
                (pos_of("H4"), PieceMoveType::Default),
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("E7"), PieceMoveType::Default),
                (pos_of("D8"), PieceMoveType::Default),
                (pos_of("G6"), PieceMoveType::Default),
                (pos_of("H6"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("F4"), PieceMoveType::Default),
                (pos_of("E6"), PieceMoveType::Default),
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("F7"), PieceMoveType::Default),
                (pos_of("F8"), PieceMoveType::Default)
            ]
            .into()
        );
    }

    #[test]
    fn queen_moves_white_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "  ♗     ",
                "   ♜    ",
                "  ♕   ♝ ",
                "        ",
                "♜   ♖   ",
                "  ♝     ",
                "        ",
            ],
        );
        assert_eq!(
            queen_moves(&board, &mode.bounds, &pos_of("C5")),
            [
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("B6"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("G5"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("C2"), PieceMoveType::Default),
                (pos_of("B5"), PieceMoveType::Default),
                (pos_of("A5"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn queen_moves_black_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "  ♝     ",
                "   ♖    ",
                "  ♛   ♗ ",
                "        ",
                "♖   ♜   ",
                "  ♗     ",
                "        ",
            ],
        );
        assert_eq!(
            queen_moves(&board, &mode.bounds, &pos_of("C5")),
            [
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("B6"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("G5"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("C2"), PieceMoveType::Default),
                (pos_of("B5"), PieceMoveType::Default),
                (pos_of("A5"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default),
            ]
            .into()
        );
    }
}
