#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Col {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Col {
    pub fn try_of_idx(i: u8) -> Option<Self> {
        match i {
            0 => Some(Col::A),
            1 => Some(Col::B),
            2 => Some(Col::C),
            3 => Some(Col::D),
            4 => Some(Col::E),
            5 => Some(Col::F),
            6 => Some(Col::G),
            7 => Some(Col::H),
            _ => None,
        }
    }

    fn of_idx(i: u8) -> Self {
        Self::try_of_idx(i).unwrap()
    }

    pub fn to_idx(&self) -> u8 {
        match self {
            Col::A => 0,
            Col::B => 1,
            Col::C => 2,
            Col::D => 3,
            Col::E => 4,
            Col::F => 5,
            Col::G => 6,
            Col::H => 7,
        }
    }

    pub fn try_of_str(s: &str) -> Option<Self> {
        match s {
            "A" => Some(Col::A),
            "B" => Some(Col::B),
            "C" => Some(Col::C),
            "D" => Some(Col::D),
            "E" => Some(Col::E),
            "F" => Some(Col::F),
            "G" => Some(Col::G),
            "H" => Some(Col::H),
            _ => None,
        }
    }

    fn of_str(s: &str) -> Self {
        Self::try_of_str(s).unwrap()
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Col::A => "A",
            Col::B => "B",
            Col::C => "C",
            Col::D => "D",
            Col::E => "E",
            Col::F => "F",
            Col::G => "G",
            Col::H => "H",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_of_idx_some() {
        assert_eq!(Col::try_of_idx(0), Some(Col::A));
        assert_eq!(Col::try_of_idx(1), Some(Col::B));
        assert_eq!(Col::try_of_idx(2), Some(Col::C));
        assert_eq!(Col::try_of_idx(3), Some(Col::D));
        assert_eq!(Col::try_of_idx(4), Some(Col::E));
        assert_eq!(Col::try_of_idx(5), Some(Col::F));
        assert_eq!(Col::try_of_idx(6), Some(Col::G));
        assert_eq!(Col::try_of_idx(7), Some(Col::H));
    }

    #[test]
    fn test_try_of_idx_none() {
        assert_eq!(Col::try_of_idx(8), None);
        assert_eq!(Col::try_of_idx(9), None);
        assert_eq!(Col::try_of_idx(10), None);
    }

    #[test]
    fn test_of_idx() {
        assert_eq!(Col::of_idx(0), Col::A);
        assert_eq!(Col::of_idx(1), Col::B);
        assert_eq!(Col::of_idx(2), Col::C);
        assert_eq!(Col::of_idx(3), Col::D);
        assert_eq!(Col::of_idx(4), Col::E);
        assert_eq!(Col::of_idx(5), Col::F);
        assert_eq!(Col::of_idx(6), Col::G);
        assert_eq!(Col::of_idx(7), Col::H);
    }

    #[test]
    fn test_to_idx() {
        assert_eq!(Col::A.to_idx(), 0);
        assert_eq!(Col::B.to_idx(), 1);
        assert_eq!(Col::C.to_idx(), 2);
        assert_eq!(Col::D.to_idx(), 3);
        assert_eq!(Col::E.to_idx(), 4);
        assert_eq!(Col::F.to_idx(), 5);
        assert_eq!(Col::G.to_idx(), 6);
        assert_eq!(Col::H.to_idx(), 7);
    }

    #[test]
    fn test_try_of_str_some() {
        assert_eq!(Col::try_of_str("A"), Some(Col::A));
        assert_eq!(Col::try_of_str("B"), Some(Col::B));
        assert_eq!(Col::try_of_str("C"), Some(Col::C));
        assert_eq!(Col::try_of_str("D"), Some(Col::D));
        assert_eq!(Col::try_of_str("E"), Some(Col::E));
        assert_eq!(Col::try_of_str("F"), Some(Col::F));
        assert_eq!(Col::try_of_str("G"), Some(Col::G));
        assert_eq!(Col::try_of_str("H"), Some(Col::H));
    }

    #[test]
    fn test_try_of_str_none() {
        assert_eq!(Col::try_of_str("I"), None);
        assert_eq!(Col::try_of_str("J"), None);
        assert_eq!(Col::try_of_str("K"), None);
    }

    #[test]
    fn test_of_str() {
        assert_eq!(Col::of_str("A"), Col::A);
        assert_eq!(Col::of_str("B"), Col::B);
        assert_eq!(Col::of_str("C"), Col::C);
        assert_eq!(Col::of_str("D"), Col::D);
        assert_eq!(Col::of_str("E"), Col::E);
        assert_eq!(Col::of_str("F"), Col::F);
        assert_eq!(Col::of_str("G"), Col::G);
        assert_eq!(Col::of_str("H"), Col::H);
    }

    #[test]
    fn test_to_str() {
        assert_eq!(Col::A.to_str(), "A");
        assert_eq!(Col::B.to_str(), "B");
        assert_eq!(Col::C.to_str(), "C");
        assert_eq!(Col::D.to_str(), "D");
        assert_eq!(Col::E.to_str(), "E");
        assert_eq!(Col::F.to_str(), "F");
        assert_eq!(Col::G.to_str(), "G");
        assert_eq!(Col::H.to_str(), "H");
    }
}
