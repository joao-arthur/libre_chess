use crate::{
    game::{board::GameBoard, game::GameBounds, mov::GameMove},
    pos::Pos,
};

use super::{bishop::bishop_moves, rook::rook_moves};

pub fn queen_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMove> {
    let mut result: Vec<GameMove> = Vec::new();
    result.append(&mut bishop_moves(board, bounds, pos));
    result.append(&mut rook_moves(board, bounds, pos));
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
            mov::GameMove,
            piece::game_piece_of,
        },
        pos::Pos,
    };

    use super::queen_moves;

    #[test]
    fn queen_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(queen_moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn queen_moves_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♛')]);
        assert_eq!(
            queen_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::default_of('♛', "C5", "D6"),
                GameMove::default_of('♛', "C5", "E7"),
                GameMove::default_of('♛', "C5", "F8"),
                GameMove::default_of('♛', "C5", "D4"),
                GameMove::default_of('♛', "C5", "E3"),
                GameMove::default_of('♛', "C5", "F2"),
                GameMove::default_of('♛', "C5", "G1"),
                GameMove::default_of('♛', "C5", "B4"),
                GameMove::default_of('♛', "C5", "A3"),
                GameMove::default_of('♛', "C5", "B6"),
                GameMove::default_of('♛', "C5", "A7"),
                GameMove::default_of('♛', "C5", "D5"),
                GameMove::default_of('♛', "C5", "E5"),
                GameMove::default_of('♛', "C5", "F5"),
                GameMove::default_of('♛', "C5", "G5"),
                GameMove::default_of('♛', "C5", "H5"),
                GameMove::default_of('♛', "C5", "C4"),
                GameMove::default_of('♛', "C5", "C3"),
                GameMove::default_of('♛', "C5", "C2"),
                GameMove::default_of('♛', "C5", "C1"),
                GameMove::default_of('♛', "C5", "B5"),
                GameMove::default_of('♛', "C5", "A5"),
                GameMove::default_of('♛', "C5", "C6"),
                GameMove::default_of('♛', "C5", "C7"),
                GameMove::default_of('♛', "C5", "C8"),
            ]
        );
    }

    #[test]
    fn queen_moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H8", '♛')]);
        assert_eq!(
            queen_moves(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMove::default_of('♛', "H8", "G7"),
                GameMove::default_of('♛', "H8", "F6"),
                GameMove::default_of('♛', "H8", "E5"),
                GameMove::default_of('♛', "H8", "D4"),
                GameMove::default_of('♛', "H8", "C3"),
                GameMove::default_of('♛', "H8", "B2"),
                GameMove::default_of('♛', "H8", "A1"),
                GameMove::default_of('♛', "H8", "H7"),
                GameMove::default_of('♛', "H8", "H6"),
                GameMove::default_of('♛', "H8", "H5"),
                GameMove::default_of('♛', "H8", "H4"),
                GameMove::default_of('♛', "H8", "H3"),
                GameMove::default_of('♛', "H8", "H2"),
                GameMove::default_of('♛', "H8", "H1"),
                GameMove::default_of('♛', "H8", "G8"),
                GameMove::default_of('♛', "H8", "F8"),
                GameMove::default_of('♛', "H8", "E8"),
                GameMove::default_of('♛', "H8", "D8"),
                GameMove::default_of('♛', "H8", "C8"),
                GameMove::default_of('♛', "H8", "B8"),
                GameMove::default_of('♛', "H8", "A8"),
            ]
        );
    }

    #[test]
    fn queen_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H1", '♛')]);
        assert_eq!(
            queen_moves(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMove::default_of('♛', "H1", "G2"),
                GameMove::default_of('♛', "H1", "F3"),
                GameMove::default_of('♛', "H1", "E4"),
                GameMove::default_of('♛', "H1", "D5"),
                GameMove::default_of('♛', "H1", "C6"),
                GameMove::default_of('♛', "H1", "B7"),
                GameMove::default_of('♛', "H1", "A8"),
                GameMove::default_of('♛', "H1", "G1"),
                GameMove::default_of('♛', "H1", "F1"),
                GameMove::default_of('♛', "H1", "E1"),
                GameMove::default_of('♛', "H1", "D1"),
                GameMove::default_of('♛', "H1", "C1"),
                GameMove::default_of('♛', "H1", "B1"),
                GameMove::default_of('♛', "H1", "A1"),
                GameMove::default_of('♛', "H1", "H2"),
                GameMove::default_of('♛', "H1", "H3"),
                GameMove::default_of('♛', "H1", "H4"),
                GameMove::default_of('♛', "H1", "H5"),
                GameMove::default_of('♛', "H1", "H6"),
                GameMove::default_of('♛', "H1", "H7"),
                GameMove::default_of('♛', "H1", "H8")
            ]
        );
    }

    #[test]
    fn queen_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A1", '♛')]);
        assert_eq!(
            queen_moves(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMove::default_of('♛', "A1", "B2"),
                GameMove::default_of('♛', "A1", "C3"),
                GameMove::default_of('♛', "A1", "D4"),
                GameMove::default_of('♛', "A1", "E5"),
                GameMove::default_of('♛', "A1", "F6"),
                GameMove::default_of('♛', "A1", "G7"),
                GameMove::default_of('♛', "A1", "H8"),
                GameMove::default_of('♛', "A1", "B1"),
                GameMove::default_of('♛', "A1", "C1"),
                GameMove::default_of('♛', "A1", "D1"),
                GameMove::default_of('♛', "A1", "E1"),
                GameMove::default_of('♛', "A1", "F1"),
                GameMove::default_of('♛', "A1", "G1"),
                GameMove::default_of('♛', "A1", "H1"),
                GameMove::default_of('♛', "A1", "A2"),
                GameMove::default_of('♛', "A1", "A3"),
                GameMove::default_of('♛', "A1", "A4"),
                GameMove::default_of('♛', "A1", "A5"),
                GameMove::default_of('♛', "A1", "A6"),
                GameMove::default_of('♛', "A1", "A7"),
                GameMove::default_of('♛', "A1", "A8"),
            ]
        );
    }

    #[test]
    fn queen_moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A8", '♛')]);
        assert_eq!(
            queen_moves(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMove::default_of('♛', "A8", "B7"),
                GameMove::default_of('♛', "A8", "C6"),
                GameMove::default_of('♛', "A8", "D5"),
                GameMove::default_of('♛', "A8", "E4"),
                GameMove::default_of('♛', "A8", "F3"),
                GameMove::default_of('♛', "A8", "G2"),
                GameMove::default_of('♛', "A8", "H1"),
                GameMove::default_of('♛', "A8", "B8"),
                GameMove::default_of('♛', "A8", "C8"),
                GameMove::default_of('♛', "A8", "D8"),
                GameMove::default_of('♛', "A8", "E8"),
                GameMove::default_of('♛', "A8", "F8"),
                GameMove::default_of('♛', "A8", "G8"),
                GameMove::default_of('♛', "A8", "H8"),
                GameMove::default_of('♛', "A8", "A7"),
                GameMove::default_of('♛', "A8", "A6"),
                GameMove::default_of('♛', "A8", "A5"),
                GameMove::default_of('♛', "A8", "A4"),
                GameMove::default_of('♛', "A8", "A3"),
                GameMove::default_of('♛', "A8", "A2"),
                GameMove::default_of('♛', "A8", "A1"),
            ]
        );
    }

    #[test]
    fn queen_moves_small_bounds() {
        let board = HashMap::from([game_piece_of("F6", '♛')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            queen_moves(&board, &bounds, &Pos::of_str("F6")),
            [
                GameMove::default_of('♛', "F6", "G7"),
                GameMove::default_of('♛', "F6", "H8"),
                GameMove::default_of('♛', "F6", "G5"),
                GameMove::default_of('♛', "F6", "H4"),
                GameMove::default_of('♛', "F6", "E5"),
                GameMove::default_of('♛', "F6", "D4"),
                GameMove::default_of('♛', "F6", "E7"),
                GameMove::default_of('♛', "F6", "D8"),
                GameMove::default_of('♛', "F6", "G6"),
                GameMove::default_of('♛', "F6", "H6"),
                GameMove::default_of('♛', "F6", "F5"),
                GameMove::default_of('♛', "F6", "F4"),
                GameMove::default_of('♛', "F6", "E6"),
                GameMove::default_of('♛', "F6", "D6"),
                GameMove::default_of('♛', "F6", "F7"),
                GameMove::default_of('♛', "F6", "F8")
            ]
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
            queen_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::capture_of('♕', "C5", "D6"),
                GameMove::default_of('♕', "C5", "D4"),
                GameMove::menace_of('♕', "C5", "E3"),
                GameMove::default_of('♕', "C5", "B4"),
                GameMove::capture_of('♕', "C5", "A3"),
                GameMove::default_of('♕', "C5", "B6"),
                GameMove::default_of('♕', "C5", "A7"),
                GameMove::default_of('♕', "C5", "D5"),
                GameMove::default_of('♕', "C5", "E5"),
                GameMove::default_of('♕', "C5", "F5"),
                GameMove::capture_of('♕', "C5", "G5"),
                GameMove::default_of('♕', "C5", "C4"),
                GameMove::default_of('♕', "C5", "C3"),
                GameMove::capture_of('♕', "C5", "C2"),
                GameMove::default_of('♕', "C5", "B5"),
                GameMove::default_of('♕', "C5", "A5"),
                GameMove::default_of('♕', "C5", "C6"),
                GameMove::menace_of('♕', "C5", "C7"),
            ]
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
            queen_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::capture_of('♛', "C5", "D6"),
                GameMove::default_of('♛', "C5", "D4"),
                GameMove::menace_of('♛', "C5", "E3"),
                GameMove::default_of('♛', "C5", "B4"),
                GameMove::capture_of('♛', "C5", "A3"),
                GameMove::default_of('♛', "C5", "B6"),
                GameMove::default_of('♛', "C5", "A7"),
                GameMove::default_of('♛', "C5", "D5"),
                GameMove::default_of('♛', "C5", "E5"),
                GameMove::default_of('♛', "C5", "F5"),
                GameMove::capture_of('♛', "C5", "G5"),
                GameMove::default_of('♛', "C5", "C4"),
                GameMove::default_of('♛', "C5", "C3"),
                GameMove::capture_of('♛', "C5", "C2"),
                GameMove::default_of('♛', "C5", "B5"),
                GameMove::default_of('♛', "C5", "A5"),
                GameMove::default_of('♛', "C5", "C6"),
                GameMove::menace_of('♛', "C5", "C7"),
            ]
        );
    }
}
