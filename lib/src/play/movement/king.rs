use crate::{
    board::{pos::Pos, Board},
    piece::Color,
};

pub fn naive_movements_king(board: &Board, pos: &Pos, color: &Color) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
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
            if let Some(curr_piece) = board[curr_pos.clone()] {
                if &curr_piece.c != color {
                    result.push(curr_pos);
                }
            } else {
                result.push(curr_pos);
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_naive_movements_king_empty_board() {
        assert_eq!(
            naive_movements_king(
                &Board::of_str([
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
                &Color::Black
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
                &Board::of_str([
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
                &Color::Black
            ),
            [Pos::of_str("A2"), Pos::of_str("B2"), Pos::of_str("B1")]
        );
    }

    #[test]
    fn test_naive_movements_king_with_capture() {
        assert_eq!(
            naive_movements_king(
                &Board::of_str([
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
                &Color::Black
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
