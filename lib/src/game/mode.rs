use std::collections::HashMap;

use crate::game::{board::GameBoard, game::GameBounds, piece::piece_of_str};

#[derive(Debug, PartialEq)]
pub struct GameMode {
    pub bounds: GameBounds,
    pub initial_board: GameBoard,
}

pub fn standard_chess() -> GameMode {
    GameMode {
        bounds: GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 },
        initial_board: HashMap::from([
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
        ]),
    }
}

pub fn chess_960() {
    //
}

#[cfg(test)]
mod tests {
    use crate::game::{board::board_of_str, game::GameBounds};

    use super::{GameMode, standard_chess};

    #[test]
    fn test_chess_standard() {
        let mode = standard_chess();
        assert_eq!(
            mode,
            GameMode {
                bounds: GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 },
                initial_board: board_of_str(
                    &mode,
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
            }
        );
    }
}
