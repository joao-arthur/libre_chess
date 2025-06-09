use crate::board::{self, Board};

pub fn standard_initial_board() -> Board {
    board::of_str([
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{board::pos::Pos, piece::Piece};

    use super::*;

    #[test]
    fn test_standard_initial_board() {
        assert_eq!(
            standard_initial_board(),
            HashMap::from([
                (Pos::of_str("A8"), Piece::of_str("♜")),
                (Pos::of_str("B8"), Piece::of_str("♞")),
                (Pos::of_str("C8"), Piece::of_str("♝")),
                (Pos::of_str("D8"), Piece::of_str("♛")),
                (Pos::of_str("E8"), Piece::of_str("♚")),
                (Pos::of_str("F8"), Piece::of_str("♝")),
                (Pos::of_str("G8"), Piece::of_str("♞")),
                (Pos::of_str("H8"), Piece::of_str("♜")),
                (Pos::of_str("A7"), Piece::of_str("♟")),
                (Pos::of_str("B7"), Piece::of_str("♟")),
                (Pos::of_str("C7"), Piece::of_str("♟")),
                (Pos::of_str("D7"), Piece::of_str("♟")),
                (Pos::of_str("E7"), Piece::of_str("♟")),
                (Pos::of_str("F7"), Piece::of_str("♟")),
                (Pos::of_str("G7"), Piece::of_str("♟")),
                (Pos::of_str("H7"), Piece::of_str("♟")),
                (Pos::of_str("A2"), Piece::of_str("♙")),
                (Pos::of_str("B2"), Piece::of_str("♙")),
                (Pos::of_str("C2"), Piece::of_str("♙")),
                (Pos::of_str("D2"), Piece::of_str("♙")),
                (Pos::of_str("E2"), Piece::of_str("♙")),
                (Pos::of_str("F2"), Piece::of_str("♙")),
                (Pos::of_str("G2"), Piece::of_str("♙")),
                (Pos::of_str("H2"), Piece::of_str("♙")),
                (Pos::of_str("A1"), Piece::of_str("♖")),
                (Pos::of_str("B1"), Piece::of_str("♘")),
                (Pos::of_str("C1"), Piece::of_str("♗")),
                (Pos::of_str("D1"), Piece::of_str("♕")),
                (Pos::of_str("E1"), Piece::of_str("♔")),
                (Pos::of_str("F1"), Piece::of_str("♗")),
                (Pos::of_str("G1"), Piece::of_str("♘")),
                (Pos::of_str("H1"), Piece::of_str("♖")),
            ])
        );
    }
}
