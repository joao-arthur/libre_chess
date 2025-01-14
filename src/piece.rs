#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    pub t: Type,
    pub c: Color,
}

pub fn of_str(s: &str) -> Option<Piece> {
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

pub fn to_str(p: &Piece) -> &'static str {
    match p.c {
        Color::White => match p.t {
            Type::Rook => "♖",
            Type::Knight => "♘",
            Type::Bishop => "♗",
            Type::Queen => "♕",
            Type::King => "♔",
            Type::Pawn => "♙",
        },
        Color::Black => match p.t {
            Type::Rook => "♜",
            Type::Knight => "♞",
            Type::Bishop => "♝",
            Type::Queen => "♛",
            Type::King => "♚",
            Type::Pawn => "♟",
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str_some() {
        assert_eq!(of_str("♖"), Some(Piece { t: Type::Rook, c: Color::White }));
        assert_eq!(of_str("♘"), Some(Piece { t: Type::Knight, c: Color::White }));
        assert_eq!(of_str("♗"), Some(Piece { t: Type::Bishop, c: Color::White }));
        assert_eq!(of_str("♕"), Some(Piece { t: Type::Queen, c: Color::White }));
        assert_eq!(of_str("♔"), Some(Piece { t: Type::King, c: Color::White }));
        assert_eq!(of_str("♙"), Some(Piece { t: Type::Pawn, c: Color::White }));
        assert_eq!(of_str("♜"), Some(Piece { t: Type::Rook, c: Color::Black }));
        assert_eq!(of_str("♞"), Some(Piece { t: Type::Knight, c: Color::Black }));
        assert_eq!(of_str("♝"), Some(Piece { t: Type::Bishop, c: Color::Black }));
        assert_eq!(of_str("♛"), Some(Piece { t: Type::Queen, c: Color::Black }));
        assert_eq!(of_str("♚"), Some(Piece { t: Type::King, c: Color::Black }));
        assert_eq!(of_str("♟"), Some(Piece { t: Type::Pawn, c: Color::Black }));
    }

    #[test]
    fn test_from_str_none() {
        assert_eq!(of_str(""), None);
        assert_eq!(of_str("R"), None);
        assert_eq!(of_str("N"), None);
        assert_eq!(of_str("B"), None);
        assert_eq!(of_str("Q"), None);
        assert_eq!(of_str("K"), None);
        assert_eq!(of_str("P"), None);
    }

    #[test]
    fn test_to_str() {
        assert_eq!(to_str(&Piece { t: Type::Rook, c: Color::White }), "♖");
        assert_eq!(to_str(&Piece { t: Type::Knight, c: Color::White }), "♘");
        assert_eq!(to_str(&Piece { t: Type::Bishop, c: Color::White }), "♗");
        assert_eq!(to_str(&Piece { t: Type::Queen, c: Color::White }), "♕");
        assert_eq!(to_str(&Piece { t: Type::King, c: Color::White }), "♔");
        assert_eq!(to_str(&Piece { t: Type::Pawn, c: Color::White }), "♙");
        assert_eq!(to_str(&Piece { t: Type::Rook, c: Color::Black }), "♜");
        assert_eq!(to_str(&Piece { t: Type::Knight, c: Color::Black }), "♞");
        assert_eq!(to_str(&Piece { t: Type::Bishop, c: Color::Black }), "♝");
        assert_eq!(to_str(&Piece { t: Type::Queen, c: Color::Black }), "♛");
        assert_eq!(to_str(&Piece { t: Type::King, c: Color::Black }), "♚");
        assert_eq!(to_str(&Piece { t: Type::Pawn, c: Color::Black }), "♟");
    }
}
