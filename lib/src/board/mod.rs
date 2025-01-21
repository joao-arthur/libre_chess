use std::fmt;

use pos::Pos;

use crate::piece::Piece;

mod col;
mod row;
mod pos;


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

fn try_of_str(rows: [&str; 8]) -> Result<Board, FromStringErr> {
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
            res[row as usize][col as usize] = Piece::try_of_str(&rows[row as usize].chars().nth(col).unwrap().to_string());
        }
    }
    Ok(res)
}

pub fn of_str(rows: [&str; 8]) -> Board {
    try_of_str(rows).unwrap()
}

pub fn get_initial_white_board() -> Board {
    of_str([
        "♖♘♗♕♔♗♘♖",
        "♙♙♙♙♙♙♙♙",
        "        ",
        "        ",
        "        ",
        "        ",
        "♟♟♟♟♟♟♟♟",
        "♜♞♝♛♚♝♞♜",
    ])
}

pub fn get_initial_black_board() -> Board {
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
}

fn get_board_piece(b: &Board, pos: &Pos) -> Option<Piece> {
    b[pos.row.to_idx() as usize][pos.col.to_idx() as usize]
}

fn board_to_string(b: &Board) -> String {
    let mut res: String = String::from("");
    for row in b {
        for col in row {
            match col {
                Some(p) => res.push_str(Piece::to_str(&p)),
                None => res.push_str(" "),
            }
        }
        res.push_str("\n")
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            try_of_str([
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
                    Piece::try_of_str("♜"),
                    Piece::try_of_str("♞"),
                    Piece::try_of_str("♝"),
                    Piece::try_of_str("♛"),
                    Piece::try_of_str("♚"),
                    Piece::try_of_str("♝"),
                    Piece::try_of_str("♞"),
                    Piece::try_of_str("♜"),
                ],
                [
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                ],
                [
                    Piece::try_of_str("♖"),
                    Piece::try_of_str("♘"),
                    Piece::try_of_str("♗"),
                    Piece::try_of_str("♕"),
                    Piece::try_of_str("♔"),
                    Piece::try_of_str("♗"),
                    Piece::try_of_str("♘"),
                    Piece::try_of_str("♖"),
                ],
            ])
        );
    }

    #[test]
    fn test_get_board_piece() {
        let b = get_initial_black_board();
        assert_eq!(get_board_piece(&b, &Pos::of_str("A8")), Piece::try_of_str("♜"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("B8")), Piece::try_of_str("♞"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("C8")), Piece::try_of_str("♝"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("D8")), Piece::try_of_str("♛"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("E8")), Piece::try_of_str("♚"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("F8")), Piece::try_of_str("♝"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("G8")), Piece::try_of_str("♞"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("H8")), Piece::try_of_str("♜"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("H8")), Piece::try_of_str("♜"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("H7")), Piece::try_of_str("♟"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("H6")), Piece::try_of_str(" "));
        assert_eq!(get_board_piece(&b, &Pos::of_str("H5")), Piece::try_of_str(" "));
        assert_eq!(get_board_piece(&b, &Pos::of_str("H4")), Piece::try_of_str(" "));
        assert_eq!(get_board_piece(&b, &Pos::of_str("H3")), Piece::try_of_str(" "));
        assert_eq!(get_board_piece(&b, &Pos::of_str("H2")), Piece::try_of_str("♙"));
        assert_eq!(get_board_piece(&b, &Pos::of_str("H1")), Piece::try_of_str("♖"));
    }

    #[test]
    fn test_board_to_string() {
        let brd = get_initial_black_board();
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
}
