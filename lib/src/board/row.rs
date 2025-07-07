pub fn row_try_of(s: &str) -> Option<u8> {
    match s.parse::<u16>() {
        Ok(value) => {
            if value > 0 && value <= (u8::MAX as u16 + 1) {
                Some((value - 1) as u8)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn row_to_string(value: u8) -> String {
    ((value as u16) + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::{row_to_string, row_try_of};

    #[test]
    fn row_try_of_0_255() {
        assert_eq!(row_try_of("1"), Some(0));
        assert_eq!(row_try_of("2"), Some(1));
        assert_eq!(row_try_of("3"), Some(2));
        assert_eq!(row_try_of("4"), Some(3));
        assert_eq!(row_try_of("5"), Some(4));
        assert_eq!(row_try_of("6"), Some(5));
        assert_eq!(row_try_of("7"), Some(6));
        assert_eq!(row_try_of("8"), Some(7));
        assert_eq!(row_try_of("9"), Some(8));
        assert_eq!(row_try_of("16"), Some(15));
        assert_eq!(row_try_of("32"), Some(31));
        assert_eq!(row_try_of("64"), Some(63));
        assert_eq!(row_try_of("128"), Some(127));
        assert_eq!(row_try_of("256"), Some(255));
    }

    #[test]
    fn test_row_try_of_out_of_bounds() {
        assert_eq!(row_try_of("0"), None);
        assert_eq!(row_try_of("257"), None);
    }

    #[test]
    fn row_try_of_none() {
        assert_eq!(row_try_of(""), None);
        assert_eq!(row_try_of("a"), None);
    }

    #[test]
    fn test_row_to_string() {
        assert_eq!(row_to_string(0), "1");
        assert_eq!(row_to_string(1), "2");
        assert_eq!(row_to_string(2), "3");
        assert_eq!(row_to_string(3), "4");
        assert_eq!(row_to_string(4), "5");
        assert_eq!(row_to_string(5), "6");
        assert_eq!(row_to_string(6), "7");
        assert_eq!(row_to_string(7), "8");
        assert_eq!(row_to_string(15), "16");
        assert_eq!(row_to_string(31), "32");
        assert_eq!(row_to_string(63), "64");
        assert_eq!(row_to_string(127), "128");
        assert_eq!(row_to_string(255), "256");
    }
}
