use std::{collections::HashMap, fmt};

use crate::{game::game::GameBounds, piece::Piece, pos::Pos};

#[derive(Debug, PartialEq)]
pub struct InvalidCharacterErr;

impl fmt::Display for InvalidCharacterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Must match the pattern [♖♘♗♕♔♙♜♞♝♛♚♟ ]")
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidLengthErr;

impl fmt::Display for InvalidLengthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Every column and row lengths must match the bounds")
    }
}

#[derive(Debug, PartialEq)]
pub enum GameBoardErr {
    InvalidCharacter(InvalidCharacterErr),
    InvalidLength(InvalidLengthErr),
}

pub type GameBoard = HashMap<Pos, Piece>;

pub fn board_empty() -> GameBoard {
    HashMap::new()
}

pub fn board_try_of_str<const N: usize>(
    bounds: &GameBounds,
    rows: [&str; N],
) -> Result<GameBoard, GameBoardErr> {
    if rows.join("").find(|c| c != ' ' && Piece::try_of(c).is_none()).is_some() {
        return Err(GameBoardErr::InvalidCharacter(InvalidCharacterErr));
    }
    let delta_x = usize::from(bounds.x2 - bounds.x1) + 1;
    let delta_y: usize = usize::from(bounds.y2 - bounds.y1) + 1;
    if rows.len() != delta_y {
        return Err(GameBoardErr::InvalidLength(InvalidLengthErr));
    }
    for row in rows {
        if row.chars().count() != delta_x {
            return Err(GameBoardErr::InvalidLength(InvalidLengthErr));
        }
    }
    let mut board = HashMap::new();
    for row in bounds.y1..=bounds.y2 {
        for col in bounds.x1..=bounds.x2 {
            let row_index = bounds.y2 - row;
            let col_index = col - bounds.x1;
            let str_row = rows[row_index as usize];
            let str_col = str_row.chars().nth(col_index.into()).unwrap();
            if let Some(piece) = Piece::try_of(str_col) {
                board.insert(Pos { row, col }, piece);
            }
        }
    }
    Ok(board)
}

pub fn board_of_str<const N: usize>(bounds: &GameBounds, rows: [&str; N]) -> GameBoard {
    board_try_of_str(bounds, rows).unwrap()
}

