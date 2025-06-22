use crate::{board::pos::Pos, game::board::GameBoard, geometry::poligon::rect::RectU8};

pub fn menace(board: &GameBoard, bounds: &RectU8, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
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
            if curr_pos.col < bounds.x1
                || curr_pos.col > bounds.x2
                || curr_pos.row < bounds.y1
                || curr_pos.row > bounds.y2
            {
                continue;
            }
            result.push(curr_pos);
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
        geometry::poligon::rect::RectU8,
    };

    use super::menace;

    #[test]
    fn menace_lonely_piece() {
        let board = HashMap::from([piece::of_str("D4", "♚")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace(&board, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "E3", "D3", "C3", "C4", "C5", "D5"])
        );
    }

    #[test]
    fn menace_edge() {
        let top_right = HashMap::from([piece::of_str("H8", "♚")]);
        let bottom_right = HashMap::from([piece::of_str("H1", "♚")]);
        let bottom_left = HashMap::from([piece::of_str("A1", "♚")]);
        let top_left = HashMap::from([piece::of_str("A8", "♚")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace(&top_right, &bounds, &Pos::of_str("H8")),
            pos_of_str_slice(["H7", "G7", "G8"])
        );
        assert_eq!(
            menace(&bottom_right, &bounds, &Pos::of_str("H1")),
            pos_of_str_slice(["G1", "G2", "H2"])
        );
        assert_eq!(
            menace(&bottom_left, &bounds, &Pos::of_str("A1")),
            pos_of_str_slice(["B2", "B1", "A2"])
        );
        assert_eq!(
            menace(&top_left, &bounds, &Pos::of_str("A8")),
            pos_of_str_slice(["B8", "B7", "A7"])
        );
    }

    #[test]
    fn menace_small_bounds() {
        let top_right = HashMap::from([piece::of_str("H8", "♚")]);
        let bottom_right = HashMap::from([piece::of_str("H4", "♚")]);
        let bottom_left = HashMap::from([piece::of_str("D4 ", "♚")]);
        let top_left = HashMap::from([piece::of_str("D8", "♚")]);
        let bounds = RectU8 { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            menace(&top_right, &bounds, &Pos::of_str("H8")),
            pos_of_str_slice(["H7", "G7", "G8"])
        );
        assert_eq!(
            menace(&bottom_right, &bounds, &Pos::of_str("H4")),
            pos_of_str_slice(["G4", "G5", "H5"])
        );
        assert_eq!(
            menace(&bottom_left, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "D5"])
        );
        assert_eq!(
            menace(&top_left, &bounds, &Pos::of_str("D8")),
            pos_of_str_slice(["E8", "E7", "D7"])
        );
    }

    #[test]
    fn menace_with_capture() {
        let board_white_king = board::of_str([
            "        ",
            "        ",
            "        ",
            "    ♜   ",
            "  ♝♔    ",
            "   ♖    ",
            "        ",
            "        ",
        ]);
        let board_black_king = board::of_str([
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
            menace(&board_white_king, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "E3", "D3", "C3", "C4", "C5", "D5"])
        );
        assert_eq!(
            menace(&board_black_king, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E5", "E4", "E3", "D3", "C3", "C4", "C5", "D5"])
        );
    }
}
