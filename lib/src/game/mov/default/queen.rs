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
            piece::game_piece_of,
        },
        pos::Pos,
    };

    use super::queen_moves;

    #[test]
    fn queen_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(queen_moves(&board_empty(), &mode.bounds, &Pos::of("A1")), HashMap::new());
    }

    #[test]
    fn queen_moves_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♛')]);
        assert_eq!(
            queen_moves(&board, &mode.bounds, &Pos::of("C5")),
            HashMap::from([
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("E7"), PieceMoveType::Default),
                (Pos::of("F8"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("E3"), PieceMoveType::Default),
                (Pos::of("F2"), PieceMoveType::Default),
                (Pos::of("G1"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("B6"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("G5"), PieceMoveType::Default),
                (Pos::of("H5"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("C2"), PieceMoveType::Default),
                (Pos::of("C1"), PieceMoveType::Default),
                (Pos::of("B5"), PieceMoveType::Default),
                (Pos::of("A5"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
                (Pos::of("C7"), PieceMoveType::Default),
                (Pos::of("C8"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn queen_moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H8", '♛')]);
        assert_eq!(
            queen_moves(&board, &mode.bounds, &Pos::of("H8")),
            HashMap::from([
                (Pos::of("G7"), PieceMoveType::Default),
                (Pos::of("F6"), PieceMoveType::Default),
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("B2"), PieceMoveType::Default),
                (Pos::of("A1"), PieceMoveType::Default),
                (Pos::of("H7"), PieceMoveType::Default),
                (Pos::of("H6"), PieceMoveType::Default),
                (Pos::of("H5"), PieceMoveType::Default),
                (Pos::of("H4"), PieceMoveType::Default),
                (Pos::of("H3"), PieceMoveType::Default),
                (Pos::of("H2"), PieceMoveType::Default),
                (Pos::of("H1"), PieceMoveType::Default),
                (Pos::of("G8"), PieceMoveType::Default),
                (Pos::of("F8"), PieceMoveType::Default),
                (Pos::of("E8"), PieceMoveType::Default),
                (Pos::of("D8"), PieceMoveType::Default),
                (Pos::of("C8"), PieceMoveType::Default),
                (Pos::of("B8"), PieceMoveType::Default),
                (Pos::of("A8"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn queen_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H1", '♛')]);
        assert_eq!(
            queen_moves(&board, &mode.bounds, &Pos::of("H1")),
            HashMap::from([
                (Pos::of("G2"), PieceMoveType::Default),
                (Pos::of("F3"), PieceMoveType::Default),
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
                (Pos::of("B7"), PieceMoveType::Default),
                (Pos::of("A8"), PieceMoveType::Default),
                (Pos::of("G1"), PieceMoveType::Default),
                (Pos::of("F1"), PieceMoveType::Default),
                (Pos::of("E1"), PieceMoveType::Default),
                (Pos::of("D1"), PieceMoveType::Default),
                (Pos::of("C1"), PieceMoveType::Default),
                (Pos::of("B1"), PieceMoveType::Default),
                (Pos::of("A1"), PieceMoveType::Default),
                (Pos::of("H2"), PieceMoveType::Default),
                (Pos::of("H3"), PieceMoveType::Default),
                (Pos::of("H4"), PieceMoveType::Default),
                (Pos::of("H5"), PieceMoveType::Default),
                (Pos::of("H6"), PieceMoveType::Default),
                (Pos::of("H7"), PieceMoveType::Default),
                (Pos::of("H8"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn queen_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A1", '♛')]);
        assert_eq!(
            queen_moves(&board, &mode.bounds, &Pos::of("A1")),
            HashMap::from([
                (Pos::of("B2"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("F6"), PieceMoveType::Default),
                (Pos::of("G7"), PieceMoveType::Default),
                (Pos::of("H8"), PieceMoveType::Default),
                (Pos::of("B1"), PieceMoveType::Default),
                (Pos::of("C1"), PieceMoveType::Default),
                (Pos::of("D1"), PieceMoveType::Default),
                (Pos::of("E1"), PieceMoveType::Default),
                (Pos::of("F1"), PieceMoveType::Default),
                (Pos::of("G1"), PieceMoveType::Default),
                (Pos::of("H1"), PieceMoveType::Default),
                (Pos::of("A2"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("A4"), PieceMoveType::Default),
                (Pos::of("A5"), PieceMoveType::Default),
                (Pos::of("A6"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
                (Pos::of("A8"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn queen_moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A8", '♛')]);
        assert_eq!(
            queen_moves(&board, &mode.bounds, &Pos::of("A8")),
            HashMap::from([
                (Pos::of("B7"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("F3"), PieceMoveType::Default),
                (Pos::of("G2"), PieceMoveType::Default),
                (Pos::of("H1"), PieceMoveType::Default),
                (Pos::of("B8"), PieceMoveType::Default),
                (Pos::of("C8"), PieceMoveType::Default),
                (Pos::of("D8"), PieceMoveType::Default),
                (Pos::of("E8"), PieceMoveType::Default),
                (Pos::of("F8"), PieceMoveType::Default),
                (Pos::of("G8"), PieceMoveType::Default),
                (Pos::of("H8"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
                (Pos::of("A6"), PieceMoveType::Default),
                (Pos::of("A5"), PieceMoveType::Default),
                (Pos::of("A4"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("A2"), PieceMoveType::Default),
                (Pos::of("A1"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn queen_moves_small_bounds() {
        let board = HashMap::from([game_piece_of("F6", '♛')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            queen_moves(&board, &bounds, &Pos::of("F6")),
            HashMap::from([
                (Pos::of("G7"), PieceMoveType::Default),
                (Pos::of("H8"), PieceMoveType::Default),
                (Pos::of("G5"), PieceMoveType::Default),
                (Pos::of("H4"), PieceMoveType::Default),
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("E7"), PieceMoveType::Default),
                (Pos::of("D8"), PieceMoveType::Default),
                (Pos::of("G6"), PieceMoveType::Default),
                (Pos::of("H6"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("F4"), PieceMoveType::Default),
                (Pos::of("E6"), PieceMoveType::Default),
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("F7"), PieceMoveType::Default),
                (Pos::of("F8"), PieceMoveType::Default)
            ])
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
            queen_moves(&board, &mode.bounds, &Pos::of("C5")),
            HashMap::from([
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("B6"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("G5"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("C2"), PieceMoveType::Default),
                (Pos::of("B5"), PieceMoveType::Default),
                (Pos::of("A5"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
            ])
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
            queen_moves(&board, &mode.bounds, &Pos::of("C5")),
            HashMap::from([
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("B6"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("G5"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("C2"), PieceMoveType::Default),
                (Pos::of("B5"), PieceMoveType::Default),
                (Pos::of("A5"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
            ])
        );
    }
}
