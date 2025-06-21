use crate::piece::Piece;

#[derive(Debug, PartialEq)]
pub struct GameCapture {
    pub piece: Piece,
    pub at: u16,
}
