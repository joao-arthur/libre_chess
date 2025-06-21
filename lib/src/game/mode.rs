use crate::{
    game::board::{GameBoard, of_str},
    geometry::poligon::rect::RectU8,
};

#[derive(Debug, PartialEq)]
pub struct GameMode {
    pub bounds: RectU8,
    pub initial_board: GameBoard,
}

pub fn standard_chess() -> GameMode {
    GameMode {
        bounds: RectU8 { x1: 0, y1: 0, x2: 7, y2: 7 },
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
    use crate::{game::board::of_str, geometry::poligon::rect::RectU8};

    use super::{GameMode, standard_chess};

    #[test]
    fn test_chess_standard() {
        assert_eq!(
            standard_chess(),
            GameMode {
                bounds: RectU8 { x1: 0, y1: 0, x2: 7, y2: 7 },
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
