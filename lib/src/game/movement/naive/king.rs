use crate::{board::pos::Pos, game::board::Board, geometry::poligon::rect::RectU8};

pub fn movements(board: &Board, bounds: &RectU8, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let base = [
            pos.try_of_rel_idx(1, 1),
            pos.try_of_rel_idx(0, 1),
            pos.try_of_rel_idx(-1, 1),
            pos.try_of_rel_idx(-1, 0),
            pos.try_of_rel_idx(-1, -1),
            pos.try_of_rel_idx(0, -1),
            pos.try_of_rel_idx(1, -1),
            pos.try_of_rel_idx(1, 0),
        ];
        for curr_pos in base {
            if let Some(curr_pos) = curr_pos {
                if curr_pos.col > 7 || curr_pos.row > 7 {
                    continue;
                }
                if let Some(curr_piece) = board.get(&curr_pos) {
                    if curr_piece.color != piece.color {
                        result.push(curr_pos);
                    }
                } else {
                    result.push(curr_pos);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::{Pos, pos_of_str_slice},
        game::{board, mode::standard_chess, piece},
    };

    use super::movements;

    #[test]
    fn movements_empty_board() {
        let bounds = standard_chess().bounds;
        assert_eq!(movements(&board::empty(), &bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_piece() {
        let board = HashMap::from([piece::of_str("D4", "♚")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "E3", "D3", "C3", "C4", "C5", "D5"])
        );
    }

    #[test]
    fn movements_edge() {
        let top_right = HashMap::from([piece::of_str("H8", "♚")]);
        let bottom_right = HashMap::from([piece::of_str("H1", "♚")]);
        let bottom_left = HashMap::from([piece::of_str("A1", "♚")]);
        let top_left = HashMap::from([piece::of_str("A8", "♚")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&top_right, &bounds, &Pos::of_str("H8")),
            pos_of_str_slice(["H7", "G7", "G8"])
        );
        assert_eq!(
            movements(&bottom_right, &bounds, &Pos::of_str("H1")),
            pos_of_str_slice(["G1", "G2", "H2"])
        );
        assert_eq!(
            movements(&bottom_left, &bounds, &Pos::of_str("A1")),
            pos_of_str_slice(["B2", "B1", "A2"])
        );
        assert_eq!(
            movements(&top_left, &bounds, &Pos::of_str("A8")),
            pos_of_str_slice(["B8", "B7", "A7"])
        );
    }

    #[test]
    fn movements_with_capture() {
        let board_white_king = &board::of_str([
            "        ",
            "        ",
            "        ",
            "    ♜   ",
            "  ♝♔    ",
            "   ♖    ",
            "        ",
            "        ",
        ]);
        let board_black_king = &board::of_str([
            "        ",
            "        ",
            "        ",
            "    ♖   ",
            "  ♗♚    ",
            "   ♜    ",
            "        ",
            "        ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board_white_king, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "E3", "C3", "C4", "C5", "D5"])
        );
        assert_eq!(
            movements(&board_black_king, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "E3", "C3", "C4", "C5", "D5"])
        );
    }
}
