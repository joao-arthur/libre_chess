use crate::board::{board_pos::BoardPos, board_y::BoardY};

fn pawn_white_move(board_pos: BoardPos) -> Vec<BoardPos> {
    let y = board_pos.y.to_idx();

    vec![
        BoardPos { x: board_pos.x, y: BoardY::from_idx(y - 1).unwrap() },
        BoardPos { x: board_pos.x, y: BoardY::from_idx(y - 2).unwrap() },
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(
            pawn_white_move(BoardPos::from_str("D7").unwrap()),
            [BoardPos::from_str("D6").unwrap(), BoardPos::from_str("D5").unwrap()]
        );
        assert_eq!(
            pawn_white_move(BoardPos::from_str("E7").unwrap()),
            [BoardPos::from_str("E6").unwrap(), BoardPos::from_str("E5").unwrap()]
        );
    }
}
