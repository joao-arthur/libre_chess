use crate::board::BoardPos;

fn white_pawn_mov(pos: &BoardPos) -> Vec<BoardPos> {
    let y = pos.y.to_idx();
    if y == 6 {
        vec![pos.try_from_rel_idx(0, -1), pos.try_from_rel_idx(0, -2)]
            .into_iter()
            .filter_map(|x| x)
            .collect()
    } else {
        vec![pos.try_from_rel_idx(0, -1)].into_iter().filter_map(|x| x).collect()
    }
}

fn black_pawn_mov(pos: &BoardPos) -> Vec<BoardPos> {
    let x = pos.x.to_idx();
    let y = pos.y.to_idx();
    if y == 1 {
        vec![BoardPos::from_idx(x, y + 1), BoardPos::from_idx(x, y + 2)]
    } else {
        vec![BoardPos::from_idx(x, y + 1)]
    }
}

fn knight_mov(pos: &BoardPos) -> Vec<BoardPos> {
    vec![
        pos.try_from_rel_idx(-2, -1),
        pos.try_from_rel_idx(-1, -2),
        pos.try_from_rel_idx(1, -2),
        pos.try_from_rel_idx(2, -1),
        pos.try_from_rel_idx(-2, 1),
        pos.try_from_rel_idx(-1, 2),
        pos.try_from_rel_idx(1, 2),
        pos.try_from_rel_idx(2, 1),
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect()
}

fn rook_mov(pos: &BoardPos) -> Vec<BoardPos> {
    let x = pos.x.to_idx();
    let y = pos.y.to_idx();
    vec![
        BoardPos::from_idx(0, y),
        BoardPos::from_idx(1, y),
        BoardPos::from_idx(2, y),
        BoardPos::from_idx(3, y),
        BoardPos::from_idx(4, y),
        BoardPos::from_idx(5, y),
        BoardPos::from_idx(6, y),
        BoardPos::from_idx(7, y),
        BoardPos::from_idx(x, 0),
        BoardPos::from_idx(x, 1),
        BoardPos::from_idx(x, 2),
        BoardPos::from_idx(x, 3),
        BoardPos::from_idx(x, 4),
        BoardPos::from_idx(x, 5),
        BoardPos::from_idx(x, 6),
        BoardPos::from_idx(x, 7),
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
            res.push(BoardPos::try_from_idx(x_idx, x_idx - x_offset));
        }
    }
    if y >= x {
        let y_offset = y - x;
        for y_idx in y_offset..=7 {
            res.push(BoardPos::try_from_idx(y_idx - y_offset, y_idx));
        }
    }
    if x_negative >= y {
        let x_offset = x_negative - y;
        for x_idx in x_offset..=7 {
            res.push(BoardPos::try_from_idx(7 - x_idx, x_idx - x_offset));
        }
    }
    if y > x_negative {
        let y_offset = y - x_negative;
        for y_idx in y_offset..=7 {
            res.push(BoardPos::try_from_idx(7 - (y_idx - y_offset), y_idx));
        }
    }

    return res.into_iter().filter_map(|x| x).collect();
}

fn queen_movements(pos: &BoardPos) {
    let x = pos.x.to_idx();
    let y = pos.y.to_idx();
}

