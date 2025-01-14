#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BoardX {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

pub fn from_idx(num: u8) -> Option<BoardX> {
    match num {
        0 => Some(BoardX::A),
        1 => Some(BoardX::B),
        2 => Some(BoardX::C),
        3 => Some(BoardX::D),
        4 => Some(BoardX::E),
        5 => Some(BoardX::F),
        6 => Some(BoardX::G),
        7 => Some(BoardX::H),
        _ => None,
    }
}

pub fn to_idx(x: &BoardX) -> u8 {
    match x {
        BoardX::A => 0,
        BoardX::B => 1,
        BoardX::C => 2,
        BoardX::D => 3,
        BoardX::E => 4,
        BoardX::F => 5,
        BoardX::G => 6,
        BoardX::H => 7,
    }
}

pub fn from_str(s: &str) -> Option<BoardX> {
    match s {
        "A" => Some(BoardX::A),
        "B" => Some(BoardX::B),
        "C" => Some(BoardX::C),
        "D" => Some(BoardX::D),
        "E" => Some(BoardX::E),
        "F" => Some(BoardX::F),
        "G" => Some(BoardX::G),
        "H" => Some(BoardX::H),
        _ => None,
    }
}

pub fn to_str(x: &BoardX) -> &'static str {
    match x {
        BoardX::A => "A",
        BoardX::B => "B",
        BoardX::C => "C",
        BoardX::D => "D",
        BoardX::E => "E",
        BoardX::F => "F",
        BoardX::G => "G",
        BoardX::H => "H",
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_idx() {
        assert_eq!(from_idx(0), Some(BoardX::A));
        assert_eq!(from_idx(1), Some(BoardX::B));
        assert_eq!(from_idx(2), Some(BoardX::C));
        assert_eq!(from_idx(3), Some(BoardX::D));
        assert_eq!(from_idx(4), Some(BoardX::E));
        assert_eq!(from_idx(5), Some(BoardX::F));
        assert_eq!(from_idx(6), Some(BoardX::G));
        assert_eq!(from_idx(7), Some(BoardX::H));
        assert_eq!(from_idx(8), None);
    }

    #[test]
    fn test_to_idx() {
        assert_eq!(to_idx(&BoardX::A), 0);
        assert_eq!(to_idx(&BoardX::B), 1);
        assert_eq!(to_idx(&BoardX::C), 2);
        assert_eq!(to_idx(&BoardX::D), 3);
        assert_eq!(to_idx(&BoardX::E), 4);
        assert_eq!(to_idx(&BoardX::F), 5);
        assert_eq!(to_idx(&BoardX::G), 6);
        assert_eq!(to_idx(&BoardX::H), 7);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(from_str("A"), Some(BoardX::A));
        assert_eq!(from_str("B"), Some(BoardX::B));
        assert_eq!(from_str("C"), Some(BoardX::C));
        assert_eq!(from_str("D"), Some(BoardX::D));
        assert_eq!(from_str("E"), Some(BoardX::E));
        assert_eq!(from_str("F"), Some(BoardX::F));
        assert_eq!(from_str("G"), Some(BoardX::G));
        assert_eq!(from_str("H"), Some(BoardX::H));
        assert_eq!(from_str("I"), None);
    }

    #[test]
    fn test_to_str() {
        assert_eq!(to_str(&BoardX::A), "A");
        assert_eq!(to_str(&BoardX::B), "B");
        assert_eq!(to_str(&BoardX::C), "C");
        assert_eq!(to_str(&BoardX::D), "D");
        assert_eq!(to_str(&BoardX::E), "E");
        assert_eq!(to_str(&BoardX::F), "F");
        assert_eq!(to_str(&BoardX::G), "G");
        assert_eq!(to_str(&BoardX::H), "H");
    }
}
