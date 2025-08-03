use crate::{
    game::{board::GameBoard, game::GameBounds},
    piece::Piece,
    pos::pos_of,
};

#[derive(Debug, PartialEq)]
pub struct GameMode {
    pub bounds: GameBounds,
    pub initial_board: GameBoard,
}

pub fn standard_chess() -> GameMode {
    GameMode {
        bounds: GameBounds::of(0, 0, 7, 7),
        initial_board: [
            (pos_of("A8"), Piece::of('♜')),
            (pos_of("B8"), Piece::of('♞')),
            (pos_of("C8"), Piece::of('♝')),
            (pos_of("D8"), Piece::of('♛')),
            (pos_of("E8"), Piece::of('♚')),
            (pos_of("F8"), Piece::of('♝')),
            (pos_of("G8"), Piece::of('♞')),
            (pos_of("H8"), Piece::of('♜')),
            (pos_of("A7"), Piece::of('♟')),
            (pos_of("B7"), Piece::of('♟')),
            (pos_of("C7"), Piece::of('♟')),
            (pos_of("D7"), Piece::of('♟')),
            (pos_of("E7"), Piece::of('♟')),
            (pos_of("F7"), Piece::of('♟')),
            (pos_of("G7"), Piece::of('♟')),
            (pos_of("H7"), Piece::of('♟')),
            (pos_of("A2"), Piece::of('♙')),
            (pos_of("B2"), Piece::of('♙')),
            (pos_of("C2"), Piece::of('♙')),
            (pos_of("D2"), Piece::of('♙')),
            (pos_of("E2"), Piece::of('♙')),
            (pos_of("F2"), Piece::of('♙')),
            (pos_of("G2"), Piece::of('♙')),
            (pos_of("H2"), Piece::of('♙')),
            (pos_of("A1"), Piece::of('♖')),
            (pos_of("B1"), Piece::of('♘')),
            (pos_of("C1"), Piece::of('♗')),
            (pos_of("D1"), Piece::of('♕')),
            (pos_of("E1"), Piece::of('♔')),
            (pos_of("F1"), Piece::of('♗')),
            (pos_of("G1"), Piece::of('♘')),
            (pos_of("H1"), Piece::of('♖')),
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
                bounds: GameBounds::of(0, 0, 7, 7),
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
