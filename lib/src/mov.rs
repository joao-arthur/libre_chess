use crate::{piece::Piece, pos::{pos_try_of, Pos}};

#[derive(Debug, PartialEq, Clone)]
pub struct Mov {
    pub piece: Piece,
    pub from: Pos,
    pub to: Pos,
}

impl Mov {
    pub fn try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        let piece = Piece::try_of(piece)?;
        let from = pos_try_of(from)?;
        let to = pos_try_of(to)?;
        Some(Mov { piece, from, to })
    }

    pub fn of(piece: char, from: &str, to: &str) -> Self {
        Self::try_of(piece, from, to).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{piece::Piece, pos::pos_of};

    use super::Mov;

    #[test]
    fn try_from_str() {
        assert_eq!(
            Mov::try_of('♟', "D2", "D4"),
            Some(Mov { piece: Piece::of('♟'), from: pos_of("D2"), to: pos_of("D4") })
        );
    }

    #[test]
    fn try_of_str_piece_none() {
        assert_eq!(Mov::try_of('P', "D2", "D4"), None);
    }

    #[test]
    fn try_of_str_piece_from_none() {
        assert_eq!(Mov::try_of('♟', "ZZ9", "D4"), None);
        assert_eq!(Mov::try_of('♟', "A299", "D4"), None);
    }

    #[test]
    fn try_of_str_piece_to_none() {
        assert_eq!(Mov::try_of('♟', "D2", "ZZ9"), None);
        assert_eq!(Mov::try_of('♟', "D4", "A299"), None);
    }

    #[test]
    fn of_str() {
        assert_eq!(
            Mov::of('♟', "D2", "D4"),
            Mov { piece: Piece::of('♟'), from: pos_of("D2"), to: pos_of("D4") }
        );
    }
}
