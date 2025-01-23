use crate::{board::pos::Pos, piece::Piece, play::Play};

pub fn king_movements(play: &Play, pos: &Pos, piece: &Piece) -> Vec<Pos> {
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
            if let Some(curr_piece) = play.board[curr_pos.clone()] {
                if curr_piece.c != piece.c {
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

    #[cfg(test)]
    mod test {
        use crate::board::Board;

        use super::*;

        #[test]
        fn test_king_movements_empty_board() {
            assert_eq!(
                king_movements(
                    &Play {
                        board: Board::of_str([
                            "        ",
                            "        ",
                            "        ",
                            "        ",
                            "   ♚    ",
                            "        ",
                            "        ",
                            "        ",
                        ]),
                        ..Default::default()
                    },
                    &Pos::of_str("D4"),
                    &Piece::of_str("♚")
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
        fn test_king_movements_edge() {
            assert_eq!(
                king_movements(
                    &Play {
                        board: Board::of_str([
                            "        ",
                            "        ",
                            "        ",
                            "        ",
                            "        ",
                            "        ",
                            "        ",
                            "♚       ",
                        ]),
                        ..Default::default()
                    },
                    &Pos::of_str("A1"),
                    &Piece::of_str("♚")
                ),
                [Pos::of_str("A2"), Pos::of_str("B2"), Pos::of_str("B1")]
            );
        }

        #[test]
        fn test_king_movements_with_capture() {
            assert_eq!(
                king_movements(
                    &Play {
                        board: Board::of_str([
                            "        ",
                            "        ",
                            "        ",
                            "    ♖   ",
                            "  ♗♚    ",
                            "        ",
                            "        ",
                            "        ",
                        ]),
                        ..Default::default()
                    },
                    &Pos::of_str("D4"),
                    &Piece::of_str("♚")
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
}
