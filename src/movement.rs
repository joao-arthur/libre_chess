use crate::board::BoardPos;

fn white_pawn_movements(board_pos: &BoardPos) -> Vec<BoardPos> {
    let x = board_pos.x.to_idx();
    let y = board_pos.y.to_idx();
    if y == 6 {
        vec![BoardPos::from_idx(x, y - 1), BoardPos::from_idx(x, y - 2)]
    } else {
        vec![BoardPos::from_idx(x, y - 1)]
    }
}

fn black_pawn_movements(board_pos: &BoardPos) -> Vec<BoardPos> {
    let x = board_pos.x.to_idx();
    let y = board_pos.y.to_idx();
    if y == 1 {
        vec![BoardPos::from_idx(x, y + 1), BoardPos::from_idx(x, y + 2)]
    } else {
        vec![BoardPos::from_idx(x, y + 1)]
    }
}

fn rook_movements(board_pos: BoardPos) -> Vec<BoardPos> {
    let y = board_pos.y.to_idx();
    if y == 6 {
        vec![
            BoardPos::from_idx(board_pos.x.to_idx(), y - 1),
            BoardPos::from_idx(board_pos.x.to_idx(), y - 2),
        ]
    } else {
        vec![BoardPos::from_idx(board_pos.x.to_idx(), y - 1)]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_white_pawn_movements() {
        assert_eq!(
            white_pawn_movements(&BoardPos::from_str("E2")),
            [BoardPos::from_str("E3"), BoardPos::from_str("E4")]
        );
        assert_eq!(white_pawn_movements(&BoardPos::from_str("E3")), [BoardPos::from_str("E4")]);
        assert_eq!(white_pawn_movements(&BoardPos::from_str("E4")), [BoardPos::from_str("E5")]);
        assert_eq!(white_pawn_movements(&BoardPos::from_str("E5")), [BoardPos::from_str("E6")]);
        assert_eq!(white_pawn_movements(&BoardPos::from_str("E6")), [BoardPos::from_str("E7")]);
        assert_eq!(white_pawn_movements(&BoardPos::from_str("E7")), [BoardPos::from_str("E8")]);
    }

    #[test]
    fn test_black_pawn_movements() {
        assert_eq!(
            black_pawn_movements(&BoardPos::from_str("E7")),
            [BoardPos::from_str("E6"), BoardPos::from_str("E5")]
        );
        assert_eq!(black_pawn_movements(&BoardPos::from_str("E6")), [BoardPos::from_str("E5")]);
        assert_eq!(black_pawn_movements(&BoardPos::from_str("E5")), [BoardPos::from_str("E4")]);
        assert_eq!(black_pawn_movements(&BoardPos::from_str("E4")), [BoardPos::from_str("E3")]);
        assert_eq!(black_pawn_movements(&BoardPos::from_str("E3")), [BoardPos::from_str("E2")]);
        assert_eq!(black_pawn_movements(&BoardPos::from_str("E2")), [BoardPos::from_str("E1")]);
    }

    }
}
