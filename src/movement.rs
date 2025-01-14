use crate::board::BoardPos;

fn white_pawn_movements(board_pos: BoardPos) -> Vec<BoardPos> {
    let y = board_pos.y.to_idx();
    if y == 6 {
        vec![
            BoardPos::of_idx(board_pos.x.to_idx(), y - 1).unwrap(),
            BoardPos::of_idx(board_pos.x.to_idx(), y - 2).unwrap(),
        ]
    } else {
        vec![BoardPos::of_idx(board_pos.x.to_idx(), y - 1).unwrap()]
    }
}

fn black_pawn_movements(board_pos: BoardPos) -> Vec<BoardPos> {
    let y = board_pos.y.to_idx();
    if y == 1 {
        vec![
            BoardPos::of_idx(board_pos.x.to_idx(), y + 1).unwrap(),
            BoardPos::of_idx(board_pos.x.to_idx(), y + 2).unwrap(),
        ]
    } else {
        vec![BoardPos::of_idx(board_pos.x.to_idx(), y + 1).unwrap()]
    }
}

fn rook_movements(board_pos: BoardPos) -> Vec<BoardPos> {
    let y = board_pos.y.to_idx();
    if y == 6 {
        vec![
            BoardPos::of_idx(board_pos.x.to_idx(), y - 1).unwrap(),
            BoardPos::of_idx(board_pos.x.to_idx(), y - 2).unwrap(),
        ]
    } else {
        vec![BoardPos::of_idx(board_pos.x.to_idx(), y - 1).unwrap()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_white_pawn_movements() {
        assert_eq!(white_pawn_movements(BoardPos::of_str("E2").unwrap()), [BoardPos::of_str("E3").unwrap(), BoardPos::of_str("E4").unwrap()]);
        assert_eq!(white_pawn_movements(BoardPos::of_str("E3").unwrap()), [BoardPos::of_str("E4").unwrap()]);
        assert_eq!(white_pawn_movements(BoardPos::of_str("E4").unwrap()), [BoardPos::of_str("E5").unwrap()]);
        assert_eq!(white_pawn_movements(BoardPos::of_str("E5").unwrap()), [BoardPos::of_str("E6").unwrap()]);
        assert_eq!(white_pawn_movements(BoardPos::of_str("E6").unwrap()), [BoardPos::of_str("E7").unwrap()]);
        assert_eq!(white_pawn_movements(BoardPos::of_str("E7").unwrap()), [BoardPos::of_str("E8").unwrap()]);
    }

    #[test]
    fn test_black_pawn_movements() {
        assert_eq!(black_pawn_movements(BoardPos::of_str("E7").unwrap()), [BoardPos::of_str("E6").unwrap(), BoardPos::of_str("E5").unwrap()]);
        assert_eq!(black_pawn_movements(BoardPos::of_str("E6").unwrap()), [BoardPos::of_str("E5").unwrap()]);
        assert_eq!(black_pawn_movements(BoardPos::of_str("E5").unwrap()), [BoardPos::of_str("E4").unwrap()]);
        assert_eq!(black_pawn_movements(BoardPos::of_str("E4").unwrap()), [BoardPos::of_str("E3").unwrap()]);
        assert_eq!(black_pawn_movements(BoardPos::of_str("E3").unwrap()), [BoardPos::of_str("E2").unwrap()]);
        assert_eq!(black_pawn_movements(BoardPos::of_str("E2").unwrap()), [BoardPos::of_str("E1").unwrap()]);
    }
}