fn king_mov(pos: &BoardPos) -> Vec<BoardPos> {
    vec![
        pos.try_from_rel_idx(-1, -1),
        pos.try_from_rel_idx(0, -1),
        pos.try_from_rel_idx(1, -1),
        pos.try_from_rel_idx(-1, 0),
        pos.try_from_rel_idx(1, 0),
        pos.try_from_rel_idx(-1, 1),
        pos.try_from_rel_idx(0, 1),
        pos.try_from_rel_idx(1, 1),
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
            white_pawn_mov(&BoardPos::from_str("E2")),
            [BoardPos::from_str("E3"), BoardPos::from_str("E4")]
        );
        assert_eq!(white_pawn_mov(&BoardPos::from_str("E3")), [BoardPos::from_str("E4")]);
        assert_eq!(white_pawn_mov(&BoardPos::from_str("E4")), [BoardPos::from_str("E5")]);
        assert_eq!(white_pawn_mov(&BoardPos::from_str("E5")), [BoardPos::from_str("E6")]);
        assert_eq!(white_pawn_mov(&BoardPos::from_str("E6")), [BoardPos::from_str("E7")]);
        assert_eq!(white_pawn_mov(&BoardPos::from_str("E7")), [BoardPos::from_str("E8")]);
    }

    #[test]
    fn test_black_pawn_mov() {
        assert_eq!(
            black_pawn_mov(&BoardPos::from_str("E7")),
            [BoardPos::from_str("E6"), BoardPos::from_str("E5")]
        );
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E6")), [BoardPos::from_str("E5")]);
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E5")), [BoardPos::from_str("E4")]);
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E4")), [BoardPos::from_str("E3")]);
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E3")), [BoardPos::from_str("E2")]);
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E2")), [BoardPos::from_str("E1")]);
    }

    #[test]
    fn test_knight_mov() {
        assert_eq!(
            knight_mov(&BoardPos::from_str("D4")),
            [
                BoardPos::from_str("B5"),
                BoardPos::from_str("C6"),
                BoardPos::from_str("E6"),
                BoardPos::from_str("F5"),
                BoardPos::from_str("B3"),
                BoardPos::from_str("C2"),
                BoardPos::from_str("E2"),
                BoardPos::from_str("F3"),
            ]
        );
        assert_eq!(
            knight_mov(&BoardPos::from_str("A1")),
            [BoardPos::from_str("B3"), BoardPos::from_str("C2")]
        );
        assert_eq!(
            knight_mov(&BoardPos::from_str("A8")),
            [BoardPos::from_str("B6"), BoardPos::from_str("C7")]
        );
        assert_eq!(
            knight_mov(&BoardPos::from_str("H1")),
            [BoardPos::from_str("F2"), BoardPos::from_str("G3")]
        );
        assert_eq!(
            knight_mov(&BoardPos::from_str("H8")),
            [BoardPos::from_str("F7"), BoardPos::from_str("G6")]
        );
    }

    #[test]
    fn test_rook_mov() {
        assert_eq!(
            rook_mov(&BoardPos::from_str("D4")),
            [
                BoardPos::from_str("A4"),
                BoardPos::from_str("B4"),
                BoardPos::from_str("C4"),
                BoardPos::from_str("D4"),
                BoardPos::from_str("E4"),
                BoardPos::from_str("F4"),
                BoardPos::from_str("G4"),
                BoardPos::from_str("H4"),
                BoardPos::from_str("D8"),
                BoardPos::from_str("D7"),
                BoardPos::from_str("D6"),
                BoardPos::from_str("D5"),
                BoardPos::from_str("D4"),
                BoardPos::from_str("D3"),
                BoardPos::from_str("D2"),
                BoardPos::from_str("D1"),
            ]
        );
    }

    #[test]
    fn test_bishop_mov() {
        assert_eq!(
            bishop_mov(&BoardPos::from_str("C8")),
            [
                BoardPos::from_str("C8"),
                BoardPos::from_str("D7"),
                BoardPos::from_str("E6"),
                BoardPos::from_str("F5"),
                BoardPos::from_str("G4"),
                BoardPos::from_str("H3"),
                //
                BoardPos::from_str("C8"),
                BoardPos::from_str("B7"),
                BoardPos::from_str("A6"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("C7")),
            [
                BoardPos::from_str("B8"),
                BoardPos::from_str("C7"),
                BoardPos::from_str("D6"),
                BoardPos::from_str("E5"),
                BoardPos::from_str("F4"),
                BoardPos::from_str("G3"),
                BoardPos::from_str("H2"),
                //
                BoardPos::from_str("D8"),
                BoardPos::from_str("C7"),
                BoardPos::from_str("B6"),
                BoardPos::from_str("A5"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("C6")),
            [
                BoardPos::from_str("A8"),
                BoardPos::from_str("B7"),
                BoardPos::from_str("C6"),
                BoardPos::from_str("D5"),
                BoardPos::from_str("E4"),
                BoardPos::from_str("F3"),
                BoardPos::from_str("G2"),
                BoardPos::from_str("H1"),
                //
                BoardPos::from_str("E8"),
                BoardPos::from_str("D7"),
                BoardPos::from_str("C6"),
                BoardPos::from_str("B5"),
                BoardPos::from_str("A4"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("C5")),
            [
                BoardPos::from_str("A7"),
                BoardPos::from_str("B6"),
                BoardPos::from_str("C5"),
                BoardPos::from_str("D4"),
                BoardPos::from_str("E3"),
                BoardPos::from_str("F2"),
                BoardPos::from_str("G1"),
                //
                BoardPos::from_str("F8"),
                BoardPos::from_str("E7"),
                BoardPos::from_str("D6"),
                BoardPos::from_str("C5"),
                BoardPos::from_str("B4"),
                BoardPos::from_str("A3"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("C4")),
            [
                BoardPos::from_str("A6"),
                BoardPos::from_str("B5"),
                BoardPos::from_str("C4"),
                BoardPos::from_str("D3"),
                BoardPos::from_str("E2"),
                BoardPos::from_str("F1"),
                //
                BoardPos::from_str("G8"),
                BoardPos::from_str("F7"),
                BoardPos::from_str("E6"),
                BoardPos::from_str("D5"),
                BoardPos::from_str("C4"),
                BoardPos::from_str("B3"),
                BoardPos::from_str("A2"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("C3")),
            [
                BoardPos::from_str("A5"),
                BoardPos::from_str("B4"),
                BoardPos::from_str("C3"),
                BoardPos::from_str("D2"),
                BoardPos::from_str("E1"),
                //
                BoardPos::from_str("H8"),
                BoardPos::from_str("G7"),
                BoardPos::from_str("F6"),
                BoardPos::from_str("E5"),
                BoardPos::from_str("D4"),
                BoardPos::from_str("C3"),
                BoardPos::from_str("B2"),
                BoardPos::from_str("A1"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("C2")),
            [
                BoardPos::from_str("A4"),
                BoardPos::from_str("B3"),
                BoardPos::from_str("C2"),
                BoardPos::from_str("D1"),
                //
                BoardPos::from_str("H7"),
                BoardPos::from_str("G6"),
                BoardPos::from_str("F5"),
                BoardPos::from_str("E4"),
                BoardPos::from_str("D3"),
                BoardPos::from_str("C2"),
                BoardPos::from_str("B1"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("C1")),
            [
                BoardPos::from_str("A3"),
                BoardPos::from_str("B2"),
                BoardPos::from_str("C1"),
                //
                BoardPos::from_str("H6"),
                BoardPos::from_str("G5"),
                BoardPos::from_str("F4"),
                BoardPos::from_str("E3"),
                BoardPos::from_str("D2"),
                BoardPos::from_str("C1"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("A8")),
            [
                BoardPos::from_str("A8"),
                BoardPos::from_str("B7"),
                BoardPos::from_str("C6"),
                BoardPos::from_str("D5"),
                BoardPos::from_str("E4"),
                BoardPos::from_str("F3"),
                BoardPos::from_str("G2"),
                BoardPos::from_str("H1"),
                //
                BoardPos::from_str("A8"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("H1")),
            [
                BoardPos::from_str("A8"),
                BoardPos::from_str("B7"),
                BoardPos::from_str("C6"),
                BoardPos::from_str("D5"),
                BoardPos::from_str("E4"),
                BoardPos::from_str("F3"),
                BoardPos::from_str("G2"),
                BoardPos::from_str("H1"),
                //
                BoardPos::from_str("H1"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("H8")),
            [
                BoardPos::from_str("H8"),
                //
                BoardPos::from_str("H8"),
                BoardPos::from_str("G7"),
                BoardPos::from_str("F6"),
                BoardPos::from_str("E5"),
                BoardPos::from_str("D4"),
                BoardPos::from_str("C3"),
                BoardPos::from_str("B2"),
                BoardPos::from_str("A1"),
            ]
        );
        assert_eq!(
            bishop_mov(&BoardPos::from_str("A1")),
            [
                BoardPos::from_str("A1"),
                //
                BoardPos::from_str("H8"),
                BoardPos::from_str("G7"),
                BoardPos::from_str("F6"),
                BoardPos::from_str("E5"),
                BoardPos::from_str("D4"),
                BoardPos::from_str("C3"),
                BoardPos::from_str("B2"),
                BoardPos::from_str("A1"),
            ]
        );
    }

    #[test]
    fn test_king_mov() {
        assert_eq!(
            king_mov(&BoardPos::from_str("D4")),
            [
                BoardPos::from_str("C5"),
                BoardPos::from_str("D5"),
                BoardPos::from_str("E5"),
                BoardPos::from_str("C4"),
                BoardPos::from_str("E4"),
                BoardPos::from_str("C3"),
                BoardPos::from_str("D3"),
                BoardPos::from_str("E3"),
            ]
        );
        assert_eq!(
            king_mov(&BoardPos::from_str("E1")),
            [
                BoardPos::from_str("D2"),
                BoardPos::from_str("E2"),
                BoardPos::from_str("F2"),
                BoardPos::from_str("D1"),
                BoardPos::from_str("F1"),
            ]
        );
        assert_eq!(
            king_mov(&BoardPos::from_str("A1")),
            [BoardPos::from_str("A2"), BoardPos::from_str("B2"), BoardPos::from_str("B1"),]
        );
        assert_eq!(
            king_mov(&BoardPos::from_str("H1")),
            [BoardPos::from_str("G2"), BoardPos::from_str("H2"), BoardPos::from_str("G1")]
        );
        assert_eq!(
            king_mov(&BoardPos::from_str("A8")),
            [BoardPos::from_str("B8"), BoardPos::from_str("A7"), BoardPos::from_str("B7")]
        );
        assert_eq!(
            king_mov(&BoardPos::from_str("H8")),
            [BoardPos::from_str("G8"), BoardPos::from_str("G7"), BoardPos::from_str("H7")]
        );
    }
}
