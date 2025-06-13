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
    pub color: Color,
}

impl Piece {
    pub fn try_of_str(s: &str) -> Option<Self> {
        match s {
            "♖" => Some(Piece { t: Type::Rook, color: Color::White }),
            "♘" => Some(Piece { t: Type::Knight, color: Color::White }),
            "♗" => Some(Piece { t: Type::Bishop, color: Color::White }),
            "♕" => Some(Piece { t: Type::Queen, color: Color::White }),
            "♔" => Some(Piece { t: Type::King, color: Color::White }),
            "♙" => Some(Piece { t: Type::Pawn, color: Color::White }),
            "♜" => Some(Piece { t: Type::Rook, color: Color::Black }),
            "♞" => Some(Piece { t: Type::Knight, color: Color::Black }),
            "♝" => Some(Piece { t: Type::Bishop, color: Color::Black }),
            "♛" => Some(Piece { t: Type::Queen, color: Color::Black }),
            "♚" => Some(Piece { t: Type::King, color: Color::Black }),
            "♟" => Some(Piece { t: Type::Pawn, color: Color::Black }),
            _ => None,
        }
    }

    pub fn of_str(s: &str) -> Self {
        Self::try_of_str(s).unwrap()
    }

    pub fn to_str(&self) -> &'static str {
        match self.color {
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
mod tests {
    use super::{Color, Piece, Type};

    #[test]
    fn pirece_try_of_str_some() {
        assert_eq!(Piece::try_of_str("♖"), Some(Piece { t: Type::Rook, color: Color::White }));
        assert_eq!(Piece::try_of_str("♘"), Some(Piece { t: Type::Knight, color: Color::White }));
        assert_eq!(Piece::try_of_str("♗"), Some(Piece { t: Type::Bishop, color: Color::White }));
        assert_eq!(Piece::try_of_str("♕"), Some(Piece { t: Type::Queen, color: Color::White }));
        assert_eq!(Piece::try_of_str("♔"), Some(Piece { t: Type::King, color: Color::White }));
        assert_eq!(Piece::try_of_str("♙"), Some(Piece { t: Type::Pawn, color: Color::White }));
        assert_eq!(Piece::try_of_str("♜"), Some(Piece { t: Type::Rook, color: Color::Black }));
        assert_eq!(Piece::try_of_str("♞"), Some(Piece { t: Type::Knight, color: Color::Black }));
        assert_eq!(Piece::try_of_str("♝"), Some(Piece { t: Type::Bishop, color: Color::Black }));
        assert_eq!(Piece::try_of_str("♛"), Some(Piece { t: Type::Queen, color: Color::Black }));
        assert_eq!(Piece::try_of_str("♚"), Some(Piece { t: Type::King, color: Color::Black }));
        assert_eq!(Piece::try_of_str("♟"), Some(Piece { t: Type::Pawn, color: Color::Black }));
    }

    #[test]
    fn try_of_str_none() {
        assert_eq!(Piece::try_of_str(""), None);
        assert_eq!(Piece::try_of_str("R"), None);
        assert_eq!(Piece::try_of_str("N"), None);
        assert_eq!(Piece::try_of_str("B"), None);
        assert_eq!(Piece::try_of_str("Q"), None);
        assert_eq!(Piece::try_of_str("K"), None);
        assert_eq!(Piece::try_of_str("P"), None);
    }

    #[test]
    fn of_str() {
        assert_eq!(Piece::of_str("♖"), Piece { t: Type::Rook, color: Color::White });
        assert_eq!(Piece::of_str("♘"), Piece { t: Type::Knight, color: Color::White });
        assert_eq!(Piece::of_str("♗"), Piece { t: Type::Bishop, color: Color::White });
        assert_eq!(Piece::of_str("♕"), Piece { t: Type::Queen, color: Color::White });
        assert_eq!(Piece::of_str("♔"), Piece { t: Type::King, color: Color::White });
        assert_eq!(Piece::of_str("♙"), Piece { t: Type::Pawn, color: Color::White });
        assert_eq!(Piece::of_str("♜"), Piece { t: Type::Rook, color: Color::Black });
        assert_eq!(Piece::of_str("♞"), Piece { t: Type::Knight, color: Color::Black });
        assert_eq!(Piece::of_str("♝"), Piece { t: Type::Bishop, color: Color::Black });
        assert_eq!(Piece::of_str("♛"), Piece { t: Type::Queen, color: Color::Black });
        assert_eq!(Piece::of_str("♚"), Piece { t: Type::King, color: Color::Black });
        assert_eq!(Piece::of_str("♟"), Piece { t: Type::Pawn, color: Color::Black });
    }

    #[test]
    fn to_str() {
        assert_eq!(Piece { t: Type::Rook, color: Color::White }.to_str(), "♖");
        assert_eq!(Piece { t: Type::Knight, color: Color::White }.to_str(), "♘");
        assert_eq!(Piece { t: Type::Bishop, color: Color::White }.to_str(), "♗");
        assert_eq!(Piece { t: Type::Queen, color: Color::White }.to_str(), "♕");
        assert_eq!(Piece { t: Type::King, color: Color::White }.to_str(), "♔");
        assert_eq!(Piece { t: Type::Pawn, color: Color::White }.to_str(), "♙");
        assert_eq!(Piece { t: Type::Rook, color: Color::Black }.to_str(), "♜");
        assert_eq!(Piece { t: Type::Knight, color: Color::Black }.to_str(), "♞");
        assert_eq!(Piece { t: Type::Bishop, color: Color::Black }.to_str(), "♝");
        assert_eq!(Piece { t: Type::Queen, color: Color::Black }.to_str(), "♛");
        assert_eq!(Piece { t: Type::King, color: Color::Black }.to_str(), "♚");
        assert_eq!(Piece { t: Type::Pawn, color: Color::Black }.to_str(), "♟");
    }
}
