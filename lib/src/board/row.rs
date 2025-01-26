#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Row {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

impl Row {
    pub fn try_of_idx(i: u8) -> Option<Self> {
        match i {
            7 => Some(Row::_1),
            6 => Some(Row::_2),
            5 => Some(Row::_3),
            4 => Some(Row::_4),
            3 => Some(Row::_5),
            2 => Some(Row::_6),
            1 => Some(Row::_7),
            0 => Some(Row::_8),
            _ => None,
        }
    }

    fn of_idx(i: u8) -> Self {
        Self::try_of_idx(i).unwrap()
    }

    pub fn to_idx(&self) -> u8 {
        match self {
            Row::_1 => 7,
            Row::_2 => 6,
            Row::_3 => 5,
            Row::_4 => 4,
            Row::_5 => 3,
            Row::_6 => 2,
            Row::_7 => 1,
            Row::_8 => 0,
        }
    }

    pub fn try_of_str(s: &str) -> Option<Row> {
        match s {
            "1" => Some(Row::_1),
            "2" => Some(Row::_2),
            "3" => Some(Row::_3),
            "4" => Some(Row::_4),
            "5" => Some(Row::_5),
            "6" => Some(Row::_6),
            "7" => Some(Row::_7),
            "8" => Some(Row::_8),
            _ => None,
        }
    }

    fn of_str(s: &str) -> Self {
        Self::try_of_str(s).unwrap()
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Row::_1 => "1",
            Row::_2 => "2",
            Row::_3 => "3",
            Row::_4 => "4",
            Row::_5 => "5",
            Row::_6 => "6",
            Row::_7 => "7",
            Row::_8 => "8",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_of_idx_some() {
        assert_eq!(Row::try_of_idx(0), Some(Row::_8));
        assert_eq!(Row::try_of_idx(1), Some(Row::_7));
        assert_eq!(Row::try_of_idx(2), Some(Row::_6));
        assert_eq!(Row::try_of_idx(3), Some(Row::_5));
        assert_eq!(Row::try_of_idx(4), Some(Row::_4));
        assert_eq!(Row::try_of_idx(5), Some(Row::_3));
        assert_eq!(Row::try_of_idx(6), Some(Row::_2));
        assert_eq!(Row::try_of_idx(7), Some(Row::_1));
    }

    #[test]
    fn test_try_of_idx_none() {
        assert_eq!(Row::try_of_idx(8), None);
        assert_eq!(Row::try_of_idx(9), None);
        assert_eq!(Row::try_of_idx(10), None);
    }

    #[test]
    fn test_of_idx() {
        assert_eq!(Row::of_idx(0), Row::_8);
        assert_eq!(Row::of_idx(1), Row::_7);
        assert_eq!(Row::of_idx(2), Row::_6);
        assert_eq!(Row::of_idx(3), Row::_5);
        assert_eq!(Row::of_idx(4), Row::_4);
        assert_eq!(Row::of_idx(5), Row::_3);
        assert_eq!(Row::of_idx(6), Row::_2);
        assert_eq!(Row::of_idx(7), Row::_1);
    }

    #[test]
    fn test_to_idx() {
        assert_eq!(Row::_1.to_idx(), 7);
        assert_eq!(Row::_2.to_idx(), 6);
        assert_eq!(Row::_3.to_idx(), 5);
        assert_eq!(Row::_4.to_idx(), 4);
        assert_eq!(Row::_5.to_idx(), 3);
        assert_eq!(Row::_6.to_idx(), 2);
        assert_eq!(Row::_7.to_idx(), 1);
        assert_eq!(Row::_8.to_idx(), 0);
    }

    #[test]
    fn test_try_of_str_some() {
        assert_eq!(Row::try_of_str("1"), Some(Row::_1));
        assert_eq!(Row::try_of_str("2"), Some(Row::_2));
        assert_eq!(Row::try_of_str("3"), Some(Row::_3));
        assert_eq!(Row::try_of_str("4"), Some(Row::_4));
        assert_eq!(Row::try_of_str("5"), Some(Row::_5));
        assert_eq!(Row::try_of_str("6"), Some(Row::_6));
        assert_eq!(Row::try_of_str("7"), Some(Row::_7));
        assert_eq!(Row::try_of_str("8"), Some(Row::_8));
    }

    #[test]
    fn test_try_of_str_none() {
        assert_eq!(Row::try_of_str("0"), None);
        assert_eq!(Row::try_of_str("9"), None);
        assert_eq!(Row::try_of_str("10"), None);
    }

    #[test]
    fn test_of_str() {
        assert_eq!(Row::of_str("1"), Row::_1);
        assert_eq!(Row::of_str("2"), Row::_2);
        assert_eq!(Row::of_str("3"), Row::_3);
        assert_eq!(Row::of_str("4"), Row::_4);
        assert_eq!(Row::of_str("5"), Row::_5);
        assert_eq!(Row::of_str("6"), Row::_6);
        assert_eq!(Row::of_str("7"), Row::_7);
        assert_eq!(Row::of_str("8"), Row::_8);
    }

    #[test]
    fn test_to_str() {
        assert_eq!(Row::_1.to_str(), "1");
        assert_eq!(Row::_2.to_str(), "2");
        assert_eq!(Row::_3.to_str(), "3");
        assert_eq!(Row::_4.to_str(), "4");
        assert_eq!(Row::_5.to_str(), "5");
        assert_eq!(Row::_6.to_str(), "6");
        assert_eq!(Row::_7.to_str(), "7");
        assert_eq!(Row::_8.to_str(), "8");
    }
}
