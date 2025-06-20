use std::collections::HashSet;

use crate::{board::pos::Pos, color::Color};

use super::capture::Capture;

#[derive(Debug, PartialEq)]
pub struct Player {
    pub color: Color,
    pub captures: Vec<Capture>,
    pub menace: HashSet<Pos>,
}
