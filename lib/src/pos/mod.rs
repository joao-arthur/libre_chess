use manfredo::matrix::point::point_u8::PointU8;

use self::{
    col::{col_to_string, col_try_of},
    row::{row_to_string, row_try_of},
};

mod col;
mod row;

pub type Pos = PointU8;

pub fn pos_try_rel_idx(pos: &Pos, row: i8, col: i8) -> Option<Pos> {
    if row < 0 && row.unsigned_abs() > pos.row {
        return None;
    }
    if col < 0 && col.unsigned_abs() > pos.col {
        return None;
    }
    if row > 0 && (row as u8) > u8::MAX - pos.row {
        return None;
    }
    if col > 0 && (col as u8) > u8::MAX - pos.col {
        return None;
    }
    Some(Pos {
        row: ((pos.row as i16) + row as i16) as u8,
        col: ((pos.col as i16) + col as i16) as u8,
    })
}

pub fn pos_rel_idx(pos: &Pos, row: i8, col: i8) -> Pos {
    pos_try_rel_idx(pos, row, col).unwrap()
}

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

fn pos_to_string(pos: &Pos) -> String {
    col_to_string(pos.col) + &row_to_string(pos.row)
}

#[cfg(test)]
mod tests {
    use super::{
        Pos,
        pos_try_rel_idx,
        pos_rel_idx,
        pos_try_of,
        pos_of,
        pos_to_string
    };

    #[test]
    fn try_rel_idx_negative() {
        let pos = Pos::of(255, 255);
        assert_eq!(pos_try_rel_idx(&pos, 0, 0), Some(Pos::of(255, 255)));
        assert_eq!(pos_try_rel_idx(&pos, -1, -1), Some(Pos::of(254, 254)));
        assert_eq!(pos_try_rel_idx(&pos, -2, -2), Some(Pos::of(253, 253)));
        assert_eq!(pos_try_rel_idx(&pos, -3, -3), Some(Pos::of(252, 252)));
        assert_eq!(pos_try_rel_idx(&pos, -4, -4), Some(Pos::of(251, 251)));
        assert_eq!(pos_try_rel_idx(&pos, -5, -5), Some(Pos::of(250, 250)));
        assert_eq!(pos_try_rel_idx(&pos, -6, -6), Some(Pos::of(249, 249)));
        assert_eq!(pos_try_rel_idx(&pos, -7, -7), Some(Pos::of(248, 248)));
    }

    #[test]
    fn try_rel_idx_positive_limit_row() {
        let pos = Pos::of(253, 253);
        assert_eq!(pos_try_rel_idx(&pos, 0, 0), Some(Pos::of(253, 253)));
        assert_eq!(pos_try_rel_idx(&pos, 1, 0), Some(Pos::of(254, 253)));
        assert_eq!(pos_try_rel_idx(&pos, 2, 0), Some(Pos::of(255, 253)));
        assert_eq!(pos_try_rel_idx(&pos, 3, 0), None);
        assert_eq!(pos_try_rel_idx(&pos, 4, 0), None);
        assert_eq!(pos_try_rel_idx(&pos, 5, 0), None);
    }

    #[test]
    fn try_rel_idx_positive_limit_col() {
        let pos = Pos::of(253, 253);
        assert_eq!(pos_try_rel_idx(&pos, 0, 0), Some(Pos::of(253, 253)));
        assert_eq!(pos_try_rel_idx(&pos, 0, 1), Some(Pos::of(253, 254)));
        assert_eq!(pos_try_rel_idx(&pos, 0, 2), Some(Pos::of(253, 255)));
        assert_eq!(pos_try_rel_idx(&pos, 0, 3), None);
        assert_eq!(pos_try_rel_idx(&pos, 0, 4), None);
        assert_eq!(pos_try_rel_idx(&pos, 0, 5), None);
    }

    #[test]
    fn try_rel_idx_negative_limit_row() {
        let pos = Pos::of(2, 2);
        assert_eq!(pos_try_rel_idx(&pos, 0, 0), Some(Pos::of(2, 2)));
        assert_eq!(pos_try_rel_idx(&pos, -1, 0), Some(Pos::of(1, 2)));
        assert_eq!(pos_try_rel_idx(&pos, -2, 0), Some(Pos::of(0, 2)));
        assert_eq!(pos_try_rel_idx(&pos, -3, 0), None);
        assert_eq!(pos_try_rel_idx(&pos, -4, 0), None);
        assert_eq!(pos_try_rel_idx(&pos, -5, 0), None);
    }

    #[test]
    fn try_rel_idx_negative_limit_col() {
        let pos = Pos::of(2, 2);
        assert_eq!(pos_try_rel_idx(&pos, 0, 0), Some(Pos::of(2, 2)));
        assert_eq!(pos_try_rel_idx(&pos, 0, -1), Some(Pos::of(2, 1)));
        assert_eq!(pos_try_rel_idx(&pos, 0, -2), Some(Pos::of(2, 0)));
        assert_eq!(pos_try_rel_idx(&pos, 0, -3), None);
        assert_eq!(pos_try_rel_idx(&pos, 0, -4), None);
        assert_eq!(pos_try_rel_idx(&pos, 0, -5), None);
    }

    #[test]
    fn rel_idx_0() {
        let pos = Pos::of(0, 0);
        assert_eq!(pos_rel_idx(&pos, 0, 0), Pos::of(0, 0));
        assert_eq!(pos_rel_idx(&pos, 1, 1), Pos::of(1, 1));
        assert_eq!(pos_rel_idx(&pos, 2, 2), Pos::of(2, 2));
        assert_eq!(pos_rel_idx(&pos, 3, 3), Pos::of(3, 3));
        assert_eq!(pos_rel_idx(&pos, 4, 4), Pos::of(4, 4));
        assert_eq!(pos_rel_idx(&pos, 5, 5), Pos::of(5, 5));
        assert_eq!(pos_rel_idx(&pos, 6, 6), Pos::of(6, 6));
        assert_eq!(pos_rel_idx(&pos, 7, 7), Pos::of(7, 7));
    }

    #[test]
    fn rel_idx_255() {
        let pos = Pos::of(255, 255);
        assert_eq!(pos_rel_idx(&pos, 0, 0), Pos::of(255, 255));
        assert_eq!(pos_rel_idx(&pos, -1, -1), Pos::of(254, 254));
        assert_eq!(pos_rel_idx(&pos, -2, -2), Pos::of(253, 253));
        assert_eq!(pos_rel_idx(&pos, -3, -3), Pos::of(252, 252));
        assert_eq!(pos_rel_idx(&pos, -4, -4), Pos::of(251, 251));
        assert_eq!(pos_rel_idx(&pos, -5, -5), Pos::of(250, 250));
        assert_eq!(pos_rel_idx(&pos, -6, -6), Pos::of(249, 249));
        assert_eq!(pos_rel_idx(&pos, -7, -7), Pos::of(248, 248));
    }

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
