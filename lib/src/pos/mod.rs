use std::fmt;

use self::{
    col::{col_to_string, col_try_of},
    row::{row_to_string, row_try_of},
};

mod col;
mod row;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Pos {
    pub row: u8,
    pub col: u8,
}

impl Pos {
    pub fn try_rel_idx(&self, row: i8, col: i8) -> Option<Self> {
        if row < 0 && row.unsigned_abs() > self.row {
            return None;
        }
        if col < 0 && col.unsigned_abs() > self.col {
            return None;
        }
        if row > 0 && (row as u8) > u8::MAX - self.row {
            return None;
        }
        if col > 0 && (col as u8) > u8::MAX - self.col {
            return None;
        }
        Some(Pos {
            row: ((self.row as i16) + row as i16) as u8,
            col: ((self.col as i16) + col as i16) as u8,
        })
    }

    pub fn rel_idx(&self, row: i8, col: i8) -> Self {
        self.try_rel_idx(row, col).unwrap()
    }

    pub fn try_of(s: &str) -> Option<Self> {
        let chars = s.chars();
        let mut pos_col = String::new();
        let mut pos_row = String::new();

        for c in chars {
            if c.is_ascii_uppercase() {
                pos_col.push(c);
            }
            if c.is_ascii_digit() {
                pos_row.push(c);
            }
        }
        Some(Pos { col: col_try_of(&pos_col)?, row: row_try_of(&pos_row)? })
    }

    pub fn of(s: &str) -> Self {
        Self::try_of(s).unwrap()
    }
}

//pub fn pos_of_str_slice<const N: usize>(values: [&str; N]) -> Vec<Pos> {
//    values.to_vec().iter().map(|value| Pos::of(value)).collect()
//}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", col_to_string(self.col) + &row_to_string(self.row))
    }
}

#[cfg(test)]
mod tests {
    use super::Pos;

    #[test]
    fn try_of_rel_idx_positive() {
        let pos = Pos { row: 0, col: 0 };
        assert_eq!(pos.try_rel_idx(0, 0), Some(Pos { row: 0, col: 0 }));
        assert_eq!(pos.try_rel_idx(1, 1), Some(Pos { row: 1, col: 1 }));
        assert_eq!(pos.try_rel_idx(2, 2), Some(Pos { row: 2, col: 2 }));
        assert_eq!(pos.try_rel_idx(3, 3), Some(Pos { row: 3, col: 3 }));
        assert_eq!(pos.try_rel_idx(4, 4), Some(Pos { row: 4, col: 4 }));
        assert_eq!(pos.try_rel_idx(5, 5), Some(Pos { row: 5, col: 5 }));
        assert_eq!(pos.try_rel_idx(6, 6), Some(Pos { row: 6, col: 6 }));
        assert_eq!(pos.try_rel_idx(7, 7), Some(Pos { row: 7, col: 7 }));
    }

    #[test]
    fn try_rel_idx_negative() {
        let pos = Pos { row: 255, col: 255 };
        assert_eq!(pos.try_rel_idx(0, 0), Some(Pos { row: 255, col: 255 }));
        assert_eq!(pos.try_rel_idx(-1, -1), Some(Pos { row: 254, col: 254 }));
        assert_eq!(pos.try_rel_idx(-2, -2), Some(Pos { row: 253, col: 253 }));
        assert_eq!(pos.try_rel_idx(-3, -3), Some(Pos { row: 252, col: 252 }));
        assert_eq!(pos.try_rel_idx(-4, -4), Some(Pos { row: 251, col: 251 }));
        assert_eq!(pos.try_rel_idx(-5, -5), Some(Pos { row: 250, col: 250 }));
        assert_eq!(pos.try_rel_idx(-6, -6), Some(Pos { row: 249, col: 249 }));
        assert_eq!(pos.try_rel_idx(-7, -7), Some(Pos { row: 248, col: 248 }));
    }

