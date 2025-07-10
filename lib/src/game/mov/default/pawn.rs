use crate::{
    color::Color,
    game::{
        board::GameBoard,
        game::GameBounds,
        mov::{CaptureMovOld, DefaultMovOld, GameMovOld, MenaceMovOld},
    },
    mov::Mov,
    pos::Pos,
};

pub fn pawn_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMovOld> {
    let mut result: Vec<GameMovOld> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let move_base = match &piece.color {
            Color::White => {
                if pos.row == 1 {
                    vec![pos.of_rel_idx(1, 0), pos.of_rel_idx(2, 0)]
                } else {
                    vec![pos.of_rel_idx(1, 0)]
                }
            }
            Color::Black => {
                if pos.row == 6 {
                    vec![pos.of_rel_idx(-1, 0), pos.of_rel_idx(-2, 0)]
                } else {
                    vec![pos.of_rel_idx(-1, 0)]
                }
            }
        };
        let capture_base = match &piece.color {
            Color::White => [pos.try_of_rel_idx(1, -1), pos.try_of_rel_idx(1, 1)],
            Color::Black => [pos.try_of_rel_idx(-1, -1), pos.try_of_rel_idx(-1, 1)],
        };
        for curr_pos in move_base {
            if board.get(&curr_pos).is_none() {
                result.push(GameMovOld::from(DefaultMovOld::from(Mov {
                    piece: *piece,
                    from: pos.clone(),
                    to: curr_pos,
                })));
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
                        result.push(GameMovOld::from(MenaceMovOld::from(Mov {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        })));
                    } else {
                        result.push(GameMovOld::from(CaptureMovOld::from(Mov {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        })));
                    }
                } else {
                    result.push(GameMovOld::from(MenaceMovOld::from(Mov {
                        piece: *piece,
                        from: pos.clone(),
                        to: curr_pos,
                    })));
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
            mov::{CaptureMovOld, DefaultMovOld, GameMovOld, MenaceMovOld},
            piece::game_piece_of,
        },
        mov::Mov,
        pos::Pos,
    };

    use super::pawn_moves;

    #[test]
    fn pawn_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(pawn_moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn pawn_moves_lonely_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♙')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♙', "C5", "C6"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♙', "C5", "B6"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♙', "C5", "D6"))),
            ]
        );
    }

    #[test]
    fn pawn_moves_lonely_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♟')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "C5", "C4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♟', "C5", "B4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♟', "C5", "D4"))),
            ]
        );
    }

    #[test]
    fn pawn_moves_first_move_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A2", '♙')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of_str("A2")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♙', "A2", "A3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♙', "A2", "A4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♙', "A2", "B3"))),
            ]
        );
    }

    #[test]
    fn pawn_moves_first_move_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H7", '♟')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of_str("H7")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "H7", "H6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "H7", "H5"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♟', "H7", "G6"))),
            ]
        );
    }

    #[test]
    fn pawn_moves_blocked_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♙'), game_piece_of("C6", '♟')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovOld::from(MenaceMovOld::from(Mov::of('♙', "C5", "B6"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♙', "C5", "D6"))),
            ]
        );
    }

    #[test]
    fn pawn_moves_blocked_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♟'), game_piece_of("C4", '♙')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovOld::from(MenaceMovOld::from(Mov::of('♟', "C5", "B4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♟', "C5", "D4"))),
            ]
        );
    }

    #[test]
    fn pawn_moves_left_bounds_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A3", '♙')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of_str("A3")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♙', "A3", "A4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♙', "A3", "B4"))),
            ]
        );
    }

    #[test]
    fn pawn_moves_right_bounds_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H3", '♙')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of_str("H3")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♙', "H3", "H4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♙', "H3", "G4"))),
            ]
        );
    }

    #[test]
    fn pawn_moves_left_bounds_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A6", '♟')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of_str("A6")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "A6", "A5"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♟', "A6", "B5"))),
            ]
        );
    }

    #[test]
    fn pawn_moves_right_bounds_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H6", '♟')]);
        assert_eq!(
            pawn_moves(&board, &mode.bounds, &Pos::of_str("H6")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "H6", "H5"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♟', "H6", "G5"))),
            ]
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
            pawn_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♙', "C5", "C6"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♙', "C5", "B6"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♙', "C5", "D6"))),
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
            pawn_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "C5", "C4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♟', "C5", "B4"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♟', "C5", "D4"))),
            ]
        );
    }
}
