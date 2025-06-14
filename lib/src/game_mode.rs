use crate::game_board::{of_str, Board};

#[derive(Debug, PartialEq)]
pub struct GameMode {
    row_len: u8,
    col_len: u8,
    initial_board: Board,
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
        ])
    }    
}

pub fn chess_960() {
    //
}

#[cfg(test)]
mod tests {
    use crate::game_board::of_str;
    
    use super::{chess_standard, chess_960, GameMode};

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
