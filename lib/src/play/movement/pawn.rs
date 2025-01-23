use crate::{
    board::{pos::Pos, row::Row},
    piece::{Color, Piece},
    play::Play,
};

fn pawn_movements(play: &Play, pos: &Pos, piece: &Piece) -> Vec<Pos> {
    match piece.c {
        Color::White => white_pawn_movements(play, pos, piece),
        Color::Black => black_pawn_movements(play, pos, piece),
    }
}

fn white_pawn_movements(play: &Play, pos: &Pos, piece: &Piece) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let base = if pos.row == Row::_2 {
        vec![pos.try_of_rel_idx(-1, 0), pos.try_of_rel_idx(-2, 0)]
    } else {
        vec![pos.try_of_rel_idx(-1, 0)]
    };
    for curr_pos in base {
        if let Some(curr_pos) = curr_pos {
            if play.board[curr_pos.clone()].is_none() {
                result.push(curr_pos);
            }
        }
    }
    let capture_base = [pos.try_of_rel_idx(-1, -1), pos.try_of_rel_idx(-1, 1)];
    for curr_pos in capture_base {
        if let Some(curr_pos) = curr_pos {
            if let Some(curr_piece) = play.board[curr_pos.clone()] {
                if curr_piece.c != piece.c {
                    result.push(curr_pos);
                }
            }
        }
    }
    result
}

fn black_pawn_movements(play: &Play, pos: &Pos, piece: &Piece) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let base = if pos.row == Row::_7 {
        vec![pos.try_of_rel_idx(1, 0), pos.try_of_rel_idx(2, 0)]
    } else {
        vec![pos.try_of_rel_idx(1, 0)]
    };
    for curr_pos in base {
        if let Some(curr_pos) = curr_pos {
            if play.board[curr_pos.clone()].is_none() {
                result.push(curr_pos);
            }
        }
    }
    let capture_base = [pos.try_of_rel_idx(1, -1), pos.try_of_rel_idx(1, 1)];
    for curr_pos in capture_base {
        if let Some(curr_pos) = curr_pos {
            if let Some(curr_piece) = play.board[curr_pos.clone()] {
                if curr_piece.c != piece.c {
                    result.push(curr_pos);
                }
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
    fn test_pawn_movements_empty_board() {
        assert_eq!(
            pawn_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "  ♙     ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("C5"),
                &Piece::of_str("♙")
            ),
            [Pos::of_str("C6")]
        );
        assert_eq!(
            pawn_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "  ♟     ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("C5"),
                &Piece::of_str("♟")
            ),
            [Pos::of_str("C4")]
        );
    }

    #[test]
    fn test_pawn_movements_first_move() {
        assert_eq!(
            pawn_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "♙       ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("A2"),
                &Piece::of_str("♙")
            ),
            [Pos::of_str("A3"), Pos::of_str("A4")]
        );
        assert_eq!(
            pawn_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "       ♟",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("H7"),
                &Piece::of_str("♟")
            ),
            [Pos::of_str("H6"), Pos::of_str("H5")]
        );
    }

    #[test]
    fn test_pawn_movements_blocked() {
        assert_eq!(
            pawn_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "  ♟     ",
                        "  ♙     ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("C5"),
                &Piece::of_str("♙")
            ),
            []
        );
        assert_eq!(
            pawn_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "  ♟     ",
                        "  ♙     ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("C5"),
                &Piece::of_str("♟")
            ),
            []
        );
    }

    #[test]
    fn test_pawn_movements_capture() {
        assert_eq!(
            pawn_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        " ♟ ♟    ",
                        "  ♙     ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("C5"),
                &Piece::of_str("♙")
            ),
            [Pos::of_str("C6"), Pos::of_str("B6"), Pos::of_str("D6")]
        );
        assert_eq!(
            pawn_movements(
                &Play {
                    board: Board::of_str([
                        "        ",
                        "        ",
                        "        ",
                        "  ♟     ",
                        " ♙ ♙    ",
                        "        ",
                        "        ",
                        "        ",
                    ]),
                    ..Default::default()
                },
                &Pos::of_str("C5"),
                &Piece::of_str("♟")
            ),
            [Pos::of_str("C4"), Pos::of_str("B4"), Pos::of_str("D4")]
        );
    }
}
