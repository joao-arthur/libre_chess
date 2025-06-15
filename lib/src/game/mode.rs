use crate::game::board::{Board, of_str};

#[derive(Debug, PartialEq)]
pub struct Mode {
    //pub row_0: u8,
    //pub row_1: u8,
    //pub col_0: u8,
    //pub col_1: u8,
    pub row_len: u8,
    pub col_len: u8,
    pub initial_board: Board,
}

pub fn standard_chess() -> Mode {
    Mode {
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
    use crate::game::board::of_str;

    use super::{Mode, standard_chess};

    #[test]
    fn test_chess_standard() {
        assert_eq!(
            standard_chess(),
            Mode {
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
