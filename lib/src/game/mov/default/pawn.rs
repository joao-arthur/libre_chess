use crate::{
    color::Color,
    game::{
        board::GameBoard,
        game::GameBounds,
        mov::{CaptureMove, DefaultMove, GameMove, GameMoveType, MenaceMove},
    },
    mov::Mov,
    pos::Pos,
};

pub fn pawn_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMove> {
    let mut result: Vec<GameMove> = Vec::new();
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
                result.push(GameMove {
                    mov: Mov { piece: *piece, from: pos.clone(), to: curr_pos },
                    typ: GameMoveType::Default(DefaultMove),
                });
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
                    if curr_piece.color == piece.color {
                        result.push(GameMove {
                            mov: Mov { piece: *piece, from: pos.clone(), to: curr_pos },
                            typ: GameMoveType::Menace(MenaceMove),
                        });
                    } else {
                        result.push(GameMove {
                            mov: Mov { piece: *piece, from: pos.clone(), to: curr_pos },
                            typ: GameMoveType::Capture(CaptureMove),
                        });
                    }
                } else {
                    result.push(GameMove {
                        mov: Mov { piece: *piece, from: pos.clone(), to: curr_pos },
                        typ: GameMoveType::Menace(MenaceMove),
                    });
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
            mov::GameMove,
            piece::game_piece_of,
        },
        pos::Pos,
    };

    use super::pawn_moves;

    #[test]
    fn pawn_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(pawn_moves(&board_empty(), &mode.bounds, &Pos::of("A1")), []);
    }

    #[test]
    fn pawn_moves_lonely_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♙')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("C5")),
            [
                GameMove::default_of('♙', "C5", "C6"),
                GameMove::menace_of('♙', "C5", "B6"),
                GameMove::menace_of('♙', "C5", "D6"),
            ]
        );
    }

    #[test]
    fn pawn_moves_lonely_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♟')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("C5")),
            [
                GameMove::default_of('♟', "C5", "C4"),
                GameMove::menace_of('♟', "C5", "B4"),
                GameMove::menace_of('♟', "C5", "D4"),
            ]
        );
    }

    #[test]
    fn pawn_moves_first_move_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A2", '♙')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("A2")),
            [
                GameMove::default_of('♙', "A2", "A3"),
                GameMove::default_of('♙', "A2", "A4"),
                GameMove::menace_of('♙', "A2", "B3"),
            ]
        );
    }

    #[test]
    fn pawn_moves_first_move_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H7", '♟')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("H7")),
            [
                GameMove::default_of('♟', "H7", "H6"),
                GameMove::default_of('♟', "H7", "H5"),
                GameMove::menace_of('♟', "H7", "G6"),
            ]
        );
    }

    #[test]
    fn pawn_moves_blocked_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♙'), game_piece_of("C6", '♟')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("C5")),
            [GameMove::menace_of('♙', "C5", "B6"), GameMove::menace_of('♙', "C5", "D6")]
        );
    }

    #[test]
    fn pawn_moves_blocked_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♟'), game_piece_of("C4", '♙')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("C5")),
            [GameMove::menace_of('♟', "C5", "B4"), GameMove::menace_of('♟', "C5", "D4")]
        );
    }

    #[test]
    fn pawn_moves_left_bounds_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A3", '♙')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("A3")),
            [GameMove::default_of('♙', "A3", "A4"), GameMove::menace_of('♙', "A3", "B4")]
        );
    }

    #[test]
    fn pawn_moves_right_bounds_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H3", '♙')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("H3")),
            [GameMove::default_of('♙', "H3", "H4"), GameMove::menace_of('♙', "H3", "G4")]
        );
    }

    #[test]
    fn pawn_moves_left_bounds_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A6", '♟')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("A6")),
            [GameMove::default_of('♟', "A6", "A5"), GameMove::menace_of('♟', "A6", "B5")]
        );
    }

    #[test]
    fn pawn_moves_right_bounds_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H6", '♟')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of("H6")),
            [GameMove::default_of('♟', "H6", "H5"), GameMove::menace_of('♟', "H6", "G5")]
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
            [
                GameMove::default_of('♙', "C5", "C6"),
                GameMove::capture_of('♙', "C5", "B6"),
                GameMove::menace_of('♙', "C5", "D6"),
            ]
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
            [
                GameMove::default_of('♟', "C5", "C4"),
                GameMove::menace_of('♟', "C5", "B4"),
                GameMove::capture_of('♟', "C5", "D4"),
            ]
        );
    }
}
