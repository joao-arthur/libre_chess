use crate::{piece::Piece, pos::Pos};

pub fn game_piece_of(pos: &str, piece: char) -> (Pos, Piece) {
    (Pos::of_str(pos), Piece::of(piece))
}

#[cfg(test)]
mod tests {
    use crate::{piece::Piece, pos::Pos};

    use super::game_piece_of;

    #[test]
    fn try_of_str_ok() {
        assert_eq!(game_piece_of("A8", '♜'), (Pos::of_str("A8"), Piece::of('♜')));
        assert_eq!(game_piece_of("B8", '♞'), (Pos::of_str("B8"), Piece::of('♞')));
        assert_eq!(game_piece_of("C8", '♝'), (Pos::of_str("C8"), Piece::of('♝')));
        assert_eq!(game_piece_of("D8", '♛'), (Pos::of_str("D8"), Piece::of('♛')));
        assert_eq!(game_piece_of("E8", '♚'), (Pos::of_str("E8"), Piece::of('♚')));
        assert_eq!(game_piece_of("F8", '♝'), (Pos::of_str("F8"), Piece::of('♝')));
        assert_eq!(game_piece_of("G8", '♞'), (Pos::of_str("G8"), Piece::of('♞')));
        assert_eq!(game_piece_of("H8", '♜'), (Pos::of_str("H8"), Piece::of('♜')));
        assert_eq!(game_piece_of("A7", '♟'), (Pos::of_str("A7"), Piece::of('♟')));
        assert_eq!(game_piece_of("B7", '♟'), (Pos::of_str("B7"), Piece::of('♟')));
        assert_eq!(game_piece_of("C7", '♟'), (Pos::of_str("C7"), Piece::of('♟')));
        assert_eq!(game_piece_of("D7", '♟'), (Pos::of_str("D7"), Piece::of('♟')));
        assert_eq!(game_piece_of("E7", '♟'), (Pos::of_str("E7"), Piece::of('♟')));
        assert_eq!(game_piece_of("F7", '♟'), (Pos::of_str("F7"), Piece::of('♟')));
        assert_eq!(game_piece_of("G7", '♟'), (Pos::of_str("G7"), Piece::of('♟')));
        assert_eq!(game_piece_of("H7", '♟'), (Pos::of_str("H7"), Piece::of('♟')));
        assert_eq!(game_piece_of("A2", '♙'), (Pos::of_str("A2"), Piece::of('♙')));
        assert_eq!(game_piece_of("B2", '♙'), (Pos::of_str("B2"), Piece::of('♙')));
        assert_eq!(game_piece_of("C2", '♙'), (Pos::of_str("C2"), Piece::of('♙')));
        assert_eq!(game_piece_of("D2", '♙'), (Pos::of_str("D2"), Piece::of('♙')));
        assert_eq!(game_piece_of("E2", '♙'), (Pos::of_str("E2"), Piece::of('♙')));
        assert_eq!(game_piece_of("F2", '♙'), (Pos::of_str("F2"), Piece::of('♙')));
        assert_eq!(game_piece_of("G2", '♙'), (Pos::of_str("G2"), Piece::of('♙')));
        assert_eq!(game_piece_of("H2", '♙'), (Pos::of_str("H2"), Piece::of('♙')));
        assert_eq!(game_piece_of("A1", '♖'), (Pos::of_str("A1"), Piece::of('♖')));
        assert_eq!(game_piece_of("B1", '♘'), (Pos::of_str("B1"), Piece::of('♘')));
        assert_eq!(game_piece_of("C1", '♗'), (Pos::of_str("C1"), Piece::of('♗')));
        assert_eq!(game_piece_of("D1", '♕'), (Pos::of_str("D1"), Piece::of('♕')));
        assert_eq!(game_piece_of("E1", '♔'), (Pos::of_str("E1"), Piece::of('♔')));
        assert_eq!(game_piece_of("F1", '♗'), (Pos::of_str("F1"), Piece::of('♗')));
        assert_eq!(game_piece_of("G1", '♘'), (Pos::of_str("G1"), Piece::of('♘')));
        assert_eq!(game_piece_of("H1", '♖'), (Pos::of_str("H1"), Piece::of('♖')));
    }
}
