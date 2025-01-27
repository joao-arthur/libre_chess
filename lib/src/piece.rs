use crate::color::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Type {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Piece {
    pub t: Type,
    pub c: Color,
}

impl Piece {
    pub fn try_of_str(s: &str) -> Option<Self> {
        match s {
            "♖" => Some(Piece { t: Type::Rook, c: Color::White }),
            "♘" => Some(Piece { t: Type::Knight, c: Color::White }),
            "♗" => Some(Piece { t: Type::Bishop, c: Color::White }),
            "♕" => Some(Piece { t: Type::Queen, c: Color::White }),
            "♔" => Some(Piece { t: Type::King, c: Color::White }),
            "♙" => Some(Piece { t: Type::Pawn, c: Color::White }),
            "♜" => Some(Piece { t: Type::Rook, c: Color::Black }),
            "♞" => Some(Piece { t: Type::Knight, c: Color::Black }),
            "♝" => Some(Piece { t: Type::Bishop, c: Color::Black }),
            "♛" => Some(Piece { t: Type::Queen, c: Color::Black }),
            "♚" => Some(Piece { t: Type::King, c: Color::Black }),
            "♟" => Some(Piece { t: Type::Pawn, c: Color::Black }),
            _ => None,
        }
    }

    pub fn of_str(s: &str) -> Self {
        Self::try_of_str(s).unwrap()
    }

    pub fn to_str(&self) -> &'static str {
        match self.c {
            Color::White => match self.t {
                Type::Rook => "♖",
                Type::Knight => "♘",
                Type::Bishop => "♗",
                Type::Queen => "♕",
                Type::King => "♔",
                Type::Pawn => "♙",
            },
            Color::Black => match self.t {
                Type::Rook => "♜",
                Type::Knight => "♞",
                Type::Bishop => "♝",
                Type::Queen => "♛",
                Type::King => "♚",
                Type::Pawn => "♟",
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_of_str_some() {
        assert_eq!(Piece::try_of_str("♖"), Some(Piece { t: Type::Rook, c: Color::White }));
        assert_eq!(Piece::try_of_str("♘"), Some(Piece { t: Type::Knight, c: Color::White }));
        assert_eq!(Piece::try_of_str("♗"), Some(Piece { t: Type::Bishop, c: Color::White }));
        assert_eq!(Piece::try_of_str("♕"), Some(Piece { t: Type::Queen, c: Color::White }));
        assert_eq!(Piece::try_of_str("♔"), Some(Piece { t: Type::King, c: Color::White }));
        assert_eq!(Piece::try_of_str("♙"), Some(Piece { t: Type::Pawn, c: Color::White }));
        assert_eq!(Piece::try_of_str("♜"), Some(Piece { t: Type::Rook, c: Color::Black }));
        assert_eq!(Piece::try_of_str("♞"), Some(Piece { t: Type::Knight, c: Color::Black }));
        assert_eq!(Piece::try_of_str("♝"), Some(Piece { t: Type::Bishop, c: Color::Black }));
        assert_eq!(Piece::try_of_str("♛"), Some(Piece { t: Type::Queen, c: Color::Black }));
        assert_eq!(Piece::try_of_str("♚"), Some(Piece { t: Type::King, c: Color::Black }));
        assert_eq!(Piece::try_of_str("♟"), Some(Piece { t: Type::Pawn, c: Color::Black }));
    }

    #[test]
    fn test_try_of_str_none() {
        assert_eq!(Piece::try_of_str(""), None);
        assert_eq!(Piece::try_of_str("R"), None);
        assert_eq!(Piece::try_of_str("N"), None);
        assert_eq!(Piece::try_of_str("B"), None);
        assert_eq!(Piece::try_of_str("Q"), None);
        assert_eq!(Piece::try_of_str("K"), None);
        assert_eq!(Piece::try_of_str("P"), None);
    }

    #[test]
    fn test_of_str() {
        assert_eq!(Piece::of_str("♖"), Piece { t: Type::Rook, c: Color::White });
        assert_eq!(Piece::of_str("♘"), Piece { t: Type::Knight, c: Color::White });
        assert_eq!(Piece::of_str("♗"), Piece { t: Type::Bishop, c: Color::White });
        assert_eq!(Piece::of_str("♕"), Piece { t: Type::Queen, c: Color::White });
        assert_eq!(Piece::of_str("♔"), Piece { t: Type::King, c: Color::White });
        assert_eq!(Piece::of_str("♙"), Piece { t: Type::Pawn, c: Color::White });
        assert_eq!(Piece::of_str("♜"), Piece { t: Type::Rook, c: Color::Black });
        assert_eq!(Piece::of_str("♞"), Piece { t: Type::Knight, c: Color::Black });
        assert_eq!(Piece::of_str("♝"), Piece { t: Type::Bishop, c: Color::Black });
        assert_eq!(Piece::of_str("♛"), Piece { t: Type::Queen, c: Color::Black });
        assert_eq!(Piece::of_str("♚"), Piece { t: Type::King, c: Color::Black });
        assert_eq!(Piece::of_str("♟"), Piece { t: Type::Pawn, c: Color::Black });
    }

    #[test]
    fn test_to_str() {
        assert_eq!(Piece { t: Type::Rook, c: Color::White }.to_str(), "♖");
        assert_eq!(Piece { t: Type::Knight, c: Color::White }.to_str(), "♘");
        assert_eq!(Piece { t: Type::Bishop, c: Color::White }.to_str(), "♗");
        assert_eq!(Piece { t: Type::Queen, c: Color::White }.to_str(), "♕");
        assert_eq!(Piece { t: Type::King, c: Color::White }.to_str(), "♔");
        assert_eq!(Piece { t: Type::Pawn, c: Color::White }.to_str(), "♙");
        assert_eq!(Piece { t: Type::Rook, c: Color::Black }.to_str(), "♜");
        assert_eq!(Piece { t: Type::Knight, c: Color::Black }.to_str(), "♞");
        assert_eq!(Piece { t: Type::Bishop, c: Color::Black }.to_str(), "♝");
        assert_eq!(Piece { t: Type::Queen, c: Color::Black }.to_str(), "♛");
        assert_eq!(Piece { t: Type::King, c: Color::Black }.to_str(), "♚");
        assert_eq!(Piece { t: Type::Pawn, c: Color::Black }.to_str(), "♟");
    }
}
