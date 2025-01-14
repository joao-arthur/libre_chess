use std::fmt;

use crate::piece::{self, Piece};

use super::{board_pos::BoardPos, board_x, board_y};

pub type Board = [[Option<Piece>; 8]; 8];

#[derive(Debug, PartialEq)]
pub struct InvalidCharacterErr;

impl fmt::Display for InvalidCharacterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Only [0-9] characters and spaces are allowed!")
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidLengthErr;

impl fmt::Display for InvalidLengthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Every line must be 8 characters long")
    }
}

#[derive(Debug, PartialEq)]
pub enum FromStringErr {
    InvalidCharacter(InvalidCharacterErr),
    InvalidLength(InvalidLengthErr),
}

pub fn of_str(rows: [&str; 8]) -> Result<Board, FromStringErr> {
    if !rows
        .join("")
        .replace("♖", "")
        .replace("♘", "")
        .replace("♗", "")
        .replace("♕", "")
        .replace("♔", "")
        .replace("♙", "")
        .replace("♜", "")
        .replace("♞", "")
        .replace("♝", "")
        .replace("♛", "")
        .replace("♚", "")
        .replace("♟", "")
        .replace(" ", "")
        .is_empty()
    {
        return Err(FromStringErr::InvalidCharacter(InvalidCharacterErr));
    }
    for line in rows {
        if line.chars().count() != 8 {
            return Err(FromStringErr::InvalidLength(InvalidLengthErr));
        }
    }
    let mut res: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    for row in 0..8 {
        for col in 0..8 {
            res[row as usize][col as usize] =
                piece::of_str(&rows[row as usize].chars().nth(col).unwrap().to_string());
        }
    }
    Ok(res)
}

fn get_default_board() -> Board {
    of_str([
        "♜♞♝♛♚♝♞♜",
        "♟♟♟♟♟♟♟♟",
        "        ",
        "        ",
        "        ",
        "        ",
        "♙♙♙♙♙♙♙♙",
        "♖♘♗♕♔♗♘♖",
    ])
    .unwrap()
}

fn board_to_string(board: &Board) -> String {
    let mut res: String = String::from("");
    for row in board {
        for col in row {
            match col {
                Some(p) => res.push_str(piece::to_str(&p)),
                None => res.push_str(" "),
            }
        }
        res.push_str("\n")
    }
    res
}

fn get_board_piece(board: &Board, board_pos: &BoardPos) -> Option<Piece> {
    let x = board_x::to_idx(&board_pos.x);
    let y = board_y::to_idx(&board_pos.y);
    let x_idx: usize = (x).into();
    let y_idx: usize = (7 - y).into();
    board[y_idx][x_idx].clone()
}

#[cfg(test)]
mod test {
    use crate::board::board_pos;

    use super::*;

    #[test]
    fn test_board_to_string() {
        let brd = get_default_board();
        let res = board_to_string(&brd);
        assert_eq!(
            res.to_owned(),
            "".to_owned()
                + "♜♞♝♛♚♝♞♜\n"
                + "♟♟♟♟♟♟♟♟\n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "♙♙♙♙♙♙♙♙\n"
                + "♖♘♗♕♔♗♘♖\n"
        );
    }

    #[test]
    fn test_get_board_position() {
        let brd = get_default_board();
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("A8").unwrap()), piece::of_str("♜"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("B8").unwrap()), piece::of_str("♞"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("C8").unwrap()), piece::of_str("♝"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("D8").unwrap()), piece::of_str("♛"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("E8").unwrap()), piece::of_str("♚"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("F8").unwrap()), piece::of_str("♝"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("G8").unwrap()), piece::of_str("♞"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("H8").unwrap()), piece::of_str("♜"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("H8").unwrap()), piece::of_str("♜"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("H7").unwrap()), piece::of_str("♟"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("H6").unwrap()), piece::of_str(" "));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("H5").unwrap()), piece::of_str(" "));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("H4").unwrap()), piece::of_str(" "));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("H3").unwrap()), piece::of_str(" "));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("H2").unwrap()), piece::of_str("♙"));
        assert_eq!(get_board_piece(&brd, &board_pos::of_str("H1").unwrap()), piece::of_str("♖"));
    }

    #[test]
    fn test_of_str() {
        assert_eq!(
            of_str([
                "♜♞♝♛♚♝♞♜",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖♘♗♕♔♗♘♖",
            ]),
            Ok([
                [
                    piece::of_str("♜"),
                    piece::of_str("♞"),
                    piece::of_str("♝"),
                    piece::of_str("♛"),
                    piece::of_str("♚"),
                    piece::of_str("♝"),
                    piece::of_str("♞"),
                    piece::of_str("♜"),
                ],
                [
                    piece::of_str("♟"),
                    piece::of_str("♟"),
                    piece::of_str("♟"),
                    piece::of_str("♟"),
                    piece::of_str("♟"),
                    piece::of_str("♟"),
                    piece::of_str("♟"),
                    piece::of_str("♟"),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    piece::of_str("♙"),
                    piece::of_str("♙"),
                    piece::of_str("♙"),
                    piece::of_str("♙"),
                    piece::of_str("♙"),
                    piece::of_str("♙"),
                    piece::of_str("♙"),
                    piece::of_str("♙"),
                ],
                [
                    piece::of_str("♖"),
                    piece::of_str("♘"),
                    piece::of_str("♗"),
                    piece::of_str("♕"),
                    piece::of_str("♔"),
                    piece::of_str("♗"),
                    piece::of_str("♘"),
                    piece::of_str("♖"),
                ],
            ])
        );
    }
}
