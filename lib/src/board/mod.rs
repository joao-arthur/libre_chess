use std::{collections::HashMap, fmt};

use pos::Pos;

use crate::piece::Piece;

pub mod col;
pub mod pos;
pub mod row;

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

pub type Board = HashMap<Pos, Piece>;

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
    let mut board = HashMap::new();
    for row in 0..8 {
        for col in 0..8 {
            let pos_str = rows[row as usize].chars().nth(col.into()).unwrap().to_string();
            if let Some(piece) = Piece::try_of_str(&pos_str) {
                board.insert(Pos::of_idx(row, col), piece);
            }
        }
    }
    Ok(board)
}

pub fn of_str(rows: [&str; 8]) -> Board {
    try_of_str(rows).unwrap()
}

fn to_string(board: &Board) -> String {
    let mut res: String = String::from("");
    for row in 0..8 {
        for col in 0..8 {
            match board.get(&Pos::of_idx(row, col)) {
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
    fn test_try_of_str_ok() {
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
            Ok(HashMap::from([
                (Pos::of_str("A8"), Piece::of_str("♜")),
                (Pos::of_str("B8"), Piece::of_str("♞")),
                (Pos::of_str("C8"), Piece::of_str("♝")),
                (Pos::of_str("D8"), Piece::of_str("♛")),
                (Pos::of_str("E8"), Piece::of_str("♚")),
                (Pos::of_str("F8"), Piece::of_str("♝")),
                (Pos::of_str("G8"), Piece::of_str("♞")),
                (Pos::of_str("H8"), Piece::of_str("♜")),
                (Pos::of_str("A7"), Piece::of_str("♟")),
                (Pos::of_str("B7"), Piece::of_str("♟")),
                (Pos::of_str("C7"), Piece::of_str("♟")),
                (Pos::of_str("D7"), Piece::of_str("♟")),
                (Pos::of_str("E7"), Piece::of_str("♟")),
                (Pos::of_str("F7"), Piece::of_str("♟")),
                (Pos::of_str("G7"), Piece::of_str("♟")),
                (Pos::of_str("H7"), Piece::of_str("♟")),
                (Pos::of_str("A2"), Piece::of_str("♙")),
                (Pos::of_str("B2"), Piece::of_str("♙")),
                (Pos::of_str("C2"), Piece::of_str("♙")),
                (Pos::of_str("D2"), Piece::of_str("♙")),
                (Pos::of_str("E2"), Piece::of_str("♙")),
                (Pos::of_str("F2"), Piece::of_str("♙")),
                (Pos::of_str("G2"), Piece::of_str("♙")),
                (Pos::of_str("H2"), Piece::of_str("♙")),
                (Pos::of_str("A1"), Piece::of_str("♖")),
                (Pos::of_str("B1"), Piece::of_str("♘")),
                (Pos::of_str("C1"), Piece::of_str("♗")),
                (Pos::of_str("D1"), Piece::of_str("♕")),
                (Pos::of_str("E1"), Piece::of_str("♔")),
                (Pos::of_str("F1"), Piece::of_str("♗")),
                (Pos::of_str("G1"), Piece::of_str("♘")),
                (Pos::of_str("H1"), Piece::of_str("♖")),
            ]))
        );
    }

    #[test]
    fn test_try_of_str_err() {
        assert_eq!(
            try_of_str([
                "RNBQKBNR",
                "PPPPPPPP",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖♘♗♕♔♗♘♖",
            ]),
            Err(FromStringErr::InvalidCharacter(InvalidCharacterErr))
        );
        assert_eq!(
            try_of_str([
                "♜♞♝♛♚♝♞",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖♘♗♕♔♗♘♖",
            ]),
            Err(FromStringErr::InvalidLength(InvalidLengthErr))
        );

        assert_eq!(
            format!("{}", InvalidCharacterErr),
            "Only [0-9] characters and spaces are allowed!"
        );
        assert_eq!(format!("{}", InvalidLengthErr), "Every line must be 8 characters long");
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
            HashMap::from([
                (Pos::of_str("A8"), Piece::of_str("♜")),
                (Pos::of_str("B8"), Piece::of_str("♞")),
                (Pos::of_str("C8"), Piece::of_str("♝")),
                (Pos::of_str("D8"), Piece::of_str("♛")),
                (Pos::of_str("E8"), Piece::of_str("♚")),
                (Pos::of_str("F8"), Piece::of_str("♝")),
                (Pos::of_str("G8"), Piece::of_str("♞")),
                (Pos::of_str("H8"), Piece::of_str("♜")),
                (Pos::of_str("A7"), Piece::of_str("♟")),
                (Pos::of_str("B7"), Piece::of_str("♟")),
                (Pos::of_str("C7"), Piece::of_str("♟")),
                (Pos::of_str("D7"), Piece::of_str("♟")),
                (Pos::of_str("E7"), Piece::of_str("♟")),
                (Pos::of_str("F7"), Piece::of_str("♟")),
                (Pos::of_str("G7"), Piece::of_str("♟")),
                (Pos::of_str("H7"), Piece::of_str("♟")),
                (Pos::of_str("A2"), Piece::of_str("♙")),
                (Pos::of_str("B2"), Piece::of_str("♙")),
                (Pos::of_str("C2"), Piece::of_str("♙")),
                (Pos::of_str("D2"), Piece::of_str("♙")),
                (Pos::of_str("E2"), Piece::of_str("♙")),
                (Pos::of_str("F2"), Piece::of_str("♙")),
                (Pos::of_str("G2"), Piece::of_str("♙")),
                (Pos::of_str("H2"), Piece::of_str("♙")),
                (Pos::of_str("A1"), Piece::of_str("♖")),
                (Pos::of_str("B1"), Piece::of_str("♘")),
                (Pos::of_str("C1"), Piece::of_str("♗")),
                (Pos::of_str("D1"), Piece::of_str("♕")),
                (Pos::of_str("E1"), Piece::of_str("♔")),
                (Pos::of_str("F1"), Piece::of_str("♗")),
                (Pos::of_str("G1"), Piece::of_str("♘")),
                (Pos::of_str("H1"), Piece::of_str("♖")),
            ])
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            to_string(&of_str([
                "♜♞♝♛♚♝♞♜",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖♘♗♕♔♗♘♖",
            ])),
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
