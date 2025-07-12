use std::fmt;

use crate::color::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PieceType {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Piece {
    pub typ: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn try_of(c: char) -> Option<Self> {
        match c {
            '♖' => Some(Piece { typ: PieceType::Rook, color: Color::White }),
            '♘' => Some(Piece { typ: PieceType::Knight, color: Color::White }),
            '♗' => Some(Piece { typ: PieceType::Bishop, color: Color::White }),
            '♕' => Some(Piece { typ: PieceType::Queen, color: Color::White }),
            '♔' => Some(Piece { typ: PieceType::King, color: Color::White }),
            '♙' => Some(Piece { typ: PieceType::Pawn, color: Color::White }),
            '♜' => Some(Piece { typ: PieceType::Rook, color: Color::Black }),
            '♞' => Some(Piece { typ: PieceType::Knight, color: Color::Black }),
            '♝' => Some(Piece { typ: PieceType::Bishop, color: Color::Black }),
            '♛' => Some(Piece { typ: PieceType::Queen, color: Color::Black }),
            '♚' => Some(Piece { typ: PieceType::King, color: Color::Black }),
            '♟' => Some(Piece { typ: PieceType::Pawn, color: Color::Black }),
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
                Color::White => match self.typ {
                    PieceType::Rook => '♖',
                    PieceType::Knight => '♘',
                    PieceType::Bishop => '♗',
                    PieceType::Queen => '♕',
                    PieceType::King => '♔',
                    PieceType::Pawn => '♙',
                },
                Color::Black => match self.typ {
                    PieceType::Rook => '♜',
                    PieceType::Knight => '♞',
                    PieceType::Bishop => '♝',
                    PieceType::Queen => '♛',
                    PieceType::King => '♚',
                    PieceType::Pawn => '♟',
                },
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Piece, PieceType};

    #[test]
    fn pirece_try_of_str_some() {
        assert_eq!(Piece::try_of('♖'), Some(Piece { typ: PieceType::Rook, color: Color::White }));
        assert_eq!(Piece::try_of('♘'), Some(Piece { typ: PieceType::Knight, color: Color::White }));
        assert_eq!(Piece::try_of('♗'), Some(Piece { typ: PieceType::Bishop, color: Color::White }));
        assert_eq!(Piece::try_of('♕'), Some(Piece { typ: PieceType::Queen, color: Color::White }));
        assert_eq!(Piece::try_of('♔'), Some(Piece { typ: PieceType::King, color: Color::White }));
        assert_eq!(Piece::try_of('♙'), Some(Piece { typ: PieceType::Pawn, color: Color::White }));
        assert_eq!(Piece::try_of('♜'), Some(Piece { typ: PieceType::Rook, color: Color::Black }));
        assert_eq!(Piece::try_of('♞'), Some(Piece { typ: PieceType::Knight, color: Color::Black }));
        assert_eq!(Piece::try_of('♝'), Some(Piece { typ: PieceType::Bishop, color: Color::Black }));
        assert_eq!(Piece::try_of('♛'), Some(Piece { typ: PieceType::Queen, color: Color::Black }));
        assert_eq!(Piece::try_of('♚'), Some(Piece { typ: PieceType::King, color: Color::Black }));
        assert_eq!(Piece::try_of('♟'), Some(Piece { typ: PieceType::Pawn, color: Color::Black }));
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
        assert_eq!(Piece::of('♖'), Piece { typ: PieceType::Rook, color: Color::White });
        assert_eq!(Piece::of('♘'), Piece { typ: PieceType::Knight, color: Color::White });
        assert_eq!(Piece::of('♗'), Piece { typ: PieceType::Bishop, color: Color::White });
        assert_eq!(Piece::of('♕'), Piece { typ: PieceType::Queen, color: Color::White });
        assert_eq!(Piece::of('♔'), Piece { typ: PieceType::King, color: Color::White });
        assert_eq!(Piece::of('♙'), Piece { typ: PieceType::Pawn, color: Color::White });
        assert_eq!(Piece::of('♜'), Piece { typ: PieceType::Rook, color: Color::Black });
        assert_eq!(Piece::of('♞'), Piece { typ: PieceType::Knight, color: Color::Black });
        assert_eq!(Piece::of('♝'), Piece { typ: PieceType::Bishop, color: Color::Black });
        assert_eq!(Piece::of('♛'), Piece { typ: PieceType::Queen, color: Color::Black });
        assert_eq!(Piece::of('♚'), Piece { typ: PieceType::King, color: Color::Black });
        assert_eq!(Piece::of('♟'), Piece { typ: PieceType::Pawn, color: Color::Black });
    }

    #[test]
    fn to_string() {
        assert_eq!(Piece { typ: PieceType::Rook, color: Color::White }.to_string(), "♖");
        assert_eq!(Piece { typ: PieceType::Knight, color: Color::White }.to_string(), "♘");
        assert_eq!(Piece { typ: PieceType::Bishop, color: Color::White }.to_string(), "♗");
        assert_eq!(Piece { typ: PieceType::Queen, color: Color::White }.to_string(), "♕");
        assert_eq!(Piece { typ: PieceType::King, color: Color::White }.to_string(), "♔");
        assert_eq!(Piece { typ: PieceType::Pawn, color: Color::White }.to_string(), "♙");
        assert_eq!(Piece { typ: PieceType::Rook, color: Color::Black }.to_string(), "♜");
        assert_eq!(Piece { typ: PieceType::Knight, color: Color::Black }.to_string(), "♞");
        assert_eq!(Piece { typ: PieceType::Bishop, color: Color::Black }.to_string(), "♝");
        assert_eq!(Piece { typ: PieceType::Queen, color: Color::Black }.to_string(), "♛");
        assert_eq!(Piece { typ: PieceType::King, color: Color::Black }.to_string(), "♚");
        assert_eq!(Piece { typ: PieceType::Pawn, color: Color::Black }.to_string(), "♟");
    }
}
