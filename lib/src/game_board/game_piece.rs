use crate::{board::pos::Pos, piece::Piece};

pub fn of_str(pos: &str, piece: &str) -> (Pos, Piece) {
    (Pos::of_str(pos), Piece::of_str(piece))
}

#[cfg(test)]
mod tests {
    use crate::{board::pos::Pos, piece::Piece};

    use super::of_str;

    #[test]
    fn try_of_str_ok() {
        assert_eq!(of_str("A8", "♜"), (Pos::of_str("A8"), Piece::of_str("♜")));
        assert_eq!(of_str("B8", "♞"), (Pos::of_str("B8"), Piece::of_str("♞")));
        assert_eq!(of_str("C8", "♝"), (Pos::of_str("C8"), Piece::of_str("♝")));
        assert_eq!(of_str("D8", "♛"), (Pos::of_str("D8"), Piece::of_str("♛")));
        assert_eq!(of_str("E8", "♚"), (Pos::of_str("E8"), Piece::of_str("♚")));
        assert_eq!(of_str("F8", "♝"), (Pos::of_str("F8"), Piece::of_str("♝")));
        assert_eq!(of_str("G8", "♞"), (Pos::of_str("G8"), Piece::of_str("♞")));
        assert_eq!(of_str("H8", "♜"), (Pos::of_str("H8"), Piece::of_str("♜")));
        assert_eq!(of_str("A7", "♟"), (Pos::of_str("A7"), Piece::of_str("♟")));
        assert_eq!(of_str("B7", "♟"), (Pos::of_str("B7"), Piece::of_str("♟")));
        assert_eq!(of_str("C7", "♟"), (Pos::of_str("C7"), Piece::of_str("♟")));
        assert_eq!(of_str("D7", "♟"), (Pos::of_str("D7"), Piece::of_str("♟")));
        assert_eq!(of_str("E7", "♟"), (Pos::of_str("E7"), Piece::of_str("♟")));
        assert_eq!(of_str("F7", "♟"), (Pos::of_str("F7"), Piece::of_str("♟")));
        assert_eq!(of_str("G7", "♟"), (Pos::of_str("G7"), Piece::of_str("♟")));
        assert_eq!(of_str("H7", "♟"), (Pos::of_str("H7"), Piece::of_str("♟")));
        assert_eq!(of_str("A2", "♙"), (Pos::of_str("A2"), Piece::of_str("♙")));
        assert_eq!(of_str("B2", "♙"), (Pos::of_str("B2"), Piece::of_str("♙")));
        assert_eq!(of_str("C2", "♙"), (Pos::of_str("C2"), Piece::of_str("♙")));
        assert_eq!(of_str("D2", "♙"), (Pos::of_str("D2"), Piece::of_str("♙")));
        assert_eq!(of_str("E2", "♙"), (Pos::of_str("E2"), Piece::of_str("♙")));
        assert_eq!(of_str("F2", "♙"), (Pos::of_str("F2"), Piece::of_str("♙")));
        assert_eq!(of_str("G2", "♙"), (Pos::of_str("G2"), Piece::of_str("♙")));
        assert_eq!(of_str("H2", "♙"), (Pos::of_str("H2"), Piece::of_str("♙")));
        assert_eq!(of_str("A1", "♖"), (Pos::of_str("A1"), Piece::of_str("♖")));
        assert_eq!(of_str("B1", "♘"), (Pos::of_str("B1"), Piece::of_str("♘")));
        assert_eq!(of_str("C1", "♗"), (Pos::of_str("C1"), Piece::of_str("♗")));
        assert_eq!(of_str("D1", "♕"), (Pos::of_str("D1"), Piece::of_str("♕")));
        assert_eq!(of_str("E1", "♔"), (Pos::of_str("E1"), Piece::of_str("♔")));
        assert_eq!(of_str("F1", "♗"), (Pos::of_str("F1"), Piece::of_str("♗")));
        assert_eq!(of_str("G1", "♘"), (Pos::of_str("G1"), Piece::of_str("♘")));
        assert_eq!(of_str("H1", "♖"), (Pos::of_str("H1"), Piece::of_str("♖")));
    }
}
