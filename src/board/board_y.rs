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

pub fn of_idx(i: u8) -> Option<BoardY> {
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

pub fn to_idx(y: &BoardY) -> u8 {
    match y {
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

pub fn of_str(s: &str) -> Option<BoardY> {
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

pub fn to_str(y: &BoardY) -> &'static str {
    match y {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_of_idx() {
        assert_eq!(of_idx(0), Some(BoardY::_1));
        assert_eq!(of_idx(1), Some(BoardY::_2));
        assert_eq!(of_idx(2), Some(BoardY::_3));
        assert_eq!(of_idx(3), Some(BoardY::_4));
        assert_eq!(of_idx(4), Some(BoardY::_5));
        assert_eq!(of_idx(5), Some(BoardY::_6));
        assert_eq!(of_idx(6), Some(BoardY::_7));
        assert_eq!(of_idx(7), Some(BoardY::_8));
        assert_eq!(of_idx(8), None);
    }

    #[test]
    fn test_to_idx() {
        assert_eq!(to_idx(&BoardY::_1), 0);
        assert_eq!(to_idx(&BoardY::_2), 1);
        assert_eq!(to_idx(&BoardY::_3), 2);
        assert_eq!(to_idx(&BoardY::_4), 3);
        assert_eq!(to_idx(&BoardY::_5), 4);
        assert_eq!(to_idx(&BoardY::_6), 5);
        assert_eq!(to_idx(&BoardY::_7), 6);
        assert_eq!(to_idx(&BoardY::_8), 7);
    }

    #[test]
    fn test_of_str() {
        assert_eq!(of_str("0"), None);
        assert_eq!(of_str("1"), Some(BoardY::_1));
        assert_eq!(of_str("2"), Some(BoardY::_2));
        assert_eq!(of_str("3"), Some(BoardY::_3));
        assert_eq!(of_str("4"), Some(BoardY::_4));
        assert_eq!(of_str("5"), Some(BoardY::_5));
        assert_eq!(of_str("6"), Some(BoardY::_6));
        assert_eq!(of_str("7"), Some(BoardY::_7));
        assert_eq!(of_str("8"), Some(BoardY::_8));
    }

    #[test]
    fn test_to_str() {
        assert_eq!(to_str(&BoardY::_1), "1");
        assert_eq!(to_str(&BoardY::_2), "2");
        assert_eq!(to_str(&BoardY::_3), "3");
        assert_eq!(to_str(&BoardY::_4), "4");
        assert_eq!(to_str(&BoardY::_5), "5");
        assert_eq!(to_str(&BoardY::_6), "6");
        assert_eq!(to_str(&BoardY::_7), "7");
        assert_eq!(to_str(&BoardY::_8), "8");
    }
}
