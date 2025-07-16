use crate::piece::Piece;

#[derive(Debug, PartialEq, Clone)]
pub struct GameCapture {
    pub piece: Piece,
    pub at: u16,
}
