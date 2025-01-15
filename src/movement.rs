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

fn rook_movements(board_pos: &BoardPos) -> Vec<BoardPos> {
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

    #[test]
    fn test_rook_movements() {
        assert_eq!(
            rook_movements(&BoardPos::from_str("D4")),
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
    }
}
