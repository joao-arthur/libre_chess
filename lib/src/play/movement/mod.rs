use crate::{board::pos::Pos, piece::Piece};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

#[derive(Debug, PartialEq, Clone)]
pub struct Movement {
    piece: Piece,
    from: Pos,
    to: Pos,
}

impl Movement {
    pub fn try_of_str(piece: &str, from: &str, to: &str) -> Option<Self> {
        let piece = Piece::try_of_str(piece)?;
        let from = Pos::try_of_str(from)?;
        let to = Pos::try_of_str(to)?;
        Some(Movement { piece, from, to })
    }

    pub fn of_str(p: &str, from: &str, to: &str) -> Self {
        Self::try_of_str(p, from, to).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_play_move_try_of_str_some() {
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
    fn test_play_move_try_of_str_none() {
        assert_eq!(Movement::try_of_str("P", "D2", "D4"), None);
        assert_eq!(Movement::try_of_str("♟", "K9", "D4"), None);
        assert_eq!(Movement::try_of_str("♟", "D2", "K9"), None);
    }

    #[test]
    fn test_play_move_of_str() {
        assert_eq!(
            Movement::of_str("♟", "D2", "D4"),
            Movement {
                piece: Piece::of_str("♟"), from: Pos::of_str("D2"), to: Pos::of_str("D4")
            }
        );
    }
}
