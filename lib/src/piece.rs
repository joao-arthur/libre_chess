use std::fmt;

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
    pub fn try_of(c: char) -> Option<Self> {
        match c {
            '♖' => Some(Piece { t: Type::Rook, color: Color::White }),
            '♘' => Some(Piece { t: Type::Knight, color: Color::White }),
            '♗' => Some(Piece { t: Type::Bishop, color: Color::White }),
            '♕' => Some(Piece { t: Type::Queen, color: Color::White }),
            '♔' => Some(Piece { t: Type::King, color: Color::White }),
            '♙' => Some(Piece { t: Type::Pawn, color: Color::White }),
            '♜' => Some(Piece { t: Type::Rook, color: Color::Black }),
            '♞' => Some(Piece { t: Type::Knight, color: Color::Black }),
            '♝' => Some(Piece { t: Type::Bishop, color: Color::Black }),
            '♛' => Some(Piece { t: Type::Queen, color: Color::Black }),
            '♚' => Some(Piece { t: Type::King, color: Color::Black }),
            '♟' => Some(Piece { t: Type::Pawn, color: Color::Black }),
            _ => None,
        }
    }

    pub fn of(s: char) -> Self {
        Self::try_of(s).unwrap()
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.color {
                Color::White => match self.t {
                    Type::Rook => '♖',
                    Type::Knight => '♘',
                    Type::Bishop => '♗',
                    Type::Queen => '♕',
                    Type::King => '♔',
                    Type::Pawn => '♙',
                },
                Color::Black => match self.t {
                    Type::Rook => '♜',
                    Type::Knight => '♞',
                    Type::Bishop => '♝',
                    Type::Queen => '♛',
                    Type::King => '♚',
                    Type::Pawn => '♟',
                },
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Piece, Type};

    #[test]
    fn pirece_try_of_str_some() {
        assert_eq!(Piece::try_of('♖'), Some(Piece { t: Type::Rook, color: Color::White }));
        assert_eq!(Piece::try_of('♘'), Some(Piece { t: Type::Knight, color: Color::White }));
        assert_eq!(Piece::try_of('♗'), Some(Piece { t: Type::Bishop, color: Color::White }));
        assert_eq!(Piece::try_of('♕'), Some(Piece { t: Type::Queen, color: Color::White }));
        assert_eq!(Piece::try_of('♔'), Some(Piece { t: Type::King, color: Color::White }));
        assert_eq!(Piece::try_of('♙'), Some(Piece { t: Type::Pawn, color: Color::White }));
        assert_eq!(Piece::try_of('♜'), Some(Piece { t: Type::Rook, color: Color::Black }));
        assert_eq!(Piece::try_of('♞'), Some(Piece { t: Type::Knight, color: Color::Black }));
        assert_eq!(Piece::try_of('♝'), Some(Piece { t: Type::Bishop, color: Color::Black }));
        assert_eq!(Piece::try_of('♛'), Some(Piece { t: Type::Queen, color: Color::Black }));
        assert_eq!(Piece::try_of('♚'), Some(Piece { t: Type::King, color: Color::Black }));
        assert_eq!(Piece::try_of('♟'), Some(Piece { t: Type::Pawn, color: Color::Black }));
    }

    #[test]
    fn try_of_str_none() {
        assert_eq!(Piece::try_of(' '), None);
        assert_eq!(Piece::try_of('R'), None);
        assert_eq!(Piece::try_of('N'), None);
        assert_eq!(Piece::try_of('B'), None);
        assert_eq!(Piece::try_of('Q'), None);
        assert_eq!(Piece::try_of('K'), None);
        assert_eq!(Piece::try_of('P'), None);
    }

    #[test]
    fn of_str() {
        assert_eq!(Piece::of('♖'), Piece { t: Type::Rook, color: Color::White });
        assert_eq!(Piece::of('♘'), Piece { t: Type::Knight, color: Color::White });
        assert_eq!(Piece::of('♗'), Piece { t: Type::Bishop, color: Color::White });
        assert_eq!(Piece::of('♕'), Piece { t: Type::Queen, color: Color::White });
        assert_eq!(Piece::of('♔'), Piece { t: Type::King, color: Color::White });
        assert_eq!(Piece::of('♙'), Piece { t: Type::Pawn, color: Color::White });
        assert_eq!(Piece::of('♜'), Piece { t: Type::Rook, color: Color::Black });
        assert_eq!(Piece::of('♞'), Piece { t: Type::Knight, color: Color::Black });
        assert_eq!(Piece::of('♝'), Piece { t: Type::Bishop, color: Color::Black });
        assert_eq!(Piece::of('♛'), Piece { t: Type::Queen, color: Color::Black });
        assert_eq!(Piece::of('♚'), Piece { t: Type::King, color: Color::Black });
        assert_eq!(Piece::of('♟'), Piece { t: Type::Pawn, color: Color::Black });
    }

    #[test]
    fn to_string() {
        assert_eq!(Piece { t: Type::Rook, color: Color::White }.to_string(), "♖");
        assert_eq!(Piece { t: Type::Knight, color: Color::White }.to_string(), "♘");
        assert_eq!(Piece { t: Type::Bishop, color: Color::White }.to_string(), "♗");
        assert_eq!(Piece { t: Type::Queen, color: Color::White }.to_string(), "♕");
        assert_eq!(Piece { t: Type::King, color: Color::White }.to_string(), "♔");
        assert_eq!(Piece { t: Type::Pawn, color: Color::White }.to_string(), "♙");
        assert_eq!(Piece { t: Type::Rook, color: Color::Black }.to_string(), "♜");
        assert_eq!(Piece { t: Type::Knight, color: Color::Black }.to_string(), "♞");
        assert_eq!(Piece { t: Type::Bishop, color: Color::Black }.to_string(), "♝");
        assert_eq!(Piece { t: Type::Queen, color: Color::Black }.to_string(), "♛");
        assert_eq!(Piece { t: Type::King, color: Color::Black }.to_string(), "♚");
        assert_eq!(Piece { t: Type::Pawn, color: Color::Black }.to_string(), "♟");
    }
}
