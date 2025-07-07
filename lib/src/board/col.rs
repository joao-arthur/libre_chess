pub fn col_try_of(s: &str) -> Option<u8> {
    let bytes = s.as_bytes();
    match bytes {
        [c] if c.is_ascii_uppercase() => Some(c - b'A'),
        [hi, lo] if hi.is_ascii_uppercase() && lo.is_ascii_uppercase() => {
            let hi_index = hi - b'A';
            let lo_index = lo - b'A';
            let value = ((hi_index as u16 + 1) * 26) + lo_index as u16;
            if value <= u8::MAX as u16 { Some(value as u8) } else { None }
        }
        _ => None,
    }
}

pub fn col_to_string(value: u8) -> String {
    if value < 26 {
        let c = (b'A' + value) as char;
        c.to_string()
    } else {
        let q = value / 26;
        let r = value % 26;
        let prefix = (b'A' + (q - 1)) as char;
        let suffix = (b'A' + r) as char;
        let mut s = String::with_capacity(2);
        s.push(prefix);
        s.push(suffix);
        s
    }
}

#[cfg(test)]
mod tests {
    use super::{col_to_string, col_try_of};

    #[test]
    fn col_try_of_a_z() {
        assert_eq!(col_try_of("A"), Some(0));
        assert_eq!(col_try_of("B"), Some(1));
        assert_eq!(col_try_of("C"), Some(2));
        assert_eq!(col_try_of("D"), Some(3));
        assert_eq!(col_try_of("E"), Some(4));
        assert_eq!(col_try_of("F"), Some(5));
        assert_eq!(col_try_of("G"), Some(6));
        assert_eq!(col_try_of("H"), Some(7));
        assert_eq!(col_try_of("I"), Some(8));
        assert_eq!(col_try_of("J"), Some(9));
        assert_eq!(col_try_of("K"), Some(10));
        assert_eq!(col_try_of("L"), Some(11));
        assert_eq!(col_try_of("M"), Some(12));
        assert_eq!(col_try_of("N"), Some(13));
        assert_eq!(col_try_of("O"), Some(14));
        assert_eq!(col_try_of("P"), Some(15));
        assert_eq!(col_try_of("Q"), Some(16));
        assert_eq!(col_try_of("R"), Some(17));
        assert_eq!(col_try_of("S"), Some(18));
        assert_eq!(col_try_of("T"), Some(19));
        assert_eq!(col_try_of("U"), Some(20));
        assert_eq!(col_try_of("V"), Some(21));
        assert_eq!(col_try_of("W"), Some(22));
        assert_eq!(col_try_of("X"), Some(23));
        assert_eq!(col_try_of("Y"), Some(24));
        assert_eq!(col_try_of("Z"), Some(25));
    }

    #[test]
    fn col_try_of_aa_iu() {
        assert_eq!(col_try_of("AA"), Some(26));
        assert_eq!(col_try_of("AZ"), Some(51));
        assert_eq!(col_try_of("BA"), Some(52));
        assert_eq!(col_try_of("BZ"), Some(77));
        assert_eq!(col_try_of("CA"), Some(78));
        assert_eq!(col_try_of("CZ"), Some(103));
        assert_eq!(col_try_of("DA"), Some(104));
        assert_eq!(col_try_of("DZ"), Some(129));
        assert_eq!(col_try_of("EA"), Some(130));
        assert_eq!(col_try_of("EZ"), Some(155));
        assert_eq!(col_try_of("FA"), Some(156));
        assert_eq!(col_try_of("FZ"), Some(181));
        assert_eq!(col_try_of("GA"), Some(182));
        assert_eq!(col_try_of("GZ"), Some(207));
        assert_eq!(col_try_of("HA"), Some(208));
        assert_eq!(col_try_of("HZ"), Some(233));
        assert_eq!(col_try_of("IA"), Some(234));
        assert_eq!(col_try_of("IU"), Some(254));
        assert_eq!(col_try_of("IV"), Some(255));
    }

    #[test]
    fn col_try_of_out_of_bounds() {
        assert_eq!(col_try_of("IW"), None);
        assert_eq!(col_try_of("ZZ"), None);
    }

    #[test]
    fn col_try_of_none() {
        assert_eq!(col_try_of(""), None);
        assert_eq!(col_try_of("123"), None);
        assert_eq!(col_try_of("a"), None);
    }

    #[test]
    fn test_col_to_string() {
        assert_eq!(&col_to_string(0), "A");
        assert_eq!(&col_to_string(25), "Z");
        assert_eq!(&col_to_string(26), "AA");
        assert_eq!(&col_to_string(51), "AZ");
        assert_eq!(&col_to_string(52), "BA");
        assert_eq!(&col_to_string(78), "CA");
        assert_eq!(&col_to_string(104), "DA");
        assert_eq!(&col_to_string(130), "EA");
        assert_eq!(&col_to_string(156), "FA");
        assert_eq!(&col_to_string(182), "GA");
        assert_eq!(&col_to_string(208), "HA");
        assert_eq!(&col_to_string(234), "IA");
    }
}
