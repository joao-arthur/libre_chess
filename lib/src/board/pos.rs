use super::{col::Col, row::Row};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Pos {
    pub row: Row,
    pub col: Col,
}

impl Pos {
    pub fn try_of_idx(row: u8, col: u8) -> Option<Self> {
        let row = Row::try_of_idx(row)?;
        let col = Col::try_of_idx(col)?;
        Some(Pos { row, col })
    }

    pub fn of_idx(row: u8, col: u8) -> Self {
        Self::try_of_idx(row, col).unwrap()
    }

    pub fn try_of_rel_idx(&self, row: i8, col: i8) -> Option<Self> {
        let self_row = self.row.to_idx() as i8;
        let self_col = self.col.to_idx() as i8;
        if col < 0 && col.abs() > self_col {
            return None;
        }
        if row < 0 && row.abs() > self_row {
            return None;
        }
        Pos::try_of_idx((self_row + row) as u8, (self_col + col) as u8)
    }

    pub fn of_rel_idx(&self, row: i8, col: i8) -> Self {
        self.try_of_rel_idx(row, col).unwrap()
    }

    pub fn try_of_str(s: &str) -> Option<Self> {
        let mut chars = s.chars();
        let col = chars.next().and_then(|pos| Col::try_of_str(&pos.to_string()))?;
        let row = chars.next().and_then(|pos| Row::try_of_str(&pos.to_string()))?;
        if chars.next() == None {
            Some(Pos { col, row })
        } else {
            None
        }
    }

