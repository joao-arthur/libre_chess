#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RectU8 {
    pub x1: u8,
    pub y1: u8,
    pub x2: u8,
    pub y2: u8,
}

impl RectU8 {
    pub fn of(x1: u8, y1: u8, x2: u8, y2: u8) -> Self {
        RectU8 { x1, y1, x2, y2 }
    }
}

#[cfg(test)]
mod tests {
    use super::RectU8;

    #[test]
    fn test_rect_u8() {
        assert_eq!(RectU8::of(1, 2, 3, 4), RectU8 { x1: 1, y1: 2, x2: 3, y2: 4 });
    }
}