    #[test]
    fn try_rel_idx_positive_limit_row() {
        let pos = Pos { row: 253, col: 253 };
        assert_eq!(pos.try_rel_idx(0, 0), Some(Pos { row: 253, col: 253 }));
        assert_eq!(pos.try_rel_idx(1, 0), Some(Pos { row: 254, col: 253 }));
        assert_eq!(pos.try_rel_idx(2, 0), Some(Pos { row: 255, col: 253 }));
        assert_eq!(pos.try_rel_idx(3, 0), None);
        assert_eq!(pos.try_rel_idx(4, 0), None);
        assert_eq!(pos.try_rel_idx(5, 0), None);
    }

    #[test]
    fn try_rel_idx_positive_limit_col() {
        let pos = Pos { row: 253, col: 253 };
        assert_eq!(pos.try_rel_idx(0, 0), Some(Pos { row: 253, col: 253 }));
        assert_eq!(pos.try_rel_idx(0, 1), Some(Pos { row: 253, col: 254 }));
        assert_eq!(pos.try_rel_idx(0, 2), Some(Pos { row: 253, col: 255 }));
        assert_eq!(pos.try_rel_idx(0, 3), None);
        assert_eq!(pos.try_rel_idx(0, 4), None);
        assert_eq!(pos.try_rel_idx(0, 5), None);
    }

    #[test]
    fn try_rel_idx_negative_limit_row() {
        let pos = Pos { row: 2, col: 2 };
        assert_eq!(pos.try_rel_idx(0, 0), Some(Pos { row: 2, col: 2 }));
        assert_eq!(pos.try_rel_idx(-1, 0), Some(Pos { row: 1, col: 2 }));
        assert_eq!(pos.try_rel_idx(-2, 0), Some(Pos { row: 0, col: 2 }));
        assert_eq!(pos.try_rel_idx(-3, 0), None);
        assert_eq!(pos.try_rel_idx(-4, 0), None);
        assert_eq!(pos.try_rel_idx(-5, 0), None);
    }

    #[test]
    fn try_rel_idx_negative_limit_col() {
        let pos = Pos { row: 2, col: 2 };
        assert_eq!(pos.try_rel_idx(0, 0), Some(Pos { row: 2, col: 2 }));
        assert_eq!(pos.try_rel_idx(0, -1), Some(Pos { row: 2, col: 1 }));
        assert_eq!(pos.try_rel_idx(0, -2), Some(Pos { row: 2, col: 0 }));
        assert_eq!(pos.try_rel_idx(0, -3), None);
        assert_eq!(pos.try_rel_idx(0, -4), None);
        assert_eq!(pos.try_rel_idx(0, -5), None);
    }

    #[test]
    fn rel_idx_0() {
        let pos = Pos { row: 0, col: 0 };
        assert_eq!(pos.rel_idx(0, 0), Pos { row: 0, col: 0 });
        assert_eq!(pos.rel_idx(1, 1), Pos { row: 1, col: 1 });
        assert_eq!(pos.rel_idx(2, 2), Pos { row: 2, col: 2 });
        assert_eq!(pos.rel_idx(3, 3), Pos { row: 3, col: 3 });
        assert_eq!(pos.rel_idx(4, 4), Pos { row: 4, col: 4 });
        assert_eq!(pos.rel_idx(5, 5), Pos { row: 5, col: 5 });
        assert_eq!(pos.rel_idx(6, 6), Pos { row: 6, col: 6 });
        assert_eq!(pos.rel_idx(7, 7), Pos { row: 7, col: 7 });
    }

    #[test]
    fn rel_idx_255() {
        let pos = Pos { row: 255, col: 255 };
        assert_eq!(pos.rel_idx(0, 0), Pos { row: 255, col: 255 });
        assert_eq!(pos.rel_idx(-1, -1), Pos { row: 254, col: 254 });
        assert_eq!(pos.rel_idx(-2, -2), Pos { row: 253, col: 253 });
        assert_eq!(pos.rel_idx(-3, -3), Pos { row: 252, col: 252 });
        assert_eq!(pos.rel_idx(-4, -4), Pos { row: 251, col: 251 });
        assert_eq!(pos.rel_idx(-5, -5), Pos { row: 250, col: 250 });
        assert_eq!(pos.rel_idx(-6, -6), Pos { row: 249, col: 249 });
        assert_eq!(pos.rel_idx(-7, -7), Pos { row: 248, col: 248 });
    }

