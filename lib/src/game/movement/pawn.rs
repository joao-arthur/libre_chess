use crate::{board::pos::Pos, color::Color, game::board::Board, piece::Piece};

//use super::Movement;

pub fn naive_movements_pawn(board: &Board, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(&pos) {
        let base = match &piece.color {
            Color::White => {
                if pos.row == 1 {
                    vec![pos.try_of_rel_idx(1, 0), pos.try_of_rel_idx(2, 0)]
                } else {
                    vec![pos.try_of_rel_idx(1, 0)]
                }
            }
            Color::Black => {
                if pos.row == 6 {
                    vec![pos.try_of_rel_idx(-1, 0), pos.try_of_rel_idx(-2, 0)]
                } else {
                    vec![pos.try_of_rel_idx(-1, 0)]
                }
            }
        };
        let capture_base = match &piece.color {
            Color::White => [pos.try_of_rel_idx(1, -1), pos.try_of_rel_idx(1, 1)],
            Color::Black => [pos.try_of_rel_idx(-1, -1), pos.try_of_rel_idx(-1, 1)],
        };
        for curr_pos in base {
            if let Some(curr_pos) = curr_pos {
                if board.get(&curr_pos).is_none() {
                    result.push(curr_pos);
                }
            }
        }
        for curr_pos in capture_base {
            if let Some(curr_pos) = curr_pos {
                if let Some(curr_piece) = board.get(&curr_pos) {
                    if &curr_piece.color != &piece.color {
                        result.push(curr_pos);
                    }
                }
            }
        }
    }
    result
}

//fn white_pawn_en_passant(board: &Board, history: Vec<Movement>, pos: &Pos) -> Vec<Pos> {
//    let mut result: Vec<Pos> = Vec::new();
//    if pos.row == Row::_5 {
//        if let Some(mov) = history.last() {
//            if mov.piece == Piece::of_str("♟") {
//                if Some(mov.from.clone()) == pos.try_of_rel_idx(-2, -1)
//                    && Some(mov.to.clone()) == pos.try_of_rel_idx(0, -1)
//                {
//                    if let Some(capture_pos) = pos.try_of_rel_idx(-1, -1) {
//                        result.push(capture_pos);
//                    }
//                }
//                if Some(mov.from.clone()) == pos.try_of_rel_idx(-2, 1)
//                    && Some(mov.to.clone()) == pos.try_of_rel_idx(0, 1)
//                {
//                    if let Some(capture_pos) = pos.try_of_rel_idx(-1, 1) {
//                        result.push(capture_pos);
//                    }
//                }
//            }
//        }
//    }
//    result
//}

#[cfg(test)]
mod tests {
    use crate::{board::pos::Pos, game::board};

    use super::naive_movements_pawn;

    #[test]
    fn pawn_movements_empty_board() {
        assert_eq!(naive_movements_pawn(&board::empty(), &Pos::of_str("A1")), []);
    }

    #[test]
    fn pawn_movements_lonely_piece() {
        assert_eq!(
            naive_movements_pawn(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            [Pos::of_str("C6")]
        );
        assert_eq!(
            naive_movements_pawn(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♟     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            [Pos::of_str("C4")]
        );
    }

    #[test]
    fn pawn_movements_first_move() {
        assert_eq!(
            naive_movements_pawn(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "♙       ",
                    "        ",
                ]),
                &Pos::of_str("A2"),
            ),
            [Pos::of_str("A3"), Pos::of_str("A4")]
        );
        assert_eq!(
            naive_movements_pawn(
                &board::of_str([
                    "        ",
                    "       ♟",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("H7"),
            ),
            [Pos::of_str("H6"), Pos::of_str("H5")]
        );
    }

    #[test]
    fn pawn_movements_blocked() {
        assert_eq!(
            naive_movements_pawn(
                &board::of_str([
                    "        ",
                    "        ",
                    "  ♟     ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            []
        );
        assert_eq!(
            naive_movements_pawn(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♟     ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            []
        );
    }

    #[test]
    fn pawn_movements_capture() {
        assert_eq!(
            naive_movements_pawn(
                &board::of_str([
                    "        ",
                    "        ",
                    " ♟ ♟    ",
                    "  ♙     ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            [Pos::of_str("C6"), Pos::of_str("B6"), Pos::of_str("D6")]
        );
        assert_eq!(
            naive_movements_pawn(
                &board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "  ♟     ",
                    " ♙ ♙    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("C5"),
            ),
            [Pos::of_str("C4"), Pos::of_str("B4"), Pos::of_str("D4")]
        );
    }

    //    #[test]
    //    fn test_white_pawn_en_passant() {
    //        assert_eq!(
    //            white_pawn_en_passant(
    //                &board::of_str([
    //                    "        ",
    //                    "        ",
    //                    "        ",
    //                    " ♟♙     ",
    //                    "        ",
    //                    "        ",
    //                    "        ",
    //                    "        ",
    //                ]),
    //                Vec::from([Movement {
    //                    piece: Piece::of_str("♟"),
    //                    from: Pos::of_str("B7"),
    //                    to: Pos::of_str("B5"),
    //                }]),
    //                &Pos::of_str("C5"),
    //            ),
    //            [Pos::of_str("B6")]
    //        );
    //    }
}
