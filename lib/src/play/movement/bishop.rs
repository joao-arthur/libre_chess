use crate::{board::pos::Pos, play::Play};

fn bishop_move(play: &Play, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let modifiers: [[i8; 2]; 4] = [[-1, 1], [1, 1], [1, -1], [-1, -1]];
    for modifier in modifiers {
        let mut rel_row: i8 = 0;
        let mut rel_col: i8 = 0;
        loop {
            rel_row += modifier[0];
            rel_col += modifier[1];
            if let Some(pos) = pos.try_of_rel_idx(rel_row, rel_col) {
                result.push(pos.clone());
                if play.board[pos].is_some() {
                    break;
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
    use crate::board::Board;

    use super::*;

    #[test]
    fn test_bishop_move() {
        assert_eq!(
            bishop_move(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "  ♝     ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("C5")
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
        assert_eq!(
            bishop_move(
                &Play {
                    board: Board::of_str([
                        "♝       ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("A8")
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
        assert_eq!(
            bishop_move(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "♝       ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("A1")
            ),
            [
                Pos::of_str("B2"),
                Pos::of_str("C3"),
                Pos::of_str("D4"),
                Pos::of_str("E5"),
                Pos::of_str("F6"),
                Pos::of_str("G7"),
                Pos::of_str("H8"),
            ]
        );
        assert_eq!(
            bishop_move(
                &Play {
                    board: Board::of_str([
                        "       ♝",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("H8")
            ),
            [
                Pos::of_str("G7"),
                Pos::of_str("F6"),
                Pos::of_str("E5"),
                Pos::of_str("D4"),
                Pos::of_str("C3"),
                Pos::of_str("B2"),
                Pos::of_str("A1"),
            ]
        );
        assert_eq!(
            bishop_move(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "       ♝",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("H1")
            ),
            [
                Pos::of_str("G2"),
                Pos::of_str("F3"),
                Pos::of_str("E4"),
                Pos::of_str("D5"),
                Pos::of_str("C6"),
                Pos::of_str("B7"),
                Pos::of_str("A8"),
            ]
        );
    }
    #[test]
    fn test_bishop_move_other_pieces() {
        assert_eq!(
            bishop_move(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "   ♖    ",
                        "  ♝     ",
                        "        ",
                        "♖   ♖   ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("C5")
            ),
            [
                Pos::of_str("D6"),
                //
                Pos::of_str("D4"),
                Pos::of_str("E3"),
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