    #[test]
    fn try_of_row() {
        assert_eq!(Pos::try_of("A1"), Some(Pos { row: 0, col: 0 }));
        assert_eq!(Pos::try_of("A2"), Some(Pos { row: 1, col: 0 }));
        assert_eq!(Pos::try_of("A3"), Some(Pos { row: 2, col: 0 }));
        assert_eq!(Pos::try_of("A4"), Some(Pos { row: 3, col: 0 }));
        assert_eq!(Pos::try_of("A5"), Some(Pos { row: 4, col: 0 }));
        assert_eq!(Pos::try_of("A6"), Some(Pos { row: 5, col: 0 }));
        assert_eq!(Pos::try_of("A7"), Some(Pos { row: 6, col: 0 }));
        assert_eq!(Pos::try_of("A8"), Some(Pos { row: 7, col: 0 }));
        assert_eq!(Pos::try_of("A9"), Some(Pos { row: 8, col: 0 }));
    }

    #[test]
    fn try_of_col() {
        assert_eq!(Pos::try_of("A1"), Some(Pos { row: 0, col: 0 }));
        assert_eq!(Pos::try_of("B1"), Some(Pos { row: 0, col: 1 }));
        assert_eq!(Pos::try_of("C1"), Some(Pos { row: 0, col: 2 }));
        assert_eq!(Pos::try_of("D1"), Some(Pos { row: 0, col: 3 }));
        assert_eq!(Pos::try_of("E1"), Some(Pos { row: 0, col: 4 }));
        assert_eq!(Pos::try_of("F1"), Some(Pos { row: 0, col: 5 }));
        assert_eq!(Pos::try_of("G1"), Some(Pos { row: 0, col: 6 }));
        assert_eq!(Pos::try_of("H1"), Some(Pos { row: 0, col: 7 }));
        assert_eq!(Pos::try_of("I1"), Some(Pos { row: 0, col: 8 }));
    }

    #[test]
    fn try_of_multiple_characters() {
        assert_eq!(Pos::try_of("AA11"), Some(Pos { row: 10, col: 26 }));
        assert_eq!(Pos::try_of("CW101"), Some(Pos { row: 100, col: 100 }));
    }

    #[test]
    fn try_of_out_of_bounds() {
        assert_eq!(Pos::try_of("IW1"), None);
        assert_eq!(Pos::try_of("A0"), None);
    }

    #[test]
    fn of() {
        assert_eq!(Pos::of("A1"), Pos { row: 0, col: 0 });
        assert_eq!(Pos::of("A2"), Pos { row: 1, col: 0 });
        assert_eq!(Pos::of("A1"), Pos { row: 0, col: 0 });
        assert_eq!(Pos::of("B1"), Pos { row: 0, col: 1 });
    }

    #[test]
    fn to_string() {
        assert_eq!(&Pos { row: 0, col: 0 }.to_string(), "A1");
        assert_eq!(&Pos { row: 1, col: 0 }.to_string(), "A2");
        assert_eq!(&Pos { row: 2, col: 0 }.to_string(), "A3");
        assert_eq!(&Pos { row: 3, col: 0 }.to_string(), "A4");
        assert_eq!(&Pos { row: 4, col: 0 }.to_string(), "A5");
        assert_eq!(&Pos { row: 5, col: 0 }.to_string(), "A6");
        assert_eq!(&Pos { row: 6, col: 0 }.to_string(), "A7");
        assert_eq!(&Pos { row: 7, col: 0 }.to_string(), "A8");
        assert_eq!(&Pos { row: 7, col: 1 }.to_string(), "B8");
        assert_eq!(&Pos { row: 7, col: 2 }.to_string(), "C8");
        assert_eq!(&Pos { row: 7, col: 3 }.to_string(), "D8");
        assert_eq!(&Pos { row: 7, col: 4 }.to_string(), "E8");
        assert_eq!(&Pos { row: 7, col: 5 }.to_string(), "F8");
        assert_eq!(&Pos { row: 7, col: 6 }.to_string(), "G8");
        assert_eq!(&Pos { row: 7, col: 7 }.to_string(), "H8");
        assert_eq!(&Pos { row: 100, col: 100 }.to_string(), "CW101");
    }
}
