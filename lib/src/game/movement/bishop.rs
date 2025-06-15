use crate::{board::pos::Pos, game::board::Board};

pub fn naive_movements_bishop(board: &Board, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(&pos) {
        let modifiers: [[i8; 2]; 4] = [[1, 1], [-1, 1], [-1, -1], [1, -1]];
        for modifier in modifiers {
            let mut rel_row: i8 = 0;
            let mut rel_col: i8 = 0;
            loop {
                rel_row += modifier[0];
                rel_col += modifier[1];
                if let Some(curr_pos) = pos.try_of_rel_idx(rel_row, rel_col) {
                    if curr_pos.col > 7 || curr_pos.row > 7 {
                        break;
                    }
                    if let Some(curr_piece) = board.get(&curr_pos) {
                        if &curr_piece.color == &piece.color {
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

    use super::naive_movements_bishop;

    #[test]
    fn naive_movements_bishop_empty_board() {
        assert_eq!(naive_movements_bishop(&board::empty(), &Pos::of_str("A1")), []);
    }

    #[test]
    fn naive_movements_bishop_lonely_piece() {
        assert_eq!(
            naive_movements_bishop(&HashMap::from([piece::of_str("C5", "♝")]), &Pos::of_str("C5")),
            pos_of_str_slice(["D6", "E7", "F8", "D4", "E3", "F2", "G1", "B4", "A3", "B6", "A7"])
        );
    }

    #[test]
    fn naive_movements_bishop_edge() {
        assert_eq!(
            naive_movements_bishop(&HashMap::from([piece::of_str("H8", "♝")]), &Pos::of_str("H8")),
            pos_of_str_slice(["G7", "F6", "E5", "D4", "C3", "B2", "A1"])
        );
        assert_eq!(
            naive_movements_bishop(&HashMap::from([piece::of_str("H1", "♝")]), &Pos::of_str("H1")),
            pos_of_str_slice(["G2", "F3", "E4", "D5", "C6", "B7", "A8"])
        );
        assert_eq!(
            naive_movements_bishop(&HashMap::from([piece::of_str("A1", "♝")]), &Pos::of_str("A1")),
            pos_of_str_slice(["B2", "C3", "D4", "E5", "F6", "G7", "H8"])
        );
        assert_eq!(
            naive_movements_bishop(&HashMap::from([piece::of_str("A8", "♝")]), &Pos::of_str("A8")),
            pos_of_str_slice(["B7", "C6", "D5", "E4", "F3", "G2", "H1"])
        );
    }

    #[test]
    fn naive_movements_bishop_with_capture() {
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
            ),
            pos_of_str_slice(["D6", "D4", "B4", "A3", "B6", "A7"])
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
            ),
            pos_of_str_slice(["D6", "D4", "B4", "A3", "B6", "A7"])
        );
    }
}
