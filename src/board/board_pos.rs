use super::{
    board_x::{self, BoardX},
    board_y::{self, BoardY},
};

#[derive(Debug, PartialEq)]
pub struct BoardPos {
    pub x: BoardX,
    pub y: BoardY,
}

pub fn from_str(s: &str) -> Option<BoardPos> {
    let mut chars = s.chars();
    let x = chars.next().and_then(|pos| board_x::from_str(&pos.to_string()))?;
    let y = chars.next().and_then(|pos| board_y::from_str(&pos.to_string()))?;
    Some(BoardPos { x, y })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str_some() {
        assert_eq!(from_str("A1"), Some(BoardPos { x: BoardX::A, y: BoardY::_1 }));
        assert_eq!(from_str("A2"), Some(BoardPos { x: BoardX::A, y: BoardY::_2 }));
        assert_eq!(from_str("A3"), Some(BoardPos { x: BoardX::A, y: BoardY::_3 }));
        assert_eq!(from_str("A4"), Some(BoardPos { x: BoardX::A, y: BoardY::_4 }));
        assert_eq!(from_str("A5"), Some(BoardPos { x: BoardX::A, y: BoardY::_5 }));
        assert_eq!(from_str("A6"), Some(BoardPos { x: BoardX::A, y: BoardY::_6 }));
        assert_eq!(from_str("A7"), Some(BoardPos { x: BoardX::A, y: BoardY::_7 }));
        assert_eq!(from_str("A8"), Some(BoardPos { x: BoardX::A, y: BoardY::_8 }));
        assert_eq!(from_str("B8"), Some(BoardPos { x: BoardX::B, y: BoardY::_8 }));
        assert_eq!(from_str("C8"), Some(BoardPos { x: BoardX::C, y: BoardY::_8 }));
        assert_eq!(from_str("D8"), Some(BoardPos { x: BoardX::D, y: BoardY::_8 }));
        assert_eq!(from_str("E8"), Some(BoardPos { x: BoardX::E, y: BoardY::_8 }));
        assert_eq!(from_str("F8"), Some(BoardPos { x: BoardX::F, y: BoardY::_8 }));
        assert_eq!(from_str("G8"), Some(BoardPos { x: BoardX::G, y: BoardY::_8 }));
        assert_eq!(from_str("H8"), Some(BoardPos { x: BoardX::H, y: BoardY::_8 }));
    }

    #[test]
    fn test_from_str_none() {
        assert_eq!(from_str("A0"), None);
        assert_eq!(from_str("A9"), None);
        assert_eq!(from_str("I1"), None);
        assert_eq!(from_str("I2"), None);
    }
}
