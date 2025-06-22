use crate::{board::pos::Pos, movement::Movement};

#[derive(Debug, PartialEq)]
pub struct BasicMovement {
    pub movement: Movement,
}

#[derive(Debug, PartialEq)]
pub struct EnPassant {
    pub movement: Movement,
}

#[derive(Debug, PartialEq)]
pub struct Castling {
    pub movement: Movement,
}

#[derive(Debug, PartialEq)]
pub struct Promotion {
    pub movement: Movement,
}

#[derive(Debug, PartialEq)]
pub struct GameMovement {
    pub movement: Movement,
    pub capture: Option<Pos>,
    pub secondary_movement: Option<Movement>,
}

impl From<Movement> for GameMovement {
    fn from(movement: Movement) -> Self {
        GameMovement { movement, capture: None, secondary_movement: None }
    }
}
