use crate::{
    board::{pos::Pos, Board},
    color::Color,
};

pub fn naive_movements_bishop(board: &Board, pos: &Pos, color: &Color) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let modifiers: [[i8; 2]; 4] = [[-1, 1], [1, 1], [1, -1], [-1, -1]];
    for modifier in modifiers {
        let mut rel_row: i8 = 0;
        let mut rel_col: i8 = 0;
        loop {
            rel_row += modifier[0];
            rel_col += modifier[1];
            if let Some(curr_pos) = pos.try_of_rel_idx(rel_row, rel_col) {
                if let Some(curr_piece) = board.get(&curr_pos) {
                    if &curr_piece.c == color {
                        break;
                    } else {
                        result.push(curr_pos);
                        break;
                    }
                } else {
                    result.push(curr_pos);
                }
            } else {
                break;
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
    fn test_naive_movements_bishop_empty_board() {
        assert_eq!(
            naive_movements_bishop(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♝     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
                &Color::Black
            ),
            [
                Pos::of_str("D6"),
                Pos::of_str("E7"),
                Pos::of_str("F8"),
                //
                Pos::of_str("D4"),
                Pos::of_str("E3"),
                Pos::of_str("F2"),
                Pos::of_str("G1"),
                //
                Pos::of_str("B4"),
                Pos::of_str("A3"),
                //
                Pos::of_str("B6"),
                Pos::of_str("A7"),
            ]
        );
    }

    #[test]
    fn test_naive_movements_bishop_edge() {
        assert_eq!(
            naive_movements_bishop(
                &board::of_str([
                    "♝       ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("A8"),
                &Color::Black
            ),
            [
                Pos::of_str("B7"),
                Pos::of_str("C6"),
                Pos::of_str("D5"),
                Pos::of_str("E4"),
                Pos::of_str("F3"),
                Pos::of_str("G2"),
                Pos::of_str("H1"),
            ]
        );
    }

    #[test]
    fn test_naive_movements_bishop_with_capture() {
        assert_eq!(
            naive_movements_bishop(
                &board::of_str([
                    "        ",
                    "        ",
                    "   ♖    ",
                    "  ♝     ",
                    "        ",
                    "♖   ♜   ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
                &Color::Black
            ),
            [
                Pos::of_str("D6"),
                //
                Pos::of_str("D4"),
                //
                Pos::of_str("B4"),
                Pos::of_str("A3"),
                //
                Pos::of_str("B6"),
                Pos::of_str("A7"),
            ]
        );
        assert_eq!(
            naive_movements_bishop(
                &board::of_str([
                    "        ",
                    "        ",
                    "   ♜    ",
                    "  ♗     ",
                    "        ",
                    "♜   ♖   ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
                &Color::White
            ),
            [
                Pos::of_str("D6"),
                //
                Pos::of_str("D4"),
                //
                Pos::of_str("B4"),
                Pos::of_str("A3"),
                //
                Pos::of_str("B6"),
                Pos::of_str("A7"),
            ]
        );
    }
}
