use std::{collections::HashMap, fmt};

use crate::{board::pos::Pos, game::mode::GameMode, piece::Piece};

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

pub type GameBoard = HashMap<Pos, Piece>;

pub fn board_empty() -> GameBoard {
    HashMap::new()
}

pub fn board_try_of_str<const N: usize>(mode: &GameMode, rows: [&str; N]) -> Result<GameBoard, FromStringErr> {
    if rows.join("").find(|c: char| c != ' ' && Piece::try_of(c).is_none()).is_some() {
        return Err(FromStringErr::InvalidCharacter(InvalidCharacterErr));
    }
    //for line in rows {
    //    if line.chars().count() != 8 {
    //        return Err(FromStringErr::InvalidLength(InvalidLengthErr));
    //    }
    //}
    let rows_iter = rows.iter().rev();
    let mut board = HashMap::new();
    for row in 0..8 {
        for col in 0..8 {
            let pos_str = rows[7 - row as usize].chars().nth(col.into()).unwrap();
            if let Some(piece) = Piece::try_of(pos_str) {
                board.insert(Pos { row, col }, piece);
            }
        }
    }
    Ok(board)
}

pub fn board_of_str(mode: &GameMode, rows: [&str; 8]) -> GameBoard {
    board_try_of_str(mode, rows).unwrap()
}

fn board_to_string(mode: &GameMode, board: &GameBoard) -> String {
    let mut res = "".to_string();
    let mut row = 8;
    while row > 0 {
        row -= 1;
        for col in 0..8 {
            match board.get(&Pos { row, col }) {
                Some(p) => res.push_str(&p.to_string()),
                None => res.push(' '),
            }
        }
        res.push('\n')
    }
    res
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::game::piece::piece_of_str;

    use super::{FromStringErr, InvalidCharacterErr, board_of_str, board_to_string, board_try_of_str};

    #[test]
    fn try_of_str_ok() {
        assert_eq!(
            board_try_of_str([
                "♜♞♝♛♚♝♞♜",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖♘♗♕♔♗♘♖",
            ])
            .unwrap(),
            HashMap::from([
                piece_of_str("A8", '♜'),
                piece_of_str("B8", '♞'),
                piece_of_str("C8", '♝'),
                piece_of_str("D8", '♛'),
                piece_of_str("E8", '♚'),
                piece_of_str("F8", '♝'),
                piece_of_str("G8", '♞'),
                piece_of_str("H8", '♜'),
                piece_of_str("A7", '♟'),
                piece_of_str("B7", '♟'),
                piece_of_str("C7", '♟'),
                piece_of_str("D7", '♟'),
                piece_of_str("E7", '♟'),
                piece_of_str("F7", '♟'),
                piece_of_str("G7", '♟'),
                piece_of_str("H7", '♟'),
                piece_of_str("A2", '♙'),
                piece_of_str("B2", '♙'),
                piece_of_str("C2", '♙'),
                piece_of_str("D2", '♙'),
                piece_of_str("E2", '♙'),
                piece_of_str("F2", '♙'),
                piece_of_str("G2", '♙'),
                piece_of_str("H2", '♙'),
                piece_of_str("A1", '♖'),
                piece_of_str("B1", '♘'),
                piece_of_str("C1", '♗'),
                piece_of_str("D1", '♕'),
                piece_of_str("E1", '♔'),
                piece_of_str("F1", '♗'),
                piece_of_str("G1", '♘'),
                piece_of_str("H1", '♖'),
            ])
        );
    }

    #[test]
    fn try_of_str_err() {
        assert_eq!(
            board_try_of_str([
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
        //assert_eq!(
        //    try_of_str([
        //        "♜♞♝♛♚♝♞",
        //        "♟♟♟♟♟♟♟♟",
        //        "        ",
        //        "        ",
        //        "        ",
        //        "        ",
        //        "♙♙♙♙♙♙♙♙",
        //        "♖♘♗♕♔♗♘♖",
        //    ]),
        //    Err(FromStringErr::InvalidLength(InvalidLengthErr))
        //);
        //assert_eq!(
        //    format!("{}", InvalidCharacterErr),
        //    "Only [0-9] characters and spaces are allowed!"
        //);
        //assert_eq!(format!("{}", InvalidLengthErr), "Every line must be 8 characters long");
    }

    #[test]
    fn test_of_str() {
        assert_eq!(
            board_of_str([
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
                piece_of_str("A8", '♜'),
                piece_of_str("B8", '♞'),
                piece_of_str("C8", '♝'),
                piece_of_str("D8", '♛'),
                piece_of_str("E8", '♚'),
                piece_of_str("F8", '♝'),
                piece_of_str("G8", '♞'),
                piece_of_str("H8", '♜'),
                piece_of_str("A7", '♟'),
                piece_of_str("B7", '♟'),
                piece_of_str("C7", '♟'),
                piece_of_str("D7", '♟'),
                piece_of_str("E7", '♟'),
                piece_of_str("F7", '♟'),
                piece_of_str("G7", '♟'),
                piece_of_str("H7", '♟'),
                piece_of_str("A2", '♙'),
                piece_of_str("B2", '♙'),
                piece_of_str("C2", '♙'),
                piece_of_str("D2", '♙'),
                piece_of_str("E2", '♙'),
                piece_of_str("F2", '♙'),
                piece_of_str("G2", '♙'),
                piece_of_str("H2", '♙'),
                piece_of_str("A1", '♖'),
                piece_of_str("B1", '♘'),
                piece_of_str("C1", '♗'),
                piece_of_str("D1", '♕'),
                piece_of_str("E1", '♔'),
                piece_of_str("F1", '♗'),
                piece_of_str("G1", '♘'),
                piece_of_str("H1", '♖'),
            ])
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            board_to_string(&board_of_str([
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
