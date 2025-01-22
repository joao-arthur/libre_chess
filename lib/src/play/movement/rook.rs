use crate::{board::pos::Pos, piece::Piece, play::Play};

fn rook_movements(play: &Play, pos: &Pos, piece: &Piece) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let modifiers: [[i8; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    for modifier in modifiers {
        let mut rel_row: i8 = 0;
        let mut rel_col: i8 = 0;
        loop {
            rel_row += modifier[0];
            rel_col += modifier[1];
            if let Some(curr_pos) = pos.try_of_rel_idx(rel_row, rel_col) {
                if let Some(curr_piece) = play.board[curr_pos.clone()] {
                    if curr_piece.c == piece.c {
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
    use crate::board::Board;

    use super::*;

    #[test]
    fn test_rook_movements_empty_board() {
        assert_eq!(
            rook_movements(
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
                &Pos::of_str("D4"),
                &Piece::of_str("♜")
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
    }

    #[test]
    fn test_rook_movements_edge() {
        assert_eq!(
            rook_movements(
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
                &Pos::of_str("A8"),
                &Piece::of_str("♜")
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
    fn test_rook_movements_with_capture() {
        assert_eq!(
            rook_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "   ♗    ",
                        "        ",
                        "        ",
                        "   ♜  ♝ ",
                        "        ",
                        "        ",
                        "   ♗    ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("D4"),
                &Piece::of_str("♜")
            ),
            [
                Pos::of_str("D5"),
                Pos::of_str("D6"),
                Pos::of_str("D7"),
                //
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
            ]
        );
        assert_eq!(
            rook_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "   ♝    ",
                        "        ",
                        "        ",
                        "   ♖  ♗ ",
                        "        ",
                        "        ",
                        "   ♝    ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("D4"),
                &Piece::of_str("♖")
            ),
            [
                Pos::of_str("D5"),
                Pos::of_str("D6"),
                Pos::of_str("D7"),
                //
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
            ]
        );
    }
}
