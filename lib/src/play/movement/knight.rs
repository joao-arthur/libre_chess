use crate::{
    board::{pos::Pos, Board},
    piece::Color,
};

pub fn naive_movements_knight(board: &Board, pos: &Pos, color: &Color) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let base = [
        pos.try_of_rel_idx(-2, -1),
        pos.try_of_rel_idx(-1, -2),
        pos.try_of_rel_idx(1, -2),
        pos.try_of_rel_idx(2, -1),
        pos.try_of_rel_idx(-2, 1),
        pos.try_of_rel_idx(-1, 2),
        pos.try_of_rel_idx(1, 2),
        pos.try_of_rel_idx(2, 1),
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
    use crate::board::Board;

    use super::*;

    #[test]
    fn test_naive_movements_knight_empty_board() {
        assert_eq!(
            naive_movements_knight(
                &Board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "   ♞    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
                &Color::Black
            ),
            [
                Pos::of_str("C6"),
                Pos::of_str("B5"),
                Pos::of_str("B3"),
                Pos::of_str("C2"),
                Pos::of_str("E6"),
                Pos::of_str("F5"),
                Pos::of_str("F3"),
                Pos::of_str("E2"),
            ]
        );
    }

    #[test]
    fn test_naive_movements_knight_edge() {
        assert_eq!(
            naive_movements_knight(
                &Board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "♞       ",
                ]),
                &Pos::of_str("A1"),
                &Color::Black
            ),
            [Pos::of_str("B3"), Pos::of_str("C2")]
        );
    }

    #[test]
    fn test_naive_movements_knight_with_capture() {
        assert_eq!(
            naive_movements_knight(
                &Board::of_str([
                    "        ",
                    "        ",
                    "    ♖   ",
                    "     ♖  ",
                    "   ♞    ",
                    " ♜      ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
                &Color::Black
            ),
            [
                Pos::of_str("C6"),
                Pos::of_str("B5"),
                Pos::of_str("C2"),
                Pos::of_str("E6"),
                Pos::of_str("F5"),
                Pos::of_str("F3"),
                Pos::of_str("E2"),
            ]
        );
        assert_eq!(
            naive_movements_knight(
                &Board::of_str([
                    "        ",
                    "        ",
                    "    ♜   ",
                    "     ♜  ",
                    "   ♘    ",
                    " ♖      ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
                &Color::White
            ),
            [
                Pos::of_str("C6"),
                Pos::of_str("B5"),
                Pos::of_str("C2"),
                Pos::of_str("E6"),
                Pos::of_str("F5"),
                Pos::of_str("F3"),
                Pos::of_str("E2"),
            ]
        );
    }
}
