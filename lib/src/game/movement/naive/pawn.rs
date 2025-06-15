use crate::{board::pos::Pos, color::Color, game::board::Board, piece::Piece};

pub fn movements(board: &Board, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(&pos) {
        let base = match &piece.color {
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
        for curr_pos in base {
            if let Some(curr_pos) = curr_pos {
                if board.get(&curr_pos).is_none() {
                    result.push(curr_pos);
                }
            }
        }
        for curr_pos in capture_base {
            if let Some(curr_pos) = curr_pos {
                if let Some(curr_piece) = board.get(&curr_pos) {
                    if &curr_piece.color != &piece.color {
                        result.push(curr_pos);
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
        board::pos::{Pos, pos_of_str_slice},
        game::{board, piece},
    };

    use super::movements;

    #[test]
    fn pawn_movements_empty_board() {
        assert_eq!(movements(&board::empty(), &Pos::of_str("A1")), []);
    }

    #[test]
    fn pawn_movements_lonely_piece() {
        assert_eq!(
            movements(&HashMap::from([piece::of_str("C5", "♙")]), &Pos::of_str("C5")),
            [Pos::of_str("C6")]
        );
        assert_eq!(
            movements(&HashMap::from([piece::of_str("C5", "♟")]), &Pos::of_str("C5")),
            [Pos::of_str("C4")]
        );
    }

    #[test]
    fn pawn_movements_first_move() {
        assert_eq!(
            movements(&HashMap::from([piece::of_str("A2", "♙")]), &Pos::of_str("A2")),
            pos_of_str_slice(["A3", "A4"])
        );
        assert_eq!(
            movements(&HashMap::from([piece::of_str("H7", "♟")]), &Pos::of_str("H7")),
            pos_of_str_slice(["H6", "H5"])
        );
    }

    #[test]
    fn pawn_movements_blocked() {
        assert_eq!(
            movements(
                &HashMap::from([piece::of_str("C5", "♙"), piece::of_str("C6", "♟")]),
                &Pos::of_str("C5"),
            ),
            []
        );
        assert_eq!(
            movements(
                &HashMap::from([piece::of_str("C5", "♟"), piece::of_str("C4", "♙")]),
                &Pos::of_str("C5"),
            ),
            []
        );
    }

    #[test]
    fn pawn_movements_capture() {
        assert_eq!(
            movements(
                &board::of_str([
                    "        ",
                    "        ",
                    " ♟ ♟    ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            pos_of_str_slice(["C6", "B6", "D6"])
        );
        assert_eq!(
            movements(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♟     ",
                    " ♙ ♙    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            pos_of_str_slice(["C4", "B4", "D4"])
        );
    }
}
