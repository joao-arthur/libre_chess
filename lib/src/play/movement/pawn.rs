use crate::{
    board::{pos::Pos, row::Row, Board},
    piece::{Color, Piece},
};

use super::Movement;

fn naive_movements_pawn(board: &Board, pos: &Pos, color: &Color) -> Vec<Pos> {
    match color {
        Color::White => naive_movements_white_pawn(board, pos, color),
        Color::Black => naive_movements_black_pawn(board, pos, color),
    }
}

fn naive_movements_white_pawn(board: &Board, pos: &Pos, color: &Color) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let base = if pos.row == Row::_2 {
        vec![pos.try_of_rel_idx(-1, 0), pos.try_of_rel_idx(-2, 0)]
    } else {
        vec![pos.try_of_rel_idx(-1, 0)]
    };
    for curr_pos in base {
        if let Some(curr_pos) = curr_pos {
            if board[curr_pos.clone()].is_none() {
                result.push(curr_pos);
            }
        }
    }
    let capture_base = [pos.try_of_rel_idx(-1, -1), pos.try_of_rel_idx(-1, 1)];
    for curr_pos in capture_base {
        if let Some(curr_pos) = curr_pos {
            if let Some(curr_piece) = board[curr_pos.clone()] {
                if &curr_piece.c != color {
                    result.push(curr_pos);
                }
            }
        }
    }
    result
}

fn naive_movements_black_pawn(board: &Board, pos: &Pos, color: &Color) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let base = if pos.row == Row::_7 {
        vec![pos.try_of_rel_idx(1, 0), pos.try_of_rel_idx(2, 0)]
    } else {
        vec![pos.try_of_rel_idx(1, 0)]
    };
    for curr_pos in base {
        if let Some(curr_pos) = curr_pos {
            if board[curr_pos.clone()].is_none() {
                result.push(curr_pos);
            }
        }
    }
    let capture_base = [pos.try_of_rel_idx(1, -1), pos.try_of_rel_idx(1, 1)];
    for curr_pos in capture_base {
        if let Some(curr_pos) = curr_pos {
            if let Some(curr_piece) = board[curr_pos.clone()] {
                if &curr_piece.c != color {
                    result.push(curr_pos);
                }
            }
        }
    }
    result
}

fn white_pawn_en_passant(board: &Board, history: Vec<Movement>, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if pos.row == Row::_5 {
        if let Some(mov) = history.last() {
            if mov.piece == Piece::of_str("♟") {
                if Some(mov.from.clone()) == pos.try_of_rel_idx(-2, -1)
                    && Some(mov.to.clone()) == pos.try_of_rel_idx(0, -1)
                {
                    if let Some(capture_pos) = pos.try_of_rel_idx(-1, -1) {
                        result.push(capture_pos);
                    }
                }
                if Some(mov.from.clone()) == pos.try_of_rel_idx(-2, 1)
                    && Some(mov.to.clone()) == pos.try_of_rel_idx(0, 1)
                {
                    if let Some(capture_pos) = pos.try_of_rel_idx(-1, 1) {
                        result.push(capture_pos);
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pawn_movements_empty_board() {
        assert_eq!(
            naive_movements_pawn(
                &Board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
                &Color::White
            ),
            [Pos::of_str("C6")]
        );
        assert_eq!(
            naive_movements_pawn(
                &Board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♟     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
                &Color::Black
            ),
            [Pos::of_str("C4")]
        );
    }

    #[test]
    fn test_pawn_movements_first_move() {
        assert_eq!(
            naive_movements_pawn(
                &Board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "♙       ",
                    "        ",
                ]),
                &Pos::of_str("A2"),
                &Color::White
            ),
            [Pos::of_str("A3"), Pos::of_str("A4")]
        );
        assert_eq!(
            naive_movements_pawn(
                &Board::of_str([
                    "        ",
                    "       ♟",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("H7"),
                &Color::Black
            ),
            [Pos::of_str("H6"), Pos::of_str("H5")]
        );
    }

    #[test]
    fn test_pawn_movements_blocked() {
        assert_eq!(
            naive_movements_pawn(
                &Board::of_str([
                    "        ",
                    "        ",
                    "  ♟     ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
                &Color::White
            ),
            []
        );
        assert_eq!(
            naive_movements_pawn(
                &Board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♟     ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
                &Color::Black
            ),
            []
        );
    }

    #[test]
    fn test_pawn_movements_capture() {
        assert_eq!(
            naive_movements_pawn(
                &Board::of_str([
                    "        ",
                    "        ",
                    " ♟ ♟    ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
                &Color::White
            ),
            [Pos::of_str("C6"), Pos::of_str("B6"), Pos::of_str("D6")]
        );
        assert_eq!(
            naive_movements_pawn(
                &Board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♟     ",
                    " ♙ ♙    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
                &Color::Black
            ),
            [Pos::of_str("C4"), Pos::of_str("B4"), Pos::of_str("D4")]
        );
    }

    #[test]
    fn test_white_pawn_en_passant() {
        assert_eq!(
            white_pawn_en_passant(
                &Board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    " ♟♙     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                Vec::from([Movement {
                    piece: Piece::of_str("♟"),
                    from: Pos::of_str("B7"),
                    to: Pos::of_str("B5"),
                }]),
                &Pos::of_str("C5"),
            ),
            [Pos::of_str("B6")]
        );
    }
}
