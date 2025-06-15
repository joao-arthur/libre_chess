use crate::{board::pos::Pos, piece::Piece};

mod bishop;
mod king;
mod knight;
pub mod naive;
mod pawn;
mod queen;
mod rook;

#[derive(Debug, PartialEq, Clone)]
pub struct Movement {
    pub piece: Piece,
    pub from: Pos,
    pub to: Pos,
}

impl Movement {
    pub fn try_of_str(piece: &str, from: &str, to: &str) -> Option<Self> {
        let piece = Piece::try_of_str(piece)?;
        let from = Pos::try_of_str(from)?;
        let to = Pos::try_of_str(to)?;
        Some(Movement { piece, from, to })
    }

    pub fn of_str(piece: &str, from: &str, to: &str) -> Self {
        Self::try_of_str(piece, from, to).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::pos::Pos, piece::Piece};

    use super::Movement;

    #[test]
    fn try_from_str() {
        assert_eq!(
            Movement::try_of_str("♟", "D2", "D4"),
            Some(Movement {
                piece: Piece::of_str("♟"),
                from: Pos::of_str("D2"),
                to: Pos::of_str("D4")
            })
        );
    }

    #[test]
    fn try_of_str_none() {
        assert_eq!(Movement::try_of_str("P", "D2", "D4"), None);
        assert_eq!(Movement::try_of_str("♟", "ZZ9", "D4"), None);
        assert_eq!(Movement::try_of_str("♟", "D2", "ZZ9"), None);
    }

    #[test]
    fn of_str() {
        assert_eq!(
            Movement::of_str("♟", "D2", "D4"),
            Movement {
                piece: Piece::of_str("♟"), from: Pos::of_str("D2"), to: Pos::of_str("D4")
            }
        );
    }
}
