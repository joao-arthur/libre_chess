use manfredo::matrix::point::point_u8::PointU8;

use self::{
    col::{col_to_string, col_try_of},
    row::{row_to_string, row_try_of},
};

mod col;
mod row;

pub type Pos = PointU8;

pub fn pos_try_of(s: &str) -> Option<Pos> {
    let mut pos_col = String::new();
    let mut pos_row = String::new();
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            pos_col.push(c);
        }
        if c.is_ascii_digit() {
            pos_row.push(c);
        }
    }
    Some(Pos::of(row_try_of(&pos_row)?, col_try_of(&pos_col)?))
}

pub fn pos_of(s: &str) -> Pos {
    pos_try_of(s).unwrap()
}

pub fn pos_to_string(pos: &Pos) -> String {
    col_to_string(pos.col) + &row_to_string(pos.row)
}

#[cfg(test)]
mod tests {
    use super::{Pos, pos_of, pos_to_string, pos_try_of};

    #[test]
    fn try_of_row() {
        assert_eq!(pos_try_of("A1"), Some(Pos::of(0, 0)));
        assert_eq!(pos_try_of("A2"), Some(Pos::of(1, 0)));
        assert_eq!(pos_try_of("A3"), Some(Pos::of(2, 0)));
        assert_eq!(pos_try_of("A4"), Some(Pos::of(3, 0)));
        assert_eq!(pos_try_of("A5"), Some(Pos::of(4, 0)));
        assert_eq!(pos_try_of("A6"), Some(Pos::of(5, 0)));
        assert_eq!(pos_try_of("A7"), Some(Pos::of(6, 0)));
        assert_eq!(pos_try_of("A8"), Some(Pos::of(7, 0)));
        assert_eq!(pos_try_of("A9"), Some(Pos::of(8, 0)));
    }

    #[test]
    fn try_of_col() {
        assert_eq!(pos_try_of("A1"), Some(Pos::of(0, 0)));
        assert_eq!(pos_try_of("B1"), Some(Pos::of(0, 1)));
        assert_eq!(pos_try_of("C1"), Some(Pos::of(0, 2)));
        assert_eq!(pos_try_of("D1"), Some(Pos::of(0, 3)));
        assert_eq!(pos_try_of("E1"), Some(Pos::of(0, 4)));
        assert_eq!(pos_try_of("F1"), Some(Pos::of(0, 5)));
        assert_eq!(pos_try_of("G1"), Some(Pos::of(0, 6)));
        assert_eq!(pos_try_of("H1"), Some(Pos::of(0, 7)));
        assert_eq!(pos_try_of("I1"), Some(Pos::of(0, 8)));
    }

    #[test]
    fn try_of_multiple_characters() {
        assert_eq!(pos_try_of("AA11"), Some(Pos::of(10, 26)));
        assert_eq!(pos_try_of("CW101"), Some(Pos::of(100, 100)));
    }

    #[test]
    fn try_of_out_of_bounds() {
        assert_eq!(pos_try_of("IW1"), None);
        assert_eq!(pos_try_of("A0"), None);
    }

    #[test]
    fn of() {
        assert_eq!(pos_of("A1"), Pos::of(0, 0));
        assert_eq!(pos_of("A2"), Pos::of(1, 0));
        assert_eq!(pos_of("A1"), Pos::of(0, 0));
        assert_eq!(pos_of("B1"), Pos::of(0, 1));
    }

    #[test]
    fn to_string() {
        assert_eq!(pos_to_string(&Pos::of(0, 0)), "A1");
        assert_eq!(pos_to_string(&Pos::of(1, 0)), "A2");
        assert_eq!(pos_to_string(&Pos::of(2, 0)), "A3");
        assert_eq!(pos_to_string(&Pos::of(3, 0)), "A4");
        assert_eq!(pos_to_string(&Pos::of(4, 0)), "A5");
        assert_eq!(pos_to_string(&Pos::of(5, 0)), "A6");
        assert_eq!(pos_to_string(&Pos::of(6, 0)), "A7");
        assert_eq!(pos_to_string(&Pos::of(7, 0)), "A8");
        assert_eq!(pos_to_string(&Pos::of(7, 1)), "B8");
        assert_eq!(pos_to_string(&Pos::of(7, 2)), "C8");
        assert_eq!(pos_to_string(&Pos::of(7, 3)), "D8");
        assert_eq!(pos_to_string(&Pos::of(7, 4)), "E8");
        assert_eq!(pos_to_string(&Pos::of(7, 5)), "F8");
        assert_eq!(pos_to_string(&Pos::of(7, 6)), "G8");
        assert_eq!(pos_to_string(&Pos::of(7, 7)), "H8");
        assert_eq!(pos_to_string(&Pos::of(100, 100)), "CW101");
    }
}
