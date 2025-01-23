use crate::{board::pos::Pos, piece::Piece, play::Play};

pub fn knight_movements(play: &Play, pos: &Pos, piece: &Piece) -> Vec<Pos> {
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

    #[test]
    fn test_knight_movements_empty_board() {
        assert_eq!(
            knight_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "   ♞    ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("D4"),
                &Piece::of_str("♞")
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
    fn test_knight_movements_edge() {
        assert_eq!(
            knight_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "♞       ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("A1"),
                &Piece::of_str("♞")
            ),
            [Pos::of_str("B3"), Pos::of_str("C2")]
        );
    }

    #[test]
    fn test_knight_movements_with_capture() {
        assert_eq!(
            knight_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "    ♖   ",
                        "     ♖  ",
                        "   ♞    ",
                        " ♜      ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("D4"),
                &Piece::of_str("♞")
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
            knight_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "    ♜   ",
                        "     ♜  ",
                        "   ♘    ",
                        " ♖      ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("D4"),
                &Piece::of_str("♘")
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
