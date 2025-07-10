use std::fmt;

use crate::color::PieceColor;

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
    pub t: PieceType,
    pub color: PieceColor,
}

impl Piece {
    pub fn try_of(c: char) -> Option<Self> {
        match c {
            '♖' => Some(Piece { t: PieceType::Rook, color: PieceColor::White }),
            '♘' => Some(Piece { t: PieceType::Knight, color: PieceColor::White }),
            '♗' => Some(Piece { t: PieceType::Bishop, color: PieceColor::White }),
            '♕' => Some(Piece { t: PieceType::Queen, color: PieceColor::White }),
            '♔' => Some(Piece { t: PieceType::King, color: PieceColor::White }),
            '♙' => Some(Piece { t: PieceType::Pawn, color: PieceColor::White }),
            '♜' => Some(Piece { t: PieceType::Rook, color: PieceColor::Black }),
            '♞' => Some(Piece { t: PieceType::Knight, color: PieceColor::Black }),
            '♝' => Some(Piece { t: PieceType::Bishop, color: PieceColor::Black }),
            '♛' => Some(Piece { t: PieceType::Queen, color: PieceColor::Black }),
            '♚' => Some(Piece { t: PieceType::King, color: PieceColor::Black }),
            '♟' => Some(Piece { t: PieceType::Pawn, color: PieceColor::Black }),
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
                PieceColor::White => match self.t {
                    PieceType::Rook => '♖',
                    PieceType::Knight => '♘',
                    PieceType::Bishop => '♗',
                    PieceType::Queen => '♕',
                    PieceType::King => '♔',
                    PieceType::Pawn => '♙',
                },
                PieceColor::Black => match self.t {
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
    use super::{Piece, PieceColor, PieceType};

    #[test]
    fn pirece_try_of_str_some() {
        assert_eq!(
            Piece::try_of('♖'),
            Some(Piece { t: PieceType::Rook, color: PieceColor::White })
        );
        assert_eq!(
            Piece::try_of('♘'),
            Some(Piece { t: PieceType::Knight, color: PieceColor::White })
        );
        assert_eq!(
            Piece::try_of('♗'),
            Some(Piece { t: PieceType::Bishop, color: PieceColor::White })
        );
        assert_eq!(
            Piece::try_of('♕'),
            Some(Piece { t: PieceType::Queen, color: PieceColor::White })
        );
        assert_eq!(
            Piece::try_of('♔'),
            Some(Piece { t: PieceType::King, color: PieceColor::White })
        );
        assert_eq!(
            Piece::try_of('♙'),
            Some(Piece { t: PieceType::Pawn, color: PieceColor::White })
        );
        assert_eq!(
            Piece::try_of('♜'),
            Some(Piece { t: PieceType::Rook, color: PieceColor::Black })
        );
        assert_eq!(
            Piece::try_of('♞'),
            Some(Piece { t: PieceType::Knight, color: PieceColor::Black })
        );
        assert_eq!(
            Piece::try_of('♝'),
            Some(Piece { t: PieceType::Bishop, color: PieceColor::Black })
        );
        assert_eq!(
            Piece::try_of('♛'),
            Some(Piece { t: PieceType::Queen, color: PieceColor::Black })
        );
        assert_eq!(
            Piece::try_of('♚'),
            Some(Piece { t: PieceType::King, color: PieceColor::Black })
        );
        assert_eq!(
            Piece::try_of('♟'),
            Some(Piece { t: PieceType::Pawn, color: PieceColor::Black })
        );
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
        assert_eq!(Piece::of('♖'), Piece { t: PieceType::Rook, color: PieceColor::White });
        assert_eq!(Piece::of('♘'), Piece { t: PieceType::Knight, color: PieceColor::White });
        assert_eq!(Piece::of('♗'), Piece { t: PieceType::Bishop, color: PieceColor::White });
        assert_eq!(Piece::of('♕'), Piece { t: PieceType::Queen, color: PieceColor::White });
        assert_eq!(Piece::of('♔'), Piece { t: PieceType::King, color: PieceColor::White });
        assert_eq!(Piece::of('♙'), Piece { t: PieceType::Pawn, color: PieceColor::White });
        assert_eq!(Piece::of('♜'), Piece { t: PieceType::Rook, color: PieceColor::Black });
        assert_eq!(Piece::of('♞'), Piece { t: PieceType::Knight, color: PieceColor::Black });
        assert_eq!(Piece::of('♝'), Piece { t: PieceType::Bishop, color: PieceColor::Black });
        assert_eq!(Piece::of('♛'), Piece { t: PieceType::Queen, color: PieceColor::Black });
        assert_eq!(Piece::of('♚'), Piece { t: PieceType::King, color: PieceColor::Black });
        assert_eq!(Piece::of('♟'), Piece { t: PieceType::Pawn, color: PieceColor::Black });
    }

    #[test]
    fn to_string() {
        assert_eq!(Piece { t: PieceType::Rook, color: PieceColor::White }.to_string(), "♖");
        assert_eq!(Piece { t: PieceType::Knight, color: PieceColor::White }.to_string(), "♘");
        assert_eq!(Piece { t: PieceType::Bishop, color: PieceColor::White }.to_string(), "♗");
        assert_eq!(Piece { t: PieceType::Queen, color: PieceColor::White }.to_string(), "♕");
        assert_eq!(Piece { t: PieceType::King, color: PieceColor::White }.to_string(), "♔");
        assert_eq!(Piece { t: PieceType::Pawn, color: PieceColor::White }.to_string(), "♙");
        assert_eq!(Piece { t: PieceType::Rook, color: PieceColor::Black }.to_string(), "♜");
        assert_eq!(Piece { t: PieceType::Knight, color: PieceColor::Black }.to_string(), "♞");
        assert_eq!(Piece { t: PieceType::Bishop, color: PieceColor::Black }.to_string(), "♝");
        assert_eq!(Piece { t: PieceType::Queen, color: PieceColor::Black }.to_string(), "♛");
        assert_eq!(Piece { t: PieceType::King, color: PieceColor::Black }.to_string(), "♚");
        assert_eq!(Piece { t: PieceType::Pawn, color: PieceColor::Black }.to_string(), "♟");
    }
}
