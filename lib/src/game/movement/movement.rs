use crate::{board::pos::Pos, movement::Movement, piece::Piece};

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultMovement {
    pub movement: Movement,
}

impl From<Movement> for DefaultMovement {
    fn from(movement: Movement) -> Self {
        DefaultMovement { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaptureMovement {
    pub movement: Movement,
}

impl From<Movement> for CaptureMovement {
    fn from(movement: Movement) -> Self {
        CaptureMovement { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnPassantMovement {
    pub movement: Movement,
}

impl From<Movement> for EnPassantMovement {
    fn from(movement: Movement) -> Self {
        EnPassantMovement { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CastlingMovement {
    pub movement: Movement,
}

impl From<Movement> for CastlingMovement {
    fn from(movement: Movement) -> Self {
        CastlingMovement { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionMovement {
    pub pos: Pos,
    pub piece: Piece,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameMovement {
    Default(DefaultMovement),
    Capture(CaptureMovement),
    EnPassant(EnPassantMovement),
    Castling(CastlingMovement),
    Promotion(PromotionMovement),
}

impl From<DefaultMovement> for GameMovement {
    fn from(movement: DefaultMovement) -> Self {
        GameMovement::Default(movement)
    }
}

impl From<EnPassantMovement> for GameMovement {
    fn from(movement: EnPassantMovement) -> Self {
        GameMovement::EnPassant(movement)
    }
}

impl From<CastlingMovement> for GameMovement {
    fn from(movement: CastlingMovement) -> Self {
        GameMovement::Castling(movement)
    }
}

impl From<PromotionMovement> for GameMovement {
    fn from(movement: PromotionMovement) -> Self {
        GameMovement::Promotion(movement)
    }
}
