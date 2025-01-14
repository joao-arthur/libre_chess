use crate::board::{board_pos::BoardPos, board_y};

fn pawn_white_move(board_pos: BoardPos) -> Vec<BoardPos> {
    let y = board_y::to_idx(&board_pos.y);
    vec![
        BoardPos { x: board_pos.x, y: board_y::from_idx(y - 1).unwrap() },
        BoardPos { x: board_pos.x, y: board_y::from_idx(y - 2).unwrap() },
    ]
}

#[cfg(test)]
mod test {
    use crate::board::board_pos;

    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(pawn_white_move(board_pos::from_str("D7").unwrap()), [board_pos::from_str("D6").unwrap(), board_pos::from_str("D5").unwrap()]);
        assert_eq!(pawn_white_move(board_pos::from_str("E7").unwrap()), [board_pos::from_str("E6").unwrap(), board_pos::from_str("E5").unwrap()]);
    }
}
