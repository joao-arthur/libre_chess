use crate::{board::pos::Pos, movement::Movement, piece::Piece};

#[derive(Debug, PartialEq)]
pub struct DefaultMovement {
    pub movement: Movement,
}

#[derive(Debug, PartialEq)]
pub struct EnPassantMovement {
    pub movement: Movement,
}

#[derive(Debug, PartialEq)]
pub struct CastlingMovement {
    pub movement: Movement,
}

#[derive(Debug, PartialEq)]
pub struct PromotionMovement {
    pub piece: Piece,
}

pub enum GameMovement {
    Default(DefaultMovement),
    EnPassant(EnPassantMovement),
    Castling(CastlingMovement),
    Promotion(PromotionMovement),
}

#[derive(Debug, PartialEq)]
pub struct GameMovementOld {
    pub movement: Movement,
    pub capture: Option<Pos>,
    pub secondary_movement: Option<Movement>,
}

impl From<Movement> for GameMovementOld {
    fn from(movement: Movement) -> Self {
        GameMovementOld { movement, capture: None, secondary_movement: None }
    }
}
