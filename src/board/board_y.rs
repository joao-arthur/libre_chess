#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BoardY {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

impl BoardY {
    pub fn from_idx(i: u8) -> Option<BoardY> {
        match i {
            0 => Some(BoardY::_1),
            1 => Some(BoardY::_2),
            2 => Some(BoardY::_3),
            3 => Some(BoardY::_4),
            4 => Some(BoardY::_5),
            5 => Some(BoardY::_6),
            6 => Some(BoardY::_7),
            7 => Some(BoardY::_8),
            _ => None,
        }
    }

    pub fn to_idx(self: &BoardY) -> u8 {
        match self {
            BoardY::_1 => 0,
            BoardY::_2 => 1,
            BoardY::_3 => 2,
            BoardY::_4 => 3,
            BoardY::_5 => 4,
            BoardY::_6 => 5,
            BoardY::_7 => 6,
            BoardY::_8 => 7,
        }
    }

    pub fn from_str(s: &str) -> Option<BoardY> {
        match s {
            "1" => Some(BoardY::_1),
            "2" => Some(BoardY::_2),
            "3" => Some(BoardY::_3),
            "4" => Some(BoardY::_4),
            "5" => Some(BoardY::_5),
            "6" => Some(BoardY::_6),
            "7" => Some(BoardY::_7),
            "8" => Some(BoardY::_8),
            _ => None,
        }
    }

    pub fn to_str(self: &BoardY) -> &'static str {
        match self {
            BoardY::_1 => "1",
            BoardY::_2 => "2",
            BoardY::_3 => "3",
            BoardY::_4 => "4",
            BoardY::_5 => "5",
            BoardY::_6 => "6",
            BoardY::_7 => "7",
            BoardY::_8 => "8",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_idx() {
        assert_eq!(BoardY::from_idx(0), Some(BoardY::_1));
        assert_eq!(BoardY::from_idx(1), Some(BoardY::_2));
        assert_eq!(BoardY::from_idx(2), Some(BoardY::_3));
        assert_eq!(BoardY::from_idx(3), Some(BoardY::_4));
        assert_eq!(BoardY::from_idx(4), Some(BoardY::_5));
        assert_eq!(BoardY::from_idx(5), Some(BoardY::_6));
        assert_eq!(BoardY::from_idx(6), Some(BoardY::_7));
        assert_eq!(BoardY::from_idx(7), Some(BoardY::_8));
        assert_eq!(BoardY::from_idx(8), None);
    }

    #[test]
    fn test_to_idx() {
        assert_eq!(BoardY::_1.to_idx(), 0);
        assert_eq!(BoardY::_2.to_idx(), 1);
        assert_eq!(BoardY::_3.to_idx(), 2);
        assert_eq!(BoardY::_4.to_idx(), 3);
        assert_eq!(BoardY::_5.to_idx(), 4);
        assert_eq!(BoardY::_6.to_idx(), 5);
        assert_eq!(BoardY::_7.to_idx(), 6);
        assert_eq!(BoardY::_8.to_idx(), 7);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(BoardY::from_str("0"), None);
        assert_eq!(BoardY::from_str("1"), Some(BoardY::_1));
        assert_eq!(BoardY::from_str("2"), Some(BoardY::_2));
        assert_eq!(BoardY::from_str("3"), Some(BoardY::_3));
        assert_eq!(BoardY::from_str("4"), Some(BoardY::_4));
        assert_eq!(BoardY::from_str("5"), Some(BoardY::_5));
        assert_eq!(BoardY::from_str("6"), Some(BoardY::_6));
        assert_eq!(BoardY::from_str("7"), Some(BoardY::_7));
        assert_eq!(BoardY::from_str("8"), Some(BoardY::_8));
    }

    #[test]
    fn test_to_str() {
        assert_eq!(BoardY::_1.to_str(), "1");
        assert_eq!(BoardY::_2.to_str(), "2");
        assert_eq!(BoardY::_3.to_str(), "3");
        assert_eq!(BoardY::_4.to_str(), "4");
        assert_eq!(BoardY::_5.to_str(), "5");
        assert_eq!(BoardY::_6.to_str(), "6");
        assert_eq!(BoardY::_7.to_str(), "7");
        assert_eq!(BoardY::_8.to_str(), "8");
    }
}
