use std::{
    fmt,
    ops::{Index, IndexMut},
};

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

#[derive(Debug, PartialEq)]
pub struct Board {
    pub b: [[Option<Piece>; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        Board { b: [[None; 8]; 8] }
    }
}

impl Index<Pos> for Board {
    type Output = Option<Piece>;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.b[pos.row.to_idx() as usize][pos.col.to_idx() as usize]
    }
}

impl IndexMut<Pos> for Board {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.b[pos.row.to_idx() as usize][pos.col.to_idx() as usize]
    }
}

impl Board {
    fn try_of_str(rows: [&str; 8]) -> Result<Self, FromStringErr> {
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
        let mut board = Self::default();
        for row in 0..8 {
            for col in 0..8 {
                let piece = Piece::try_of_str(
                    &rows[row as usize].chars().nth(col.into()).unwrap().to_string(),
                );
                board[Pos::of_idx(row, col)] = piece;
            }
        }
        Ok(board)
    }

    pub fn of_str(rows: [&str; 8]) -> Board {
        Self::try_of_str(rows).unwrap()
    }

    pub fn get_initial_board() -> Board {
        Self::of_str([
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

    fn to_string(&self) -> String {
        let mut res: String = String::from("");
        for row in self.b {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_of_str() {
        assert_eq!(
            Board::try_of_str([
                "♜♞♝♛♚♝♞♜",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖♘♗♕♔♗♘♖",
            ]),
            Ok(Board {
                b: [
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
                ]
            })
        );
    }

    #[test]
    fn test_of_str() {
        assert_eq!(
            Board::of_str([
                "♜♞♝♛♚♝♞♜",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖♘♗♕♔♗♘♖",
            ]),
            Board {
                b: [
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
                ]
            }
        );
    }

    #[test]
    fn test_get_piece() {
        let board = Board::get_initial_board();
        assert_eq!(board[Pos::of_str("A8")], Piece::try_of_str("♜"));
        assert_eq!(board[Pos::of_str("B8")], Piece::try_of_str("♞"));
        assert_eq!(board[Pos::of_str("C8")], Piece::try_of_str("♝"));
        assert_eq!(board[Pos::of_str("D8")], Piece::try_of_str("♛"));
        assert_eq!(board[Pos::of_str("E8")], Piece::try_of_str("♚"));
        assert_eq!(board[Pos::of_str("F8")], Piece::try_of_str("♝"));
        assert_eq!(board[Pos::of_str("G8")], Piece::try_of_str("♞"));
        assert_eq!(board[Pos::of_str("H8")], Piece::try_of_str("♜"));
        assert_eq!(board[Pos::of_str("H8")], Piece::try_of_str("♜"));
        assert_eq!(board[Pos::of_str("H7")], Piece::try_of_str("♟"));
        assert_eq!(board[Pos::of_str("H6")], Piece::try_of_str(" "));
        assert_eq!(board[Pos::of_str("H5")], Piece::try_of_str(" "));
        assert_eq!(board[Pos::of_str("H4")], Piece::try_of_str(" "));
        assert_eq!(board[Pos::of_str("H3")], Piece::try_of_str(" "));
        assert_eq!(board[Pos::of_str("H2")], Piece::try_of_str("♙"));
        assert_eq!(board[Pos::of_str("H1")], Piece::try_of_str("♖"));
    }

    #[test]
    fn test_set_piece() {
        let mut board = Board::get_initial_board();
        assert_eq!(board[Pos::of_str("A8")], Piece::try_of_str("♜"));
        board[Pos::of_str("A8")] = Piece::try_of_str("♖");
        assert_eq!(board[Pos::of_str("A8")], Piece::try_of_str("♖"));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            Board::get_initial_board().to_string(),
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
