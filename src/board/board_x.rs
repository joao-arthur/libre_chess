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

impl BoardX {
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

    pub fn to_idx(self: &BoardX) -> u8 {
        match self {
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

    pub fn from_str(str: &str) -> Option<BoardX> {
        match str {
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

    pub fn to_str(self: &BoardX) -> &'static str {
        match self {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_idx() {
        assert_eq!(BoardX::from_idx(0), Some(BoardX::A));
        assert_eq!(BoardX::from_idx(1), Some(BoardX::B));
        assert_eq!(BoardX::from_idx(2), Some(BoardX::C));
        assert_eq!(BoardX::from_idx(3), Some(BoardX::D));
        assert_eq!(BoardX::from_idx(4), Some(BoardX::E));
        assert_eq!(BoardX::from_idx(5), Some(BoardX::F));
        assert_eq!(BoardX::from_idx(6), Some(BoardX::G));
        assert_eq!(BoardX::from_idx(7), Some(BoardX::H));
        assert_eq!(BoardX::from_idx(8), None);
    }

    #[test]
    fn test_to_idx() {
        assert_eq!(BoardX::A.to_idx(), 0);
        assert_eq!(BoardX::B.to_idx(), 1);
        assert_eq!(BoardX::C.to_idx(), 2);
        assert_eq!(BoardX::D.to_idx(), 3);
        assert_eq!(BoardX::E.to_idx(), 4);
        assert_eq!(BoardX::F.to_idx(), 5);
        assert_eq!(BoardX::G.to_idx(), 6);
        assert_eq!(BoardX::H.to_idx(), 7);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(BoardX::from_str("A"), Some(BoardX::A));
        assert_eq!(BoardX::from_str("B"), Some(BoardX::B));
        assert_eq!(BoardX::from_str("C"), Some(BoardX::C));
        assert_eq!(BoardX::from_str("D"), Some(BoardX::D));
        assert_eq!(BoardX::from_str("E"), Some(BoardX::E));
        assert_eq!(BoardX::from_str("F"), Some(BoardX::F));
        assert_eq!(BoardX::from_str("G"), Some(BoardX::G));
        assert_eq!(BoardX::from_str("H"), Some(BoardX::H));
        assert_eq!(BoardX::from_str("I"), None);
    }

    #[test]
    fn test_to_str() {
        assert_eq!(BoardX::A.to_str(), "A");
        assert_eq!(BoardX::B.to_str(), "B");
        assert_eq!(BoardX::C.to_str(), "C");
        assert_eq!(BoardX::D.to_str(), "D");
        assert_eq!(BoardX::E.to_str(), "E");
        assert_eq!(BoardX::F.to_str(), "F");
        assert_eq!(BoardX::G.to_str(), "G");
        assert_eq!(BoardX::H.to_str(), "H");
    }
}
