use crate::{piece::Piece, pos::Pos};

pub fn game_piece_of(pos: &str, piece: char) -> (Pos, Piece) {
    (Pos::of(pos), Piece::of(piece))
}

#[cfg(test)]
mod tests {
    use crate::{piece::Piece, pos::Pos};

    use super::game_piece_of;

    #[test]
    fn try_of_str_ok() {
        assert_eq!(game_piece_of("A8", '♜'), (Pos::of("A8"), Piece::of('♜')));
        assert_eq!(game_piece_of("B8", '♞'), (Pos::of("B8"), Piece::of('♞')));
        assert_eq!(game_piece_of("C8", '♝'), (Pos::of("C8"), Piece::of('♝')));
        assert_eq!(game_piece_of("D8", '♛'), (Pos::of("D8"), Piece::of('♛')));
        assert_eq!(game_piece_of("E8", '♚'), (Pos::of("E8"), Piece::of('♚')));
        assert_eq!(game_piece_of("F8", '♝'), (Pos::of("F8"), Piece::of('♝')));
        assert_eq!(game_piece_of("G8", '♞'), (Pos::of("G8"), Piece::of('♞')));
        assert_eq!(game_piece_of("H8", '♜'), (Pos::of("H8"), Piece::of('♜')));
        assert_eq!(game_piece_of("A7", '♟'), (Pos::of("A7"), Piece::of('♟')));
        assert_eq!(game_piece_of("B7", '♟'), (Pos::of("B7"), Piece::of('♟')));
        assert_eq!(game_piece_of("C7", '♟'), (Pos::of("C7"), Piece::of('♟')));
        assert_eq!(game_piece_of("D7", '♟'), (Pos::of("D7"), Piece::of('♟')));
        assert_eq!(game_piece_of("E7", '♟'), (Pos::of("E7"), Piece::of('♟')));
        assert_eq!(game_piece_of("F7", '♟'), (Pos::of("F7"), Piece::of('♟')));
        assert_eq!(game_piece_of("G7", '♟'), (Pos::of("G7"), Piece::of('♟')));
        assert_eq!(game_piece_of("H7", '♟'), (Pos::of("H7"), Piece::of('♟')));
        assert_eq!(game_piece_of("A2", '♙'), (Pos::of("A2"), Piece::of('♙')));
        assert_eq!(game_piece_of("B2", '♙'), (Pos::of("B2"), Piece::of('♙')));
        assert_eq!(game_piece_of("C2", '♙'), (Pos::of("C2"), Piece::of('♙')));
        assert_eq!(game_piece_of("D2", '♙'), (Pos::of("D2"), Piece::of('♙')));
        assert_eq!(game_piece_of("E2", '♙'), (Pos::of("E2"), Piece::of('♙')));
        assert_eq!(game_piece_of("F2", '♙'), (Pos::of("F2"), Piece::of('♙')));
        assert_eq!(game_piece_of("G2", '♙'), (Pos::of("G2"), Piece::of('♙')));
        assert_eq!(game_piece_of("H2", '♙'), (Pos::of("H2"), Piece::of('♙')));
        assert_eq!(game_piece_of("A1", '♖'), (Pos::of("A1"), Piece::of('♖')));
        assert_eq!(game_piece_of("B1", '♘'), (Pos::of("B1"), Piece::of('♘')));
        assert_eq!(game_piece_of("C1", '♗'), (Pos::of("C1"), Piece::of('♗')));
        assert_eq!(game_piece_of("D1", '♕'), (Pos::of("D1"), Piece::of('♕')));
        assert_eq!(game_piece_of("E1", '♔'), (Pos::of("E1"), Piece::of('♔')));
        assert_eq!(game_piece_of("F1", '♗'), (Pos::of("F1"), Piece::of('♗')));
        assert_eq!(game_piece_of("G1", '♘'), (Pos::of("G1"), Piece::of('♘')));
        assert_eq!(game_piece_of("H1", '♖'), (Pos::of("H1"), Piece::of('♖')));
    }
}
