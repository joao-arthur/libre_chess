pub fn try_of_str(s: &str) -> Option<u8> {
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

pub fn to_str(value: u8) -> String {
    ((value as u16) + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::{to_str, try_of_str};

    #[test]
    fn test_try_of_str() {
        assert_eq!(try_of_str("1"), Some(0));
        assert_eq!(try_of_str("2"), Some(1));
        assert_eq!(try_of_str("3"), Some(2));
        assert_eq!(try_of_str("4"), Some(3));
        assert_eq!(try_of_str("5"), Some(4));
        assert_eq!(try_of_str("6"), Some(5));
        assert_eq!(try_of_str("7"), Some(6));
        assert_eq!(try_of_str("8"), Some(7));
        assert_eq!(try_of_str("9"), Some(8));
        assert_eq!(try_of_str("16"), Some(15));
        assert_eq!(try_of_str("32"), Some(31));
        assert_eq!(try_of_str("64"), Some(63));
        assert_eq!(try_of_str("128"), Some(127));
        assert_eq!(try_of_str("256"), Some(255));
    }

    #[test]
    fn test_try_of_str_out_of_bounds() {
        assert_eq!(try_of_str("0"), None);
        assert_eq!(try_of_str("257"), None);
    }

    #[test]
    fn try_of_str_none() {
        assert_eq!(try_of_str(""), None);
        assert_eq!(try_of_str("a"), None);
    }

    #[test]
    fn test_to_str() {
        assert_eq!(to_str(0), "1");
        assert_eq!(to_str(1), "2");
        assert_eq!(to_str(2), "3");
        assert_eq!(to_str(3), "4");
        assert_eq!(to_str(4), "5");
        assert_eq!(to_str(5), "6");
        assert_eq!(to_str(6), "7");
        assert_eq!(to_str(7), "8");
        assert_eq!(to_str(15), "16");
        assert_eq!(to_str(31), "32");
        assert_eq!(to_str(63), "64");
        assert_eq!(to_str(127), "128");
        assert_eq!(to_str(255), "256");
    }
}
