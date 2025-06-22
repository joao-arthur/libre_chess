use crate::{
    board::pos::Pos,
    color::Color,
    game::{board::GameBoard, movement::movement::DefaultMovement},
    geometry::poligon::rect::RectU8,
    movement::Movement,
};

pub fn movements(board: &GameBoard, bounds: &RectU8, pos: &Pos) -> Vec<DefaultMovement> {
    let mut result: Vec<DefaultMovement> = Vec::new();
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
                if board.get(&curr_pos).is_none() {
                    result.push(DefaultMovement::from(Movement {
                        piece: *piece,
                        from: pos.clone(),
                        to: curr_pos,
                    }));
                }
            }
        }
        for curr_pos in capture_base {
            if let Some(curr_pos) = curr_pos {
                if let Some(curr_piece) = board.get(&curr_pos) {
                    if curr_piece.color != piece.color {
                        result.push(DefaultMovement::from(Movement {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        }));
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
        board::pos::Pos,
        game::{board, mode::standard_chess, movement::movement::DefaultMovement, piece},
        movement::Movement,
    };

    use super::movements;

    #[test]
    fn movements_empty_board() {
        let bounds = standard_chess().bounds;
        assert_eq!(movements(&board::empty(), &bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_white_pawn() {
        let board = HashMap::from([piece::of_str("C5", "♙")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("C5")),
            [DefaultMovement::from(Movement::of_str("♙", "C5", "C6"))]
        );
    }

    #[test]
    fn movements_lonely_black_pawn() {
        let board = HashMap::from([piece::of_str("C5", "♟")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("C5")),
            [DefaultMovement::from(Movement::of_str("♟", "C5", "C4"))]
        );
    }

    #[test]
    fn movements_first_move_white_pawn() {
        let board = HashMap::from([piece::of_str("A2", "♙")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("A2")),
            [
                DefaultMovement::from(Movement::of_str("♙", "A2", "A3")),
                DefaultMovement::from(Movement::of_str("♙", "A2", "A4"))
            ]
        );
    }

    #[test]
    fn movements_first_move_black_pawn() {
        let board = HashMap::from([piece::of_str("H7", "♟")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("H7")),
            [
                DefaultMovement::from(Movement::of_str("♟", "H7", "H6")),
                DefaultMovement::from(Movement::of_str("♟", "H7", "H5"))
            ]
        );
    }

    #[test]
    fn movements_blocked_white_pawn() {
        let board = HashMap::from([piece::of_str("C5", "♙"), piece::of_str("C6", "♟")]);
        let bounds = standard_chess().bounds;
        assert_eq!(movements(&board, &bounds, &Pos::of_str("C5")), []);
    }

    #[test]
    fn movements_blocked_black_pawn() {
        let board = HashMap::from([piece::of_str("C5", "♟"), piece::of_str("C4", "♙")]);
        let bounds = standard_chess().bounds;
        assert_eq!(movements(&board, &bounds, &Pos::of_str("C5")), []);
    }

    #[test]
    fn movements_white_capture() {
        let board = board::of_str([
            "        ",
            "        ",
            " ♟ ♙    ",
            "  ♙     ",
            "        ",
            "        ",
            "        ",
            "        ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("C5")),
            [
                DefaultMovement::from(Movement::of_str("♙", "C5", "C6")),
                DefaultMovement::from(Movement::of_str("♙", "C5", "B6"))
            ]
        );
    }

    #[test]
    fn movements_black_capture() {
        let board = board::of_str([
            "        ",
            "        ",
            "        ",
            "  ♟     ",
            " ♟ ♙    ",
            "        ",
            "        ",
            "        ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("C5")),
            [
                DefaultMovement::from(Movement::of_str("♟", "C5", "C4")),
                DefaultMovement::from(Movement::of_str("♟", "C5", "D4"))
            ]
        );
    }
}
