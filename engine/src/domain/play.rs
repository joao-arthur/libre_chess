use crate::domain::{board::BoardPos, piece::Piece};

struct PlayMove {
    piece: Piece,
    from: BoardPos,
    to: BoardPos
}

struct Play {
    moves: PlayMove
}
