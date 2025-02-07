use crate::board::{pos::Pos, Board};

pub fn naive_movements_king(board: &Board, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(&pos) {
        let base = [
            pos.try_of_rel_idx(-1, -1),
            pos.try_of_rel_idx(0, -1),
            pos.try_of_rel_idx(1, -1),
            pos.try_of_rel_idx(-1, 0),
            pos.try_of_rel_idx(1, 0),
            pos.try_of_rel_idx(-1, 1),
            pos.try_of_rel_idx(0, 1),
            pos.try_of_rel_idx(1, 1),
        ];
        for curr_pos in base {
            if let Some(curr_pos) = curr_pos {
                if let Some(curr_piece) = board.get(&curr_pos) {
                    if &curr_piece.c != &piece.c {
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
mod test {
    use crate::board;

    use super::*;

    #[test]
    fn test_naive_movements_king_none() {
        assert_eq!(
            naive_movements_king(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "   ♚    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("A1"),
            ),
            []
        );
    }

    #[test]
    fn test_naive_movements_king_empty_board() {
        assert_eq!(
            naive_movements_king(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "   ♚    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                Pos::of_str("C5"),
                Pos::of_str("C4"),
                Pos::of_str("C3"),
                Pos::of_str("D5"),
                Pos::of_str("D3"),
                Pos::of_str("E5"),
                Pos::of_str("E4"),
                Pos::of_str("E3"),
            ]
        );
    }

    #[test]
    fn test_naive_movements_king_edge() {
        assert_eq!(
            naive_movements_king(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "♚       ",
                ]),
                &Pos::of_str("A1"),
            ),
            [Pos::of_str("A2"), Pos::of_str("B2"), Pos::of_str("B1")]
        );
    }

    #[test]
    fn test_naive_movements_king_with_capture() {
        assert_eq!(
            naive_movements_king(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "    ♖   ",
                    "  ♗♚    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                Pos::of_str("C5"),
                Pos::of_str("C4"),
                Pos::of_str("C3"),
                Pos::of_str("D5"),
                Pos::of_str("D3"),
                Pos::of_str("E5"),
                Pos::of_str("E4"),
                Pos::of_str("E3"),
            ]
        );
    }
}
