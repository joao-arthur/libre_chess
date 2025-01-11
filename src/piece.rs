#[derive(Debug, PartialEq, Clone)]
pub enum PieceType {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Piece {
    pub piece: PieceType,
    pub color: PieceColor,
}

impl Piece {
    pub fn from_str(str: &str) -> Option<Piece> {
        match str {
            "♖" => Some(Piece { piece: PieceType::Rook, color: PieceColor::White }),
            "♘" => Some(Piece { piece: PieceType::Knight, color: PieceColor::White }),
            "♗" => Some(Piece { piece: PieceType::Bishop, color: PieceColor::White }),
            "♕" => Some(Piece { piece: PieceType::Queen, color: PieceColor::White }),
            "♔" => Some(Piece { piece: PieceType::King, color: PieceColor::White }),
            "♙" => Some(Piece { piece: PieceType::Pawn, color: PieceColor::White }),
            "♜" => Some(Piece { piece: PieceType::Rook, color: PieceColor::Black }),
            "♞" => Some(Piece { piece: PieceType::Knight, color: PieceColor::Black }),
            "♝" => Some(Piece { piece: PieceType::Bishop, color: PieceColor::Black }),
            "♛" => Some(Piece { piece: PieceType::Queen, color: PieceColor::Black }),
            "♚" => Some(Piece { piece: PieceType::King, color: PieceColor::Black }),
            "♟" => Some(Piece { piece: PieceType::Pawn, color: PieceColor::Black }),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self.color {
            PieceColor::White => match self.piece {
                PieceType::Rook => "♖",
                PieceType::Knight => "♘",
                PieceType::Bishop => "♗",
                PieceType::Queen => "♕",
                PieceType::King => "♔",
                PieceType::Pawn => "♙",
            },
            PieceColor::Black => match self.piece {
                PieceType::Rook => "♜",
                PieceType::Knight => "♞",
                PieceType::Bishop => "♝",
                PieceType::Queen => "♛",
                PieceType::King => "♚",
                PieceType::Pawn => "♟",
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str_some() {
        assert_eq!(Piece::from_str("♖"), Some(Piece { piece: PieceType::Rook, color: PieceColor::White }));
        assert_eq!(Piece::from_str("♘"), Some(Piece { piece: PieceType::Knight, color: PieceColor::White }));
        assert_eq!(Piece::from_str("♗"), Some(Piece { piece: PieceType::Bishop, color: PieceColor::White }));
        assert_eq!(Piece::from_str("♕"), Some(Piece { piece: PieceType::Queen, color: PieceColor::White }));
        assert_eq!(Piece::from_str("♔"), Some(Piece { piece: PieceType::King, color: PieceColor::White }));
        assert_eq!(Piece::from_str("♙"), Some(Piece { piece: PieceType::Pawn, color: PieceColor::White }));
        assert_eq!(Piece::from_str("♜"), Some(Piece { piece: PieceType::Rook, color: PieceColor::Black }));
        assert_eq!(Piece::from_str("♞"), Some(Piece { piece: PieceType::Knight, color: PieceColor::Black }));
        assert_eq!(Piece::from_str("♝"), Some(Piece { piece: PieceType::Bishop, color: PieceColor::Black }));
        assert_eq!(Piece::from_str("♛"), Some(Piece { piece: PieceType::Queen, color: PieceColor::Black }));
        assert_eq!(Piece::from_str("♚"), Some(Piece { piece: PieceType::King, color: PieceColor::Black }));
        assert_eq!(Piece::from_str("♟"), Some(Piece { piece: PieceType::Pawn, color: PieceColor::Black }));
    }

    #[test]
    fn test_from_str_none() {
        assert_eq!(Piece::from_str(""), None);
        assert_eq!(Piece::from_str("R"), None);
        assert_eq!(Piece::from_str("N"), None);
        assert_eq!(Piece::from_str("B"), None);
        assert_eq!(Piece::from_str("Q"), None);
        assert_eq!(Piece::from_str("K"), None);
        assert_eq!(Piece::from_str("P"), None);
    }

    #[test]
    fn test_to_str() {
        assert_eq!(Piece::from_str("♖").map(|p| p.to_str()), Some("♖"));
        assert_eq!(Piece::from_str("♘").map(|p| p.to_str()), Some("♘"));
        assert_eq!(Piece::from_str("♗").map(|p| p.to_str()), Some("♗"));
        assert_eq!(Piece::from_str("♕").map(|p| p.to_str()), Some("♕"));
        assert_eq!(Piece::from_str("♔").map(|p| p.to_str()), Some("♔"));
        assert_eq!(Piece::from_str("♙").map(|p| p.to_str()), Some("♙"));
        assert_eq!(Piece::from_str("♜").map(|p| p.to_str()), Some("♜"));
        assert_eq!(Piece::from_str("♞").map(|p| p.to_str()), Some("♞"));
        assert_eq!(Piece::from_str("♝").map(|p| p.to_str()), Some("♝"));
        assert_eq!(Piece::from_str("♛").map(|p| p.to_str()), Some("♛"));
        assert_eq!(Piece::from_str("♚").map(|p| p.to_str()), Some("♚"));
        assert_eq!(Piece::from_str("♟").map(|p| p.to_str()), Some("♟"));
    }
}
