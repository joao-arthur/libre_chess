use crate::board::{board_pos::BoardPos, board_y};

fn white_pawn_movements(board_pos: BoardPos) -> Vec<BoardPos> {
    let y = board_y::to_idx(&board_pos.y);
    vec![
        BoardPos { x: board_pos.x, y: board_y::of_idx(y - 1).unwrap() },
        BoardPos { x: board_pos.x, y: board_y::of_idx(y - 2).unwrap() },
    ]
}

#[cfg(test)]
mod test {
    use crate::board::board_pos;

    use super::*;

    #[test]
    fn test_pawn_white_move() {
        assert_eq!(
            white_pawn_movements(board_pos::of_str("D7").unwrap()),
            [board_pos::of_str("D6").unwrap(), board_pos::of_str("D5").unwrap()]
        );
        assert_eq!(
            white_pawn_movements(board_pos::of_str("E7").unwrap()),
            [board_pos::of_str("E6").unwrap(), board_pos::of_str("E5").unwrap()]
        );
    }
}
