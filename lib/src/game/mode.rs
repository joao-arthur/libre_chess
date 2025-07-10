use std::collections::HashMap;

use crate::game::{board::GameBoard, game::GameBounds, piece::game_piece_of};

#[derive(Debug, PartialEq)]
pub struct GameMode {
    pub bounds: GameBounds,
    pub initial_board: GameBoard,
}

pub fn standard_chess() -> GameMode {
    GameMode {
        bounds: GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 },
        initial_board: HashMap::from([
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
        ]),
    }
}

// pub fn chess_960() {
//     let mut order: [Option<Type>; 8] = [None, None, None, None, None, None, None, None];
//
//     let black_col_bishop_i = random(0, 3) * 2;
//     let white_col_bishop_i = random(0, 3) * 2 + 1;
//
//     let queen_i = random(0, 5);
//     let st_knight_i = random(0, 4);
//     let nd_knight_i = random(0, 3);
//
//     let mut final_queen_i = 0;
//     let mut final_st_knight_i = 0;
//     let mut final_nd_knight_i = 0;
//
//     let mut i = 0;
//
//     loop {
//         if order[i].is_none() {
//
//         }
//     }
//
//     order[black_col_bishop_i] = PieceTypeBishop;
//     order[white_col_bishop_i] = PieceTypeBishop;
//
//     order[final_queen_i] = PieceTypeQueen;
//     order[final_st_knight_i] = PieceTypeKnight;
//     order[final_nd_knight_i] = PieceTypeKnight;
// }

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
            }
        );
    }
}
