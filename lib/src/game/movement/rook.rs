use crate::{board::pos::Pos, game::board::Board};

pub fn naive_movements_rook(board: &Board, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(&pos) {
        let modifiers: [[i8; 2]; 4] = [[0, 1], [-1, 0], [0, -1], [1, 0]];
        for modifier in modifiers {
            let mut rel_row: i8 = 0;
            let mut rel_col: i8 = 0;
            loop {
                rel_row += modifier[0];
                rel_col += modifier[1];
                if let Some(curr_pos) = pos.try_of_rel_idx(rel_row, rel_col) {
                    // fix
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
    use crate::{board::pos::Pos, game::board};

    use super::naive_movements_rook;

    #[test]
    fn naive_movements_rook_empty_board() {
        assert_eq!(naive_movements_rook(&board::empty(), &Pos::of_str("A1")), []);
    }

    #[test]
    fn naive_movements_rook_lonely_piece() {
        assert_eq!(
            naive_movements_rook(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "   ♜    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                Pos::of_str("E4"),
                Pos::of_str("F4"),
                Pos::of_str("G4"),
                Pos::of_str("H4"),
                //
                Pos::of_str("D3"),
                Pos::of_str("D2"),
                Pos::of_str("D1"),
                //
                Pos::of_str("C4"),
                Pos::of_str("B4"),
                Pos::of_str("A4"),
                //
                Pos::of_str("D5"),
                Pos::of_str("D6"),
                Pos::of_str("D7"),
                Pos::of_str("D8"),
            ]
        );
    }

    #[test]
    fn naive_movements_rook_edge() {
        assert_eq!(
            naive_movements_rook(
                &board::of_str([
                    "♜       ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("A8"),
            ),
            [
                Pos::of_str("B8"),
                Pos::of_str("C8"),
                Pos::of_str("D8"),
                Pos::of_str("E8"),
                Pos::of_str("F8"),
                Pos::of_str("G8"),
                Pos::of_str("H8"),
                //
                Pos::of_str("A7"),
                Pos::of_str("A6"),
                Pos::of_str("A5"),
                Pos::of_str("A4"),
                Pos::of_str("A3"),
                Pos::of_str("A2"),
                Pos::of_str("A1"),
            ]
        );
    }

    #[test]
    fn naive_movements_rook_with_capture() {
        assert_eq!(
            naive_movements_rook(
                &board::of_str([
                    "        ",
                    "   ♗    ",
                    "        ",
                    "        ",
                    "   ♜  ♝ ",
                    "        ",
                    "        ",
                    "   ♗    ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                Pos::of_str("E4"),
                Pos::of_str("F4"),
                //
                Pos::of_str("D3"),
                Pos::of_str("D2"),
                Pos::of_str("D1"),
                //
                Pos::of_str("C4"),
                Pos::of_str("B4"),
                Pos::of_str("A4"),
                //
                Pos::of_str("D5"),
                Pos::of_str("D6"),
                Pos::of_str("D7"),
            ]
        );
        assert_eq!(
            naive_movements_rook(
                &board::of_str([
                    "        ",
                    "   ♝    ",
                    "        ",
                    "        ",
                    "   ♖  ♗ ",
                    "        ",
                    "        ",
                    "   ♝    ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                Pos::of_str("E4"),
                Pos::of_str("F4"),
                //
                Pos::of_str("D3"),
                Pos::of_str("D2"),
                Pos::of_str("D1"),
                //
                Pos::of_str("C4"),
                Pos::of_str("B4"),
                Pos::of_str("A4"),
                //
                Pos::of_str("D5"),
                Pos::of_str("D6"),
                Pos::of_str("D7"),
            ]
        );
    }
}
