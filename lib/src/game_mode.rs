use crate::game_board::{Board, of_str};

#[derive(Debug, PartialEq)]
pub struct GameMode {
    pub row_len: u8,
    pub col_len: u8,
    pub initial_board: Board,
}

pub fn chess_standard() -> GameMode {
    GameMode {
        row_len: 8,
        col_len: 8,
        initial_board: of_str([
            "♜♞♝♛♚♝♞♜",
            "♟♟♟♟♟♟♟♟",
            "        ",
            "        ",
            "        ",
            "        ",
            "♙♙♙♙♙♙♙♙",
            "♖♘♗♕♔♗♘♖",
        ]),
    }
}

pub fn chess_960() {
    //
}

#[cfg(test)]
mod tests {
    use crate::game_board::of_str;

    use super::{GameMode, chess_960, chess_standard};

    #[test]
    fn test_chess_standard() {
        assert_eq!(
            chess_standard(),
            GameMode {
                row_len: 8,
                col_len: 8,
                initial_board: of_str([
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
        );
    }
}
