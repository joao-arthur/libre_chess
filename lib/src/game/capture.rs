use crate::piece::Piece;

#[derive(Debug, PartialEq)]
pub struct Capture {
    pub piece: Piece,
    pub at: u16,
}
