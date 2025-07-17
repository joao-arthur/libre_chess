use std::collections::HashMap;

use crate::{
    game::{board::GameBoard, game::GameBounds},
    piece::Piece,
    pos::Pos,
};

#[derive(Debug, PartialEq)]
pub struct GameMode {
    pub bounds: GameBounds,
    pub initial_board: GameBoard,
}

pub fn standard_chess() -> GameMode {
    GameMode {
        bounds: GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 },
        initial_board: [
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
        .into(),
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
