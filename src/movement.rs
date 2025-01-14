use crate::board::BoardPos;

fn white_pawn_mov(board_pos: &BoardPos) -> Vec<BoardPos> {
    let x = board_pos.x.to_idx();
    let y = board_pos.y.to_idx();
    if y == 6 {
        vec![BoardPos::from_idx(x, y - 1), BoardPos::from_idx(x, y - 2)]
    } else {
        vec![BoardPos::from_idx(x, y - 1)]
    }
}

fn black_pawn_mov(board_pos: &BoardPos) -> Vec<BoardPos> {
    let x = board_pos.x.to_idx();
    let y = board_pos.y.to_idx();
    if y == 1 {
        vec![BoardPos::from_idx(x, y + 1), BoardPos::from_idx(x, y + 2)]
    } else {
        vec![BoardPos::from_idx(x, y + 1)]
    }
}

fn rook_mov(board_pos: &BoardPos) -> Vec<BoardPos> {
    let x = board_pos.x.to_idx();
    let y = board_pos.y.to_idx();
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

fn knight_mov(board_pos: &BoardPos) -> Vec<BoardPos> {
    let x = board_pos.x.to_idx();
    let y = board_pos.y.to_idx();
    vec![
        BoardPos::try_from_idx(x - 2, y - 1),
        BoardPos::try_from_idx(x - 1, y - 2),
        BoardPos::try_from_idx(x + 1, y - 2),
        BoardPos::try_from_idx(x + 2, y - 1),
        BoardPos::try_from_idx(x - 2, y + 1),
        BoardPos::try_from_idx(x - 1, y + 2),
        BoardPos::try_from_idx(x + 1, y + 2),
        BoardPos::try_from_idx(x + 2, y + 1),
    ]
        .into_iter()
        .filter_map(|x| x)
        .collect()
}

fn bishop_movements(board_pos: &BoardPos) {
    let x = board_pos.x.to_idx();
    let y = board_pos.y.to_idx();
}

fn queen_movements(board_pos: &BoardPos) {
    let x = board_pos.x.to_idx();
    let y = board_pos.y.to_idx();
}

fn king_mov(board_pos: &BoardPos) -> Vec<BoardPos> {
    let x = board_pos.x.to_idx();
    let y = board_pos.y.to_idx();
    vec![
        if x > 0 && y > 0 { BoardPos::try_from_idx(x - 1, y - 1) } else { None },
        if y > 0 { BoardPos::try_from_idx(x, y - 1) } else { None },
        if y > 0 { BoardPos::try_from_idx(x + 1, y - 1) } else { None },
        if x > 0 { BoardPos::try_from_idx(x - 1, y) } else { None },
        BoardPos::try_from_idx(x + 1, y),
        if x > 0 { BoardPos::try_from_idx(x - 1, y + 1) } else { None },
        BoardPos::try_from_idx(x, y + 1),
        BoardPos::try_from_idx(x + 1, y + 1),
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
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E7")), [BoardPos::from_str("E6"), BoardPos::from_str("E5")]);
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E6")), [BoardPos::from_str("E5")]);
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E5")), [BoardPos::from_str("E4")]);
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E4")), [BoardPos::from_str("E3")]);
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E3")), [BoardPos::from_str("E2")]);
        assert_eq!(black_pawn_mov(&BoardPos::from_str("E2")), [BoardPos::from_str("E1")]);
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
        )
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
            [ BoardPos::from_str("A2"), BoardPos::from_str("B2"), BoardPos::from_str("B1"),]
        );
        assert_eq!(
            king_mov(&BoardPos::from_str("H1")),
            [ BoardPos::from_str("G2"), BoardPos::from_str("H2"), BoardPos::from_str("G1")]
        );
        assert_eq!(
            king_mov(&BoardPos::from_str("A8")),
            [ BoardPos::from_str("B8"), BoardPos::from_str("A7"), BoardPos::from_str("B7")]
        );
        assert_eq!(
            king_mov(&BoardPos::from_str("H8")),
            [ BoardPos::from_str("G8"), BoardPos::from_str("G7"), BoardPos::from_str("H7")]
        );
    }
}