    pub fn of_str(s: &str) -> Self {
        Self::try_of_str(s).unwrap()
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.col.to_str(), self.row.to_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_of_idx_some() {
        assert_eq!(Pos::try_of_idx(7, 0), Some(Pos { row: Row::_1, col: Col::A }));
        assert_eq!(Pos::try_of_idx(6, 0), Some(Pos { row: Row::_2, col: Col::A }));
        assert_eq!(Pos::try_of_idx(5, 0), Some(Pos { row: Row::_3, col: Col::A }));
        assert_eq!(Pos::try_of_idx(4, 0), Some(Pos { row: Row::_4, col: Col::A }));
        assert_eq!(Pos::try_of_idx(3, 0), Some(Pos { row: Row::_5, col: Col::A }));
        assert_eq!(Pos::try_of_idx(2, 0), Some(Pos { row: Row::_6, col: Col::A }));
        assert_eq!(Pos::try_of_idx(1, 0), Some(Pos { row: Row::_7, col: Col::A }));
        assert_eq!(Pos::try_of_idx(0, 0), Some(Pos { row: Row::_8, col: Col::A }));
        assert_eq!(Pos::try_of_idx(0, 1), Some(Pos { row: Row::_8, col: Col::B }));
        assert_eq!(Pos::try_of_idx(0, 2), Some(Pos { row: Row::_8, col: Col::C }));
        assert_eq!(Pos::try_of_idx(0, 3), Some(Pos { row: Row::_8, col: Col::D }));
        assert_eq!(Pos::try_of_idx(0, 4), Some(Pos { row: Row::_8, col: Col::E }));
        assert_eq!(Pos::try_of_idx(0, 5), Some(Pos { row: Row::_8, col: Col::F }));
        assert_eq!(Pos::try_of_idx(0, 6), Some(Pos { row: Row::_8, col: Col::G }));
        assert_eq!(Pos::try_of_idx(0, 7), Some(Pos { row: Row::_8, col: Col::H }));
    }

    #[test]
    fn test_try_of_idx_none() {
        assert_eq!(Pos::try_of_idx(8, 8), None);
        assert_eq!(Pos::try_of_idx(9, 9), None);
        assert_eq!(Pos::try_of_idx(10, 10), None);
    }

    #[test]
    fn test_of_idx() {
        assert_eq!(Pos::of_idx(7, 0), Pos { row: Row::_1, col: Col::A });
        assert_eq!(Pos::of_idx(6, 0), Pos { row: Row::_2, col: Col::A });
        assert_eq!(Pos::of_idx(5, 0), Pos { row: Row::_3, col: Col::A });
        assert_eq!(Pos::of_idx(4, 0), Pos { row: Row::_4, col: Col::A });
        assert_eq!(Pos::of_idx(3, 0), Pos { row: Row::_5, col: Col::A });
        assert_eq!(Pos::of_idx(2, 0), Pos { row: Row::_6, col: Col::A });
        assert_eq!(Pos::of_idx(1, 0), Pos { row: Row::_7, col: Col::A });
        assert_eq!(Pos::of_idx(0, 0), Pos { row: Row::_8, col: Col::A });
        assert_eq!(Pos::of_idx(0, 1), Pos { row: Row::_8, col: Col::B });
        assert_eq!(Pos::of_idx(0, 2), Pos { row: Row::_8, col: Col::C });
        assert_eq!(Pos::of_idx(0, 3), Pos { row: Row::_8, col: Col::D });
        assert_eq!(Pos::of_idx(0, 4), Pos { row: Row::_8, col: Col::E });
        assert_eq!(Pos::of_idx(0, 5), Pos { row: Row::_8, col: Col::F });
        assert_eq!(Pos::of_idx(0, 6), Pos { row: Row::_8, col: Col::G });
        assert_eq!(Pos::of_idx(0, 7), Pos { row: Row::_8, col: Col::H });
    }

    #[test]
    fn test_try_of_rel_idx_some() {
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(0, 0), Some(Pos::of_idx(7, 7)));
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(-1, -1,), Some(Pos::of_idx(6, 6)));
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(-2, -2,), Some(Pos::of_idx(5, 5)));
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(-3, -3,), Some(Pos::of_idx(4, 4)));
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(-4, -4,), Some(Pos::of_idx(3, 3)));
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(-5, -5,), Some(Pos::of_idx(2, 2)));
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(-6, -6,), Some(Pos::of_idx(1, 1)));
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(-7, -7,), Some(Pos::of_idx(0, 0)));
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(0, 0,), Some(Pos::of_idx(0, 0)));
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(1, 1,), Some(Pos::of_idx(1, 1)));
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(2, 2,), Some(Pos::of_idx(2, 2)));
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(3, 3,), Some(Pos::of_idx(3, 3)));
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(4, 4,), Some(Pos::of_idx(4, 4)));
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(5, 5,), Some(Pos::of_idx(5, 5)));
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(6, 6,), Some(Pos::of_idx(6, 6)));
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(7, 7,), Some(Pos::of_idx(7, 7)));
    }

    #[test]
    fn test_try_of_rel_idx_none() {
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(-1, -1), None);
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(-2, -2), None);
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(-3, -3), None);
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(-4, -4), None);
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(-5, -5), None);
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(-6, -6), None);
        assert_eq!(Pos::of_idx(0, 0).try_of_rel_idx(-7, -7), None);
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(1, 1), None);
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(2, 2), None);
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(3, 3), None);
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(4, 4), None);
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(5, 5), None);
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(6, 6), None);
        assert_eq!(Pos::of_idx(7, 7).try_of_rel_idx(7, 7), None);
    }

    #[test]
    fn test_of_rel_idx() {
        assert_eq!(Pos::of_idx(7, 7).of_rel_idx(0, 0), Pos::of_idx(7, 7));
        assert_eq!(Pos::of_idx(7, 7).of_rel_idx(-1, -1), Pos::of_idx(6, 6));
        assert_eq!(Pos::of_idx(7, 7).of_rel_idx(-2, -2), Pos::of_idx(5, 5));
        assert_eq!(Pos::of_idx(7, 7).of_rel_idx(-3, -3), Pos::of_idx(4, 4));
        assert_eq!(Pos::of_idx(7, 7).of_rel_idx(-4, -4), Pos::of_idx(3, 3));
        assert_eq!(Pos::of_idx(7, 7).of_rel_idx(-5, -5), Pos::of_idx(2, 2));
        assert_eq!(Pos::of_idx(7, 7).of_rel_idx(-6, -6), Pos::of_idx(1, 1));
        assert_eq!(Pos::of_idx(7, 7).of_rel_idx(-7, -7), Pos::of_idx(0, 0));
        assert_eq!(Pos::of_idx(0, 0).of_rel_idx(0, 0), Pos::of_idx(0, 0));
        assert_eq!(Pos::of_idx(0, 0).of_rel_idx(1, 1), Pos::of_idx(1, 1));
        assert_eq!(Pos::of_idx(0, 0).of_rel_idx(2, 2), Pos::of_idx(2, 2));
        assert_eq!(Pos::of_idx(0, 0).of_rel_idx(3, 3), Pos::of_idx(3, 3));
        assert_eq!(Pos::of_idx(0, 0).of_rel_idx(4, 4), Pos::of_idx(4, 4));
        assert_eq!(Pos::of_idx(0, 0).of_rel_idx(5, 5), Pos::of_idx(5, 5));
        assert_eq!(Pos::of_idx(0, 0).of_rel_idx(6, 6), Pos::of_idx(6, 6));
        assert_eq!(Pos::of_idx(0, 0).of_rel_idx(7, 7), Pos::of_idx(7, 7));
    }

    #[test]
    fn test_try_of_str_some() {
        assert_eq!(Pos::try_of_str("A1"), Some(Pos { row: Row::_1, col: Col::A }));
        assert_eq!(Pos::try_of_str("A2"), Some(Pos { row: Row::_2, col: Col::A }));
        assert_eq!(Pos::try_of_str("A3"), Some(Pos { row: Row::_3, col: Col::A }));
        assert_eq!(Pos::try_of_str("A4"), Some(Pos { row: Row::_4, col: Col::A }));
        assert_eq!(Pos::try_of_str("A5"), Some(Pos { row: Row::_5, col: Col::A }));
        assert_eq!(Pos::try_of_str("A6"), Some(Pos { row: Row::_6, col: Col::A }));
        assert_eq!(Pos::try_of_str("A7"), Some(Pos { row: Row::_7, col: Col::A }));
        assert_eq!(Pos::try_of_str("A8"), Some(Pos { row: Row::_8, col: Col::A }));
        assert_eq!(Pos::try_of_str("B8"), Some(Pos { row: Row::_8, col: Col::B }));
        assert_eq!(Pos::try_of_str("C8"), Some(Pos { row: Row::_8, col: Col::C }));
        assert_eq!(Pos::try_of_str("D8"), Some(Pos { row: Row::_8, col: Col::D }));
        assert_eq!(Pos::try_of_str("E8"), Some(Pos { row: Row::_8, col: Col::E }));
        assert_eq!(Pos::try_of_str("F8"), Some(Pos { row: Row::_8, col: Col::F }));
        assert_eq!(Pos::try_of_str("G8"), Some(Pos { row: Row::_8, col: Col::G }));
        assert_eq!(Pos::try_of_str("H8"), Some(Pos { row: Row::_8, col: Col::H }));
    }

    #[test]
    fn test_try_of_str_none() {
        assert_eq!(Pos::try_of_str("A10"), None);
        assert_eq!(Pos::try_of_str("H10"), None);
        assert_eq!(Pos::try_of_str("A0"), None);
        assert_eq!(Pos::try_of_str("A9"), None);
        assert_eq!(Pos::try_of_str("I1"), None);
        assert_eq!(Pos::try_of_str("J2"), None);
        assert_eq!(Pos::try_of_str("K3"), None);
        assert_eq!(Pos::try_of_str("33"), None);
        assert_eq!(Pos::try_of_str("AB"), None);
        assert_eq!(Pos::try_of_str("4A"), None);
    }

    #[test]
    fn test_of_str() {
        assert_eq!(Pos::of_str("A1"), Pos { row: Row::_1, col: Col::A });
        assert_eq!(Pos::of_str("A2"), Pos { row: Row::_2, col: Col::A });
        assert_eq!(Pos::of_str("A3"), Pos { row: Row::_3, col: Col::A });
        assert_eq!(Pos::of_str("A4"), Pos { row: Row::_4, col: Col::A });
        assert_eq!(Pos::of_str("A5"), Pos { row: Row::_5, col: Col::A });
        assert_eq!(Pos::of_str("A6"), Pos { row: Row::_6, col: Col::A });
        assert_eq!(Pos::of_str("A7"), Pos { row: Row::_7, col: Col::A });
        assert_eq!(Pos::of_str("A8"), Pos { row: Row::_8, col: Col::A });
        assert_eq!(Pos::of_str("B8"), Pos { row: Row::_8, col: Col::B });
        assert_eq!(Pos::of_str("C8"), Pos { row: Row::_8, col: Col::C });
        assert_eq!(Pos::of_str("D8"), Pos { row: Row::_8, col: Col::D });
        assert_eq!(Pos::of_str("E8"), Pos { row: Row::_8, col: Col::E });
        assert_eq!(Pos::of_str("F8"), Pos { row: Row::_8, col: Col::F });
        assert_eq!(Pos::of_str("G8"), Pos { row: Row::_8, col: Col::G });
        assert_eq!(Pos::of_str("H8"), Pos { row: Row::_8, col: Col::H });
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Pos { row: Row::_1, col: Col::A }.to_string(), String::from("A1"));
        assert_eq!(Pos { row: Row::_2, col: Col::A }.to_string(), String::from("A2"));
        assert_eq!(Pos { row: Row::_3, col: Col::A }.to_string(), String::from("A3"));
        assert_eq!(Pos { row: Row::_4, col: Col::A }.to_string(), String::from("A4"));
        assert_eq!(Pos { row: Row::_5, col: Col::A }.to_string(), String::from("A5"));
        assert_eq!(Pos { row: Row::_6, col: Col::A }.to_string(), String::from("A6"));
        assert_eq!(Pos { row: Row::_7, col: Col::A }.to_string(), String::from("A7"));
        assert_eq!(Pos { row: Row::_8, col: Col::A }.to_string(), String::from("A8"));
        assert_eq!(Pos { row: Row::_8, col: Col::B }.to_string(), String::from("B8"));
        assert_eq!(Pos { row: Row::_8, col: Col::C }.to_string(), String::from("C8"));
        assert_eq!(Pos { row: Row::_8, col: Col::D }.to_string(), String::from("D8"));
        assert_eq!(Pos { row: Row::_8, col: Col::E }.to_string(), String::from("E8"));
        assert_eq!(Pos { row: Row::_8, col: Col::F }.to_string(), String::from("F8"));
        assert_eq!(Pos { row: Row::_8, col: Col::G }.to_string(), String::from("G8"));
        assert_eq!(Pos { row: Row::_8, col: Col::H }.to_string(), String::from("H8"));
    }
}
