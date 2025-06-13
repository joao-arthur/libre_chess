pub fn try_of_str(s: &str) -> Option<u8> {
    let bytes = s.as_bytes();
    match bytes {
        [c] if (b'A'..=b'Z').contains(c) => Some(c - b'A'),
        [hi, lo] if (b'A'..=b'Z').contains(hi) && (b'A'..=b'Z').contains(lo) => {
            let hi_index = hi - b'A';
            let lo_index = lo - b'A';
            let value = ((hi_index as u16 + 1) * 26) + lo_index as u16;
            if value <= u8::MAX as u16 { Some(value as u8) } else { None }
        }
        _ => None,
    }
}

pub fn of_str(s: &str) -> u8 {
    try_of_str(s).unwrap()
}

pub fn to_str(value: u8) -> String {
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
    use super::{of_str, to_str, try_of_str};

    #[test]
    fn try_of_str_a_z() {
        assert_eq!(try_of_str("A"), Some(0));
        assert_eq!(try_of_str("B"), Some(1));
        assert_eq!(try_of_str("C"), Some(2));
        assert_eq!(try_of_str("D"), Some(3));
        assert_eq!(try_of_str("E"), Some(4));
        assert_eq!(try_of_str("F"), Some(5));
        assert_eq!(try_of_str("G"), Some(6));
        assert_eq!(try_of_str("H"), Some(7));
        assert_eq!(try_of_str("I"), Some(8));
        assert_eq!(try_of_str("J"), Some(9));
        assert_eq!(try_of_str("K"), Some(10));
        assert_eq!(try_of_str("L"), Some(11));
        assert_eq!(try_of_str("M"), Some(12));
        assert_eq!(try_of_str("N"), Some(13));
        assert_eq!(try_of_str("O"), Some(14));
        assert_eq!(try_of_str("P"), Some(15));
        assert_eq!(try_of_str("Q"), Some(16));
        assert_eq!(try_of_str("R"), Some(17));
        assert_eq!(try_of_str("S"), Some(18));
        assert_eq!(try_of_str("T"), Some(19));
        assert_eq!(try_of_str("U"), Some(20));
        assert_eq!(try_of_str("V"), Some(21));
        assert_eq!(try_of_str("W"), Some(22));
        assert_eq!(try_of_str("X"), Some(23));
        assert_eq!(try_of_str("Y"), Some(24));
        assert_eq!(try_of_str("Z"), Some(25));
    }

    #[test]
    fn try_of_str_aa_iu() {
        assert_eq!(try_of_str("AA"), Some(26));
        assert_eq!(try_of_str("AZ"), Some(51));
        assert_eq!(try_of_str("BA"), Some(52));
        assert_eq!(try_of_str("BZ"), Some(77));
        assert_eq!(try_of_str("CA"), Some(78));
        assert_eq!(try_of_str("CZ"), Some(103));
        assert_eq!(try_of_str("DA"), Some(104));
        assert_eq!(try_of_str("DZ"), Some(129));
        assert_eq!(try_of_str("EA"), Some(130));
        assert_eq!(try_of_str("EZ"), Some(155));
        assert_eq!(try_of_str("FA"), Some(156));
        assert_eq!(try_of_str("FZ"), Some(181));
        assert_eq!(try_of_str("GA"), Some(182));
        assert_eq!(try_of_str("GZ"), Some(207));
        assert_eq!(try_of_str("HA"), Some(208));
        assert_eq!(try_of_str("HZ"), Some(233));
        assert_eq!(try_of_str("IA"), Some(234));
        assert_eq!(try_of_str("IU"), Some(254));
        assert_eq!(try_of_str("IV"), Some(255));
    }

    #[test]
    fn try_of_str_out_of_bounds() {
        assert_eq!(try_of_str("IW"), None);
        assert_eq!(try_of_str("ZZ"), None);
    }

    #[test]
    fn try_of_str_none() {
        assert_eq!(try_of_str(""), None);
        assert_eq!(try_of_str("123"), None);
        assert_eq!(try_of_str("a"), None);
    }

    #[test]
    fn test_of_str() {
        assert_eq!(of_str("A"), 0);
        assert_eq!(of_str("B"), 1);
        assert_eq!(of_str("C"), 2);
    }

    #[test]
    fn test_to_str() {
        assert_eq!(to_str(0), "A".to_string());
        assert_eq!(to_str(25), "Z".to_string());
        assert_eq!(to_str(26), "AA".to_string());
        assert_eq!(to_str(51), "AZ".to_string());
        assert_eq!(to_str(52), "BA".to_string());
        assert_eq!(to_str(78), "CA".to_string());
        assert_eq!(to_str(104), "DA".to_string());
        assert_eq!(to_str(130), "EA".to_string());
        assert_eq!(to_str(156), "FA".to_string());
        assert_eq!(to_str(182), "GA".to_string());
        assert_eq!(to_str(208), "HA".to_string());
        assert_eq!(to_str(234), "IA".to_string());
    }
}
