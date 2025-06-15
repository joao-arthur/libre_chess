use crate::{board::pos::Pos, game::board::Board};

pub fn movements(board: &Board, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(&pos) {
        let base = [
            pos.try_of_rel_idx(1, 1),
            pos.try_of_rel_idx(0, 1),
            pos.try_of_rel_idx(-1, 1),
            pos.try_of_rel_idx(-1, 0),
            pos.try_of_rel_idx(-1, -1),
            pos.try_of_rel_idx(0, -1),
            pos.try_of_rel_idx(1, -1),
            pos.try_of_rel_idx(1, 0),
        ];
        for curr_pos in base {
            if let Some(curr_pos) = curr_pos {
                if curr_pos.col > 7 || curr_pos.row > 7 {
                    continue;
                }
                if let Some(curr_piece) = board.get(&curr_pos) {
                    if &curr_piece.color != &piece.color {
                        result.push(curr_pos);
                    }
                } else {
                    result.push(curr_pos);
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
    fn movements_empty_board() {
        assert_eq!(movements(&board::empty(), &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_piece() {
        assert_eq!(
            movements(&HashMap::from([piece::of_str("D4", "♚")]), &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "E3", "D3", "C3", "C4", "C5", "D5"])
        );
    }

    #[test]
    fn movements_edge() {
        assert_eq!(
            movements(&HashMap::from([piece::of_str("H8", "♚")]), &Pos::of_str("H8")),
            pos_of_str_slice(["H7", "G7", "G8"])
        );
        assert_eq!(
            movements(&HashMap::from([piece::of_str("H1", "♚")]), &Pos::of_str("H1")),
            pos_of_str_slice(["G1", "G2", "H2"])
        );
        assert_eq!(
            movements(&HashMap::from([piece::of_str("A1", "♚")]), &Pos::of_str("A1")),
            pos_of_str_slice(["B2", "B1", "A2"])
        );
        assert_eq!(
            movements(&HashMap::from([piece::of_str("A8", "♚")]), &Pos::of_str("A8")),
            pos_of_str_slice(["B8", "B7", "A7"])
        );
    }

    #[test]
    fn movements_with_capture() {
        assert_eq!(
            movements(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "    ♖   ",
                    "  ♗♚    ",
                    "   ♜    ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            pos_of_str_slice(["E5", "E4", "E3", "C3", "C4", "C5", "D5"])
        );
        assert_eq!(
            movements(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "    ♜   ",
                    "  ♝♔    ",
                    "   ♖    ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            pos_of_str_slice(["E5", "E4", "E3", "C3", "C4", "C5", "D5"])
        );
    }
}
