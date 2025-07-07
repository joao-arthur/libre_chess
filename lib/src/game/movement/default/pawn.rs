use crate::{
    board::pos::Pos,
    color::Color,
    game::{
        board::GameBoard,
        game::GameBounds,
        movement::movement::{CaptureMovement, DefaultMovement, GameMovement, MenaceMovement},
    },
    movement::Movement,
};

pub fn movements(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMovement> {
    let mut result: Vec<GameMovement> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let move_base = match &piece.color {
            Color::White => {
                if pos.row == 1 {
                    vec![pos.try_of_rel_idx(1, 0), pos.try_of_rel_idx(2, 0)]
                } else {
                    vec![pos.try_of_rel_idx(1, 0)]
                }
            }
            Color::Black => {
                if pos.row == 6 {
                    vec![pos.try_of_rel_idx(-1, 0), pos.try_of_rel_idx(-2, 0)]
                } else {
                    vec![pos.try_of_rel_idx(-1, 0)]
                }
            }
        };
        let capture_base = match &piece.color {
            Color::White => [pos.try_of_rel_idx(1, -1), pos.try_of_rel_idx(1, 1)],
            Color::Black => [pos.try_of_rel_idx(-1, -1), pos.try_of_rel_idx(-1, 1)],
        };
        for curr_pos in move_base {
            if let Some(curr_pos) = curr_pos {
                if curr_pos.col < bounds.x1
                    || curr_pos.col > bounds.x2
                    || curr_pos.row < bounds.y1
                    || curr_pos.row > bounds.y2
                {
                    continue;
                }
                if board.get(&curr_pos).is_none() {
                    result.push(GameMovement::from(DefaultMovement::from(Movement {
                        piece: *piece,
                        from: pos.clone(),
                        to: curr_pos,
                    })));
                }
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
                        result.push(GameMovement::from(MenaceMovement::from(Movement {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        })));
                    } else {
                        result.push(GameMovement::from(CaptureMovement::from(Movement {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        })));
                    }
                } else {
                    result.push(GameMovement::from(MenaceMovement::from(Movement {
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
        board::pos::Pos,
        game::{
            board::{board_empty, board_of_str},
            mode::standard_chess,
            movement::movement::{CaptureMovement, DefaultMovement, GameMovement, MenaceMovement},
            piece::piece_of_str,
        },
        movement::Movement,
    };

    use super::movements;

    #[test]
    fn movements_empty_board() {
        let mode = standard_chess();
        assert_eq!(movements(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♙')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "C5", "C6"))),
                GameMovement::from(MenaceMovement::from(Movement::of_str('♙', "C5", "B6"))),
                GameMovement::from(MenaceMovement::from(Movement::of_str('♙', "C5", "D6"))),
            ]
        );
    }

    #[test]
    fn movements_lonely_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♟')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "C5", "C4"))),
                GameMovement::from(MenaceMovement::from(Movement::of_str('♟', "C5", "B4"))),
                GameMovement::from(MenaceMovement::from(Movement::of_str('♟', "C5", "D4"))),
            ]
        );
    }

    #[test]
    fn movements_first_move_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A2", '♙')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A2")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "A2", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "A2", "A4"))),
                GameMovement::from(MenaceMovement::from(Movement::of_str('♙', "A2", "B3"))),
            ]
        );
    }

    #[test]
    fn movements_first_move_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H7", '♟')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H7")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "H7", "H6"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "H7", "H5"))),
                GameMovement::from(MenaceMovement::from(Movement::of_str('♟', "H7", "G6"))),
            ]
        );
    }

    #[test]
    fn movements_blocked_white_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♙'), piece_of_str("C6", '♟')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(MenaceMovement::from(Movement::of_str('♙', "C5", "B6"))),
                GameMovement::from(MenaceMovement::from(Movement::of_str('♙', "C5", "D6"))),
            ]
        );
    }

    #[test]
    fn movements_blocked_black_pawn() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♟'), piece_of_str("C4", '♙')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(MenaceMovement::from(Movement::of_str('♟', "C5", "B4"))),
                GameMovement::from(MenaceMovement::from(Movement::of_str('♟', "C5", "D4"))),
            ]
        );
    }

    #[test]
    fn movements_white_capture() {
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
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "C5", "C6"))),
                GameMovement::from(CaptureMovement::from(Movement::of_str('♙', "C5", "B6"))),
                GameMovement::from(MenaceMovement::from(Movement::of_str('♙', "C5", "D6"))),
            ]
        );
    }

    #[test]
    fn movements_black_capture() {
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
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "C5", "C4"))),
                GameMovement::from(MenaceMovement::from(Movement::of_str('♟', "C5", "B4"))),
                GameMovement::from(CaptureMovement::from(Movement::of_str('♟', "C5", "D4"))),
            ]
        );
    }
}
