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

    use crate::game::{game::GameBounds, mode::standard_chess, piece::game_piece_of};

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
            Ok(HashMap::from([
                game_piece_of("A8", '♜'),
                game_piece_of("B8", '♞'),
                game_piece_of("C8", '♝'),
                game_piece_of("D8", '♛'),
                game_piece_of("E8", '♚'),
                game_piece_of("F8", '♝'),
                game_piece_of("G8", '♞'),
                game_piece_of("H8", '♜'),
                game_piece_of("A7", '♟'),
                game_piece_of("B7", '♟'),
                game_piece_of("C7", '♟'),
                game_piece_of("D7", '♟'),
                game_piece_of("E7", '♟'),
                game_piece_of("F7", '♟'),
                game_piece_of("G7", '♟'),
                game_piece_of("H7", '♟'),
                game_piece_of("A2", '♙'),
                game_piece_of("B2", '♙'),
                game_piece_of("C2", '♙'),
                game_piece_of("D2", '♙'),
                game_piece_of("E2", '♙'),
                game_piece_of("F2", '♙'),
                game_piece_of("G2", '♙'),
                game_piece_of("H2", '♙'),
                game_piece_of("A1", '♖'),
                game_piece_of("B1", '♘'),
                game_piece_of("C1", '♗'),
                game_piece_of("D1", '♕'),
                game_piece_of("E1", '♔'),
                game_piece_of("F1", '♗'),
                game_piece_of("G1", '♘'),
                game_piece_of("H1", '♖'),
            ]))
        );
    }

    #[test]
    fn board_try_of_str_custom_bounds() {
        assert_eq!(
            board_try_of_str(
                &GameBounds { x1: 10, y1: 10, x2: 13, y2: 13 },
                [" ♛♚ ", "    ", "    ", " ♕♔ ",]
            ),
            Ok(HashMap::from([
                game_piece_of("L14", '♛'),
                game_piece_of("M14", '♚'),
                game_piece_of("L11", '♕'),
                game_piece_of("M11", '♔'),
            ]))
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
            HashMap::from([
                game_piece_of("A8", '♜'),
                game_piece_of("B8", '♞'),
                game_piece_of("C8", '♝'),
                game_piece_of("D8", '♛'),
                game_piece_of("E8", '♚'),
                game_piece_of("F8", '♝'),
                game_piece_of("G8", '♞'),
                game_piece_of("H8", '♜'),
                game_piece_of("A7", '♟'),
                game_piece_of("B7", '♟'),
                game_piece_of("C7", '♟'),
                game_piece_of("D7", '♟'),
                game_piece_of("E7", '♟'),
                game_piece_of("F7", '♟'),
                game_piece_of("G7", '♟'),
                game_piece_of("H7", '♟'),
                game_piece_of("A2", '♙'),
                game_piece_of("B2", '♙'),
                game_piece_of("C2", '♙'),
                game_piece_of("D2", '♙'),
                game_piece_of("E2", '♙'),
                game_piece_of("F2", '♙'),
                game_piece_of("G2", '♙'),
                game_piece_of("H2", '♙'),
                game_piece_of("A1", '♖'),
                game_piece_of("B1", '♘'),
                game_piece_of("C1", '♗'),
                game_piece_of("D1", '♕'),
                game_piece_of("E1", '♔'),
                game_piece_of("F1", '♗'),
                game_piece_of("G1", '♘'),
                game_piece_of("H1", '♖'),
            ])
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
