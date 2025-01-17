use crate::board::BoardPos;

fn white_pawn_mov(pos: &BoardPos) -> Vec<BoardPos> {
    let y = pos.y.to_idx();
    if y == 6 {
        vec![pos.try_of_rel_idx(0, -1), pos.try_of_rel_idx(0, -2)]
            .into_iter()
            .filter_map(|x| x)
            .collect()
    } else {
        vec![pos.try_of_rel_idx(0, -1)].into_iter().filter_map(|x| x).collect()
    }
}

fn black_pawn_mov(pos: &BoardPos) -> Vec<BoardPos> {
    let x = pos.x.to_idx();
    let y = pos.y.to_idx();
    if y == 1 {
        vec![BoardPos::of_idx(x, y + 1), BoardPos::of_idx(x, y + 2)]
    } else {
        vec![BoardPos::of_idx(x, y + 1)]
    }
}

fn knight_mov(pos: &BoardPos) -> Vec<BoardPos> {
    vec![
        pos.try_of_rel_idx(-2, -1),
        pos.try_of_rel_idx(-1, -2),
        pos.try_of_rel_idx(1, -2),
        pos.try_of_rel_idx(2, -1),
        pos.try_of_rel_idx(-2, 1),
        pos.try_of_rel_idx(-1, 2),
        pos.try_of_rel_idx(1, 2),
        pos.try_of_rel_idx(2, 1),
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect()
}

fn rook_mov(pos: &BoardPos) -> Vec<BoardPos> {
    let x = pos.x.to_idx();
    let y = pos.y.to_idx();
    vec![
        BoardPos::of_idx(0, y),
        BoardPos::of_idx(1, y),
        BoardPos::of_idx(2, y),
        BoardPos::of_idx(3, y),
        BoardPos::of_idx(4, y),
        BoardPos::of_idx(5, y),
        BoardPos::of_idx(6, y),
        BoardPos::of_idx(7, y),
        BoardPos::of_idx(x, 0),
        BoardPos::of_idx(x, 1),
        BoardPos::of_idx(x, 2),
        BoardPos::of_idx(x, 3),
        BoardPos::of_idx(x, 4),
        BoardPos::of_idx(x, 5),
        BoardPos::of_idx(x, 6),
        BoardPos::of_idx(x, 7),
    ]
}

fn bishop_mov(pos: &BoardPos) -> Vec<BoardPos> {
    let x = pos.x.to_idx();
    let y = pos.y.to_idx();
    let mut res = vec![];
    let x_negative = 7 - x;
    if x > y {
        let x_offset = x - y;
        for x_idx in x_offset..=7 {
            res.push(BoardPos::try_of_idx(x_idx, x_idx - x_offset));
        }
    }
    if y >= x {
        let y_offset = y - x;
        for y_idx in y_offset..=7 {
            res.push(BoardPos::try_of_idx(y_idx - y_offset, y_idx));
        }
    }
    if x_negative >= y {
        let x_offset = x_negative - y;
        for x_idx in x_offset..=7 {
            res.push(BoardPos::try_of_idx(7 - x_idx, x_idx - x_offset));
        }
    }
    if y > x_negative {
        let y_offset = y - x_negative;
        for y_idx in y_offset..=7 {
            res.push(BoardPos::try_of_idx(7 - (y_idx - y_offset), y_idx));
        }
    }
    res.into_iter().filter_map(|x| x).collect()
}

fn queen_movements(pos: &BoardPos) {
    let x = pos.x.to_idx();
    let y = pos.y.to_idx();
}

fn king_mov(pos: &BoardPos) -> Vec<BoardPos> {
    vec![
        pos.try_of_rel_idx(-1, -1),
        pos.try_of_rel_idx(0, -1),
        pos.try_of_rel_idx(1, -1),
        pos.try_of_rel_idx(-1, 0),
        pos.try_of_rel_idx(1, 0),
        pos.try_of_rel_idx(-1, 1),
        pos.try_of_rel_idx(0, 1),
        pos.try_of_rel_idx(1, 1),
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_white_pawn_mov() {
        assert_eq!(
            white_pawn_mov(&BoardPos::of_str("E2")),
            [BoardPos::of_str("E3"), BoardPos::of_str("E4")]
        );
        assert_eq!(white_pawn_mov(&BoardPos::of_str("E3")), [BoardPos::of_str("E4")]);
        assert_eq!(white_pawn_mov(&BoardPos::of_str("E4")), [BoardPos::of_str("E5")]);
        assert_eq!(white_pawn_mov(&BoardPos::of_str("E5")), [BoardPos::of_str("E6")]);
        assert_eq!(white_pawn_mov(&BoardPos::of_str("E6")), [BoardPos::of_str("E7")]);
        assert_eq!(white_pawn_mov(&BoardPos::of_str("E7")), [BoardPos::of_str("E8")]);
    }

    #[test]
    fn test_black_pawn_mov() {
        assert_eq!(
            black_pawn_mov(&BoardPos::of_str("E7")),
            [BoardPos::of_str("E6"), BoardPos::of_str("E5")]
        );
        assert_eq!(black_pawn_mov(&BoardPos::of_str("E6")), [BoardPos::of_str("E5")]);
        assert_eq!(black_pawn_mov(&BoardPos::of_str("E5")), [BoardPos::of_str("E4")]);
        assert_eq!(black_pawn_mov(&BoardPos::of_str("E4")), [BoardPos::of_str("E3")]);
        assert_eq!(black_pawn_mov(&BoardPos::of_str("E3")), [BoardPos::of_str("E2")]);
        assert_eq!(black_pawn_mov(&BoardPos::of_str("E2")), [BoardPos::of_str("E1")]);
    }

    #[test]
    fn test_knight_mov() {
        assert_eq!(
            knight_mov(&BoardPos::of_str("D4")),
            [
                BoardPos::of_str("B5"),
                BoardPos::of_str("C6"),
                BoardPos::of_str("E6"),
                BoardPos::of_str("F5"),
                BoardPos::of_str("B3"),
                BoardPos::of_str("C2"),
                BoardPos::of_str("E2"),
                BoardPos::of_str("F3"),
            ]
        );
        assert_eq!(
            knight_mov(&BoardPos::of_str("A1")),
            [BoardPos::of_str("B3"), BoardPos::of_str("C2")]
        );
        assert_eq!(
            knight_mov(&BoardPos::of_str("A8")),
            [BoardPos::of_str("B6"), BoardPos::of_str("C7")]
        );
        assert_eq!(
            knight_mov(&BoardPos::of_str("H1")),
            [BoardPos::of_str("F2"), BoardPos::of_str("G3")]
        );
        assert_eq!(
            knight_mov(&BoardPos::of_str("H8")),
            [BoardPos::of_str("F7"), BoardPos::of_str("G6")]
        );
    }

    #[test]
    fn test_rook_mov() {
        assert_eq!(
            rook_mov(&BoardPos::of_str("D4")),
            [
                BoardPos::of_str("A4"),
                BoardPos::of_str("B4"),
                BoardPos::of_str("C4"),
                BoardPos::of_str("D4"),
                BoardPos::of_str("E4"),
                BoardPos::of_str("F4"),
                BoardPos::of_str("G4"),
                BoardPos::of_str("H4"),
                BoardPos::of_str("D8"),
                BoardPos::of_str("D7"),
                BoardPos::of_str("D6"),
                BoardPos::of_str("D5"),
                BoardPos::of_str("D4"),
                BoardPos::of_str("D3"),
                BoardPos::of_str("D2"),
                BoardPos::of_str("D1"),
            ]
        );
    }

    #[test]
    fn test_bishop_mov() {
        assert_eq!(
            bishop_mov(&BoardPos::of_str("C8")),
            [
                BoardPos::of_str("C8"),
                BoardPos::of_str("D7"),
                BoardPos::of_str("E6"),
                BoardPos::of_str("F5"),
                BoardPos::of_str("G4"),
                BoardPos::of_str("H3"),
                //
                BoardPos::of_str("C8"),
                BoardPos::of_str("B7"),
                BoardPos::of_str("A6"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("C7")),
            [
                BoardPos::of_str("B8"),
                BoardPos::of_str("C7"),
                BoardPos::of_str("D6"),
                BoardPos::of_str("E5"),
                BoardPos::of_str("F4"),
                BoardPos::of_str("G3"),
                BoardPos::of_str("H2"),
                //
                BoardPos::of_str("D8"),
                BoardPos::of_str("C7"),
                BoardPos::of_str("B6"),
                BoardPos::of_str("A5"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("C6")),
            [
                BoardPos::of_str("A8"),
                BoardPos::of_str("B7"),
                BoardPos::of_str("C6"),
                BoardPos::of_str("D5"),
                BoardPos::of_str("E4"),
                BoardPos::of_str("F3"),
                BoardPos::of_str("G2"),
                BoardPos::of_str("H1"),
                //
                BoardPos::of_str("E8"),
                BoardPos::of_str("D7"),
                BoardPos::of_str("C6"),
                BoardPos::of_str("B5"),
                BoardPos::of_str("A4"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("C5")),
            [
                BoardPos::of_str("A7"),
                BoardPos::of_str("B6"),
                BoardPos::of_str("C5"),
                BoardPos::of_str("D4"),
                BoardPos::of_str("E3"),
                BoardPos::of_str("F2"),
                BoardPos::of_str("G1"),
                //
                BoardPos::of_str("F8"),
                BoardPos::of_str("E7"),
                BoardPos::of_str("D6"),
                BoardPos::of_str("C5"),
                BoardPos::of_str("B4"),
                BoardPos::of_str("A3"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("C4")),
            [
                BoardPos::of_str("A6"),
                BoardPos::of_str("B5"),
                BoardPos::of_str("C4"),
                BoardPos::of_str("D3"),
                BoardPos::of_str("E2"),
                BoardPos::of_str("F1"),
                //
                BoardPos::of_str("G8"),
                BoardPos::of_str("F7"),
                BoardPos::of_str("E6"),
                BoardPos::of_str("D5"),
                BoardPos::of_str("C4"),
                BoardPos::of_str("B3"),
                BoardPos::of_str("A2"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("C3")),
            [
                BoardPos::of_str("A5"),
                BoardPos::of_str("B4"),
                BoardPos::of_str("C3"),
                BoardPos::of_str("D2"),
                BoardPos::of_str("E1"),
                //
                BoardPos::of_str("H8"),
                BoardPos::of_str("G7"),
                BoardPos::of_str("F6"),
                BoardPos::of_str("E5"),
                BoardPos::of_str("D4"),
                BoardPos::of_str("C3"),
                BoardPos::of_str("B2"),
                BoardPos::of_str("A1"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("C2")),
            [
                BoardPos::of_str("A4"),
                BoardPos::of_str("B3"),
                BoardPos::of_str("C2"),
                BoardPos::of_str("D1"),
                //
                BoardPos::of_str("H7"),
                BoardPos::of_str("G6"),
                BoardPos::of_str("F5"),
                BoardPos::of_str("E4"),
                BoardPos::of_str("D3"),
                BoardPos::of_str("C2"),
                BoardPos::of_str("B1"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("C1")),
            [
                BoardPos::of_str("A3"),
                BoardPos::of_str("B2"),
                BoardPos::of_str("C1"),
                //
                BoardPos::of_str("H6"),
                BoardPos::of_str("G5"),
                BoardPos::of_str("F4"),
                BoardPos::of_str("E3"),
                BoardPos::of_str("D2"),
                BoardPos::of_str("C1"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("A8")),
            [
                BoardPos::of_str("A8"),
                BoardPos::of_str("B7"),
                BoardPos::of_str("C6"),
                BoardPos::of_str("D5"),
                BoardPos::of_str("E4"),
                BoardPos::of_str("F3"),
                BoardPos::of_str("G2"),
                BoardPos::of_str("H1"),
                //
                BoardPos::of_str("A8"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("H1")),
            [
                BoardPos::of_str("A8"),
                BoardPos::of_str("B7"),
                BoardPos::of_str("C6"),
                BoardPos::of_str("D5"),
                BoardPos::of_str("E4"),
                BoardPos::of_str("F3"),
                BoardPos::of_str("G2"),
                BoardPos::of_str("H1"),
                //
                BoardPos::of_str("H1"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("H8")),
            [
                BoardPos::of_str("H8"),
                //
                BoardPos::of_str("H8"),
                BoardPos::of_str("G7"),
                BoardPos::of_str("F6"),
                BoardPos::of_str("E5"),
                BoardPos::of_str("D4"),
                BoardPos::of_str("C3"),
                BoardPos::of_str("B2"),
                BoardPos::of_str("A1"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::of_str("A1")),
            [
                BoardPos::of_str("A1"),
                //
                BoardPos::of_str("H8"),
                BoardPos::of_str("G7"),
                BoardPos::of_str("F6"),
                BoardPos::of_str("E5"),
                BoardPos::of_str("D4"),
                BoardPos::of_str("C3"),
                BoardPos::of_str("B2"),
                BoardPos::of_str("A1"),
            ]
        );
    }

    #[test]
    fn test_king_mov() {
        assert_eq!(
            king_mov(&BoardPos::of_str("D4")),
            [
                BoardPos::of_str("C5"),
                BoardPos::of_str("D5"),
                BoardPos::of_str("E5"),
                BoardPos::of_str("C4"),
                BoardPos::of_str("E4"),
                BoardPos::of_str("C3"),
                BoardPos::of_str("D3"),
                BoardPos::of_str("E3"),
            ]
        );
        assert_eq!(
            king_mov(&BoardPos::of_str("E1")),
            [
                BoardPos::of_str("D2"),
                BoardPos::of_str("E2"),
                BoardPos::of_str("F2"),
                BoardPos::of_str("D1"),
                BoardPos::of_str("F1"),
            ]
        );
        assert_eq!(
            king_mov(&BoardPos::of_str("A1")),
            [BoardPos::of_str("A2"), BoardPos::of_str("B2"), BoardPos::of_str("B1"),]
        );
        assert_eq!(
            king_mov(&BoardPos::of_str("H1")),
            [BoardPos::of_str("G2"), BoardPos::of_str("H2"), BoardPos::of_str("G1")]
        );
        assert_eq!(
            king_mov(&BoardPos::of_str("A8")),
            [BoardPos::of_str("B8"), BoardPos::of_str("A7"), BoardPos::of_str("B7")]
        );
        assert_eq!(
            king_mov(&BoardPos::of_str("H8")),
            [BoardPos::of_str("G8"), BoardPos::of_str("G7"), BoardPos::of_str("H7")]
        );
    }
}
