use crate::{board::pos::Pos, play::Play};

fn rook_move(play: &Play, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let modifiers: [[i8; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
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

mod test {
    use crate::board::Board;

    use super::*;

    #[test]
    fn test_rook_move() {
        assert_eq!(
            rook_move(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "   ♜    ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("D4")
            ),
            [
                Pos::of_str("D5"),
                Pos::of_str("D6"),
                Pos::of_str("D7"),
                Pos::of_str("D8"),
                //
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
            ]
        );
        assert_eq!(
            rook_move(
                &Play {
                    board: Board::of_str([
                        "♜       ",
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
        assert_eq!(
            rook_move(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "♜       ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("A1")
            ),
            [
                Pos::of_str("A2"),
                Pos::of_str("A3"),
                Pos::of_str("A4"),
                Pos::of_str("A5"),
                Pos::of_str("A6"),
                Pos::of_str("A7"),
                Pos::of_str("A8"),
                //
                Pos::of_str("B1"),
                Pos::of_str("C1"),
                Pos::of_str("D1"),
                Pos::of_str("E1"),
                Pos::of_str("F1"),
                Pos::of_str("G1"),
                Pos::of_str("H1"),
            ]
        );
        assert_eq!(
            rook_move(
                &Play {
                    board: Board::of_str([
                        "       ♜",
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
                Pos::of_str("H7"),
                Pos::of_str("H6"),
                Pos::of_str("H5"),
                Pos::of_str("H4"),
                Pos::of_str("H3"),
                Pos::of_str("H2"),
                Pos::of_str("H1"),
                //
                Pos::of_str("G8"),
                Pos::of_str("F8"),
                Pos::of_str("E8"),
                Pos::of_str("D8"),
                Pos::of_str("C8"),
                Pos::of_str("B8"),
                Pos::of_str("A8"),
            ]
        );
        assert_eq!(
            rook_move(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "       ♜",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("H1")
            ),
            [
                Pos::of_str("H2"),
                Pos::of_str("H3"),
                Pos::of_str("H4"),
                Pos::of_str("H5"),
                Pos::of_str("H6"),
                Pos::of_str("H7"),
                Pos::of_str("H8"),
                //
                Pos::of_str("G1"),
                Pos::of_str("F1"),
                Pos::of_str("E1"),
                Pos::of_str("D1"),
                Pos::of_str("C1"),
                Pos::of_str("B1"),
                Pos::of_str("A1"),
            ]
        );
    }
}
