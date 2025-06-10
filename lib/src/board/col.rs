
pub fn try_of_str(s: &str) -> Option<u8> {
    let mut chars = s.chars().rev().filter(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase()).peekable();
    if let None = chars.peek() {
        return None;
    }
    let mut result = 0;
    let mut power = 0;
    for c in chars {
        let mut as_ut8: [u8; 2] = [0; 2];
        c.encode_utf8(&mut as_ut8);
        let value = as_ut8[0] - 64;
        let value_in_position = value * (26 as u8).pow(power);
        result += value_in_position;
        power += 1;
    }
    Some(result - 1)
}

pub fn of_str(s: &str) -> u8 {
    try_of_str(s).unwrap()
}

pub fn to_str(value: u8) -> String {
    let mut result = String::new();
    let mut curr_value = value;

    let mut power = 0;
    let mut curr_value_power = curr_value;

    let mut div = (curr_value_power as f32) / (26 as f32);
    while div > 1.0 {
        power += 1;
        curr_value_power -= (26 as u8).pow(power);
        div = (curr_value_power as f32) / (26 as f32);
    }

    curr_value -= (26 as u8).pow(power);

    let code = curr_value + 65;
    result.push(char::from_u32(code.into()).unwrap());

    result
}


#[cfg(test)]
mod tests {
    use super::{try_of_str,of_str , to_str };

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
    }

    #[test]
    fn try_of_str_none() {
        assert_eq!(try_of_str(""), None);
        assert_eq!(try_of_str("123"), None);
        assert_eq!(try_of_str("a"), None);
    }

    #[test]
    fn test_of_str() {
        assert_eq!(of_str("A"), 0 );
        assert_eq!(of_str("B"), 1 );
        assert_eq!(of_str("C"), 2 );
    }

    #[test]
    fn test_to_str() {
        assert_eq!(to_str(0), "A".to_string());
        assert_eq!(to_str(26), "AA".to_string());
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