pub fn board_to_string(bounds: &GameBounds, board: &GameBoard) -> String {
    let mut res = "".to_string();
    let mut row = bounds.y2 + 1;
    while row > bounds.y1 {
        row -= 1;
        for col in bounds.x1..=bounds.x2 {
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

    use crate::{
        game::{game::GameBounds, mode::standard_chess},
        piece::Piece,
        pos::Pos,
    };

    use super::{
        GameBoardErr, InvalidCharacterErr, InvalidLengthErr, board_of_str, board_to_string,
        board_try_of_str,
    };

    #[test]
    fn invalid_character_err() {
        assert_eq!(InvalidCharacterErr.to_string(), "Must match the pattern [♖♘♗♕♔♙♜♞♝♛♚♟ ]");
    }

    #[test]
    fn invalid_length_err() {
        assert_eq!(
            InvalidLengthErr.to_string(),
            "Every column and row lengths must match the bounds"
        );
    }

    #[test]
    fn board_try_of_str_ok() {
        let mode = standard_chess();
        assert_eq!(
            board_try_of_str(
                &mode.bounds,
                [
                    "♜♞♝♛♚♝♞♜",
                    "♟♟♟♟♟♟♟♟",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "♙♙♙♙♙♙♙♙",
                    "♖♘♗♕♔♗♘♖",
                ]
            ),
            Ok([
                (Pos::of("A8"), Piece::of('♜')),
                (Pos::of("B8"), Piece::of('♞')),
                (Pos::of("C8"), Piece::of('♝')),
                (Pos::of("D8"), Piece::of('♛')),
                (Pos::of("E8"), Piece::of('♚')),
                (Pos::of("F8"), Piece::of('♝')),
                (Pos::of("G8"), Piece::of('♞')),
                (Pos::of("H8"), Piece::of('♜')),
                (Pos::of("A7"), Piece::of('♟')),
                (Pos::of("B7"), Piece::of('♟')),
                (Pos::of("C7"), Piece::of('♟')),
                (Pos::of("D7"), Piece::of('♟')),
                (Pos::of("E7"), Piece::of('♟')),
                (Pos::of("F7"), Piece::of('♟')),
                (Pos::of("G7"), Piece::of('♟')),
                (Pos::of("H7"), Piece::of('♟')),
                (Pos::of("A2"), Piece::of('♙')),
                (Pos::of("B2"), Piece::of('♙')),
                (Pos::of("C2"), Piece::of('♙')),
                (Pos::of("D2"), Piece::of('♙')),
                (Pos::of("E2"), Piece::of('♙')),
                (Pos::of("F2"), Piece::of('♙')),
                (Pos::of("G2"), Piece::of('♙')),
                (Pos::of("H2"), Piece::of('♙')),
                (Pos::of("A1"), Piece::of('♖')),
                (Pos::of("B1"), Piece::of('♘')),
                (Pos::of("C1"), Piece::of('♗')),
                (Pos::of("D1"), Piece::of('♕')),
                (Pos::of("E1"), Piece::of('♔')),
                (Pos::of("F1"), Piece::of('♗')),
                (Pos::of("G1"), Piece::of('♘')),
                (Pos::of("H1"), Piece::of('♖')),
            ]
            .into())
        );
    }

    #[test]
    fn board_try_of_str_custom_bounds() {
        assert_eq!(
            board_try_of_str(
                &GameBounds { x1: 10, y1: 10, x2: 13, y2: 13 },
                [
                    " ♛♚ ", //
                    "    ", //
                    "    ", //
                    " ♕♔ ", //
                ]
            ),
            Ok([
                (Pos::of("L14"), Piece::of('♛')),
                (Pos::of("M14"), Piece::of('♚')),
                (Pos::of("L11"), Piece::of('♕')),
                (Pos::of("M11"), Piece::of('♔')),
            ]
            .into())
        );
    }

    #[test]
    fn board_try_of_str_invalid_character_err() {
        let mode = standard_chess();
        assert_eq!(
            board_try_of_str(
                &mode.bounds,
                [
                    "RNBQKBNR",
                    "PPPPPPPP",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "♙♙♙♙♙♙♙♙",
                    "♖♘♗♕♔♗♘♖",
                ]
            ),
            Err(GameBoardErr::InvalidCharacter(InvalidCharacterErr))
        );
    }

    #[test]
    fn board_try_of_str_invalid_col_len() {
        let mode = standard_chess();
        assert_eq!(
            board_try_of_str(
                &mode.bounds,
                [
                    "♜♞♝♛♚♝♞",
                    "♟♟♟♟♟♟♟♟",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "♙♙♙♙♙♙♙♙",
                    "♖♘♗♕♔♗♘♖",
                ]
            ),
            Err(GameBoardErr::InvalidLength(InvalidLengthErr))
        );
    }

    #[test]
    fn board_try_of_str_invalid_row_len() {
        let mode = standard_chess();
        assert_eq!(
            board_try_of_str(
                &mode.bounds,
                [
                    "♜♞♝♛♚♝♞♜",
                    "♟♟♟♟♟♟♟♟",
                    "        ",
                    "        ",
                    "        ",
                    "♙♙♙♙♙♙♙♙",
                    "♖♘♗♕♔♗♘♖",
                ]
            ),
            Err(GameBoardErr::InvalidLength(InvalidLengthErr))
        );
    }

    #[test]
    fn test_of_str() {
        let mode = standard_chess();
        assert_eq!(
            board_of_str(
                &mode.bounds,
                [
                    "♜♞♝♛♚♝♞♜",
                    "♟♟♟♟♟♟♟♟",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "♙♙♙♙♙♙♙♙",
                    "♖♘♗♕♔♗♘♖",
                ]
            ),
            [
                (Pos::of("A8"), Piece::of('♜')),
                (Pos::of("B8"), Piece::of('♞')),
                (Pos::of("C8"), Piece::of('♝')),
                (Pos::of("D8"), Piece::of('♛')),
                (Pos::of("E8"), Piece::of('♚')),
                (Pos::of("F8"), Piece::of('♝')),
                (Pos::of("G8"), Piece::of('♞')),
                (Pos::of("H8"), Piece::of('♜')),
                (Pos::of("A7"), Piece::of('♟')),
                (Pos::of("B7"), Piece::of('♟')),
                (Pos::of("C7"), Piece::of('♟')),
                (Pos::of("D7"), Piece::of('♟')),
                (Pos::of("E7"), Piece::of('♟')),
                (Pos::of("F7"), Piece::of('♟')),
                (Pos::of("G7"), Piece::of('♟')),
                (Pos::of("H7"), Piece::of('♟')),
                (Pos::of("A2"), Piece::of('♙')),
                (Pos::of("B2"), Piece::of('♙')),
                (Pos::of("C2"), Piece::of('♙')),
                (Pos::of("D2"), Piece::of('♙')),
                (Pos::of("E2"), Piece::of('♙')),
                (Pos::of("F2"), Piece::of('♙')),
                (Pos::of("G2"), Piece::of('♙')),
                (Pos::of("H2"), Piece::of('♙')),
                (Pos::of("A1"), Piece::of('♖')),
                (Pos::of("B1"), Piece::of('♘')),
                (Pos::of("C1"), Piece::of('♗')),
                (Pos::of("D1"), Piece::of('♕')),
                (Pos::of("E1"), Piece::of('♔')),
                (Pos::of("F1"), Piece::of('♗')),
                (Pos::of("G1"), Piece::of('♘')),
                (Pos::of("H1"), Piece::of('♖')),
            ]
            .into()
        );
    }

    #[test]
    fn test_board_to_string() {
        let mode = standard_chess();
        assert_eq!(
            board_to_string(
                &mode.bounds,
                &board_of_str(
                    &mode.bounds,
                    [
                        "♜♞♝♛♚♝♞♜",
                        "♟♟♟♟♟♟♟♟",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "♙♙♙♙♙♙♙♙",
                        "♖♘♗♕♔♗♘♖",
                    ]
                )
            ),
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
    fn test_board_to_string_custom_bounds() {
        let bounds = GameBounds { x1: 10, y1: 10, x2: 13, y2: 13 };
        assert_eq!(
            board_to_string(
                &bounds,
                &board_of_str(
                    &bounds,
                    [
                        " ♛♚ ", //
                        "    ", //
                        "    ", //
                        " ♕♔ ", //
                    ]
                )
            ),
            "".to_owned() + " ♛♚ \n" + "    \n" + "    \n" + " ♕♔ \n"
        );
    }
}
