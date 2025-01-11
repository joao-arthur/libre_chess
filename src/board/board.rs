use crate::piece::Piece;

use super::board_pos::BoardPos;

pub type Board = [[Option<Piece>; 8]; 8];

fn get_default_board() -> Board {
    [
        [
            Piece::from_str("♜"),
            Piece::from_str("♞"),
            Piece::from_str("♝"),
            Piece::from_str("♛"),
            Piece::from_str("♚"),
            Piece::from_str("♝"),
            Piece::from_str("♞"),
            Piece::from_str("♜"),
        ],
        [
            Piece::from_str("♟"),
            Piece::from_str("♟"),
            Piece::from_str("♟"),
            Piece::from_str("♟"),
            Piece::from_str("♟"),
            Piece::from_str("♟"),
            Piece::from_str("♟"),
            Piece::from_str("♟"),
        ],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [
            Piece::from_str("♙"),
            Piece::from_str("♙"),
            Piece::from_str("♙"),
            Piece::from_str("♙"),
            Piece::from_str("♙"),
            Piece::from_str("♙"),
            Piece::from_str("♙"),
            Piece::from_str("♙"),
        ],
        [
            Piece::from_str("♖"),
            Piece::from_str("♘"),
            Piece::from_str("♗"),
            Piece::from_str("♕"),
            Piece::from_str("♔"),
            Piece::from_str("♗"),
            Piece::from_str("♘"),
            Piece::from_str("♖"),
        ],
    ]
}

fn board_to_string(board: &Board) -> String {
    let mut res: String = String::from("");
    for row in board {
        for col in row {
            match col {
                Some(val) => res.push_str(val.to_str()),
                None => res.push_str(" "),
            }
        }
        res.push_str("\n")
    }
    res
}

fn get_board_piece(board: &Board, board_pos: &BoardPos) -> Option<Piece> {
    let x = board_pos.x.to_idx();
    let y = board_pos.y.to_idx();

    let x_idx: usize = (x).into();
    let y_idx: usize = (7 - y).into();

    board[y_idx][x_idx].clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board_to_string() {
        let brd = get_default_board();
        let res = board_to_string(&brd);
        assert_eq!(
            res.to_owned(),
            "".to_owned()
                + "♜♞♝♛♚♝♞♜\n"
                + "♟♟♟♟♟♟♟♟\n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "♙♙♙♙♙♙♙♙\n"
                + "♖♘♗♕♔♗♘♖\n"
        );
    }

    #[test]
    fn test_get_board_position() {
        let brd = get_default_board();
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("A8").unwrap()), Piece::from_str("♜"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("B8").unwrap()), Piece::from_str("♞"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("C8").unwrap()), Piece::from_str("♝"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("D8").unwrap()), Piece::from_str("♛"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("E8").unwrap()), Piece::from_str("♚"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("F8").unwrap()), Piece::from_str("♝"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("G8").unwrap()), Piece::from_str("♞"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("H8").unwrap()), Piece::from_str("♜"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("H8").unwrap()), Piece::from_str("♜"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("H7").unwrap()), Piece::from_str("♟"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("H6").unwrap()), Piece::from_str(" "));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("H5").unwrap()), Piece::from_str(" "));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("H4").unwrap()), Piece::from_str(" "));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("H3").unwrap()), Piece::from_str(" "));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("H2").unwrap()), Piece::from_str("♙"));
        assert_eq!(get_board_piece(&brd, &BoardPos::from_str("H1").unwrap()), Piece::from_str("♖"));
    }
}
