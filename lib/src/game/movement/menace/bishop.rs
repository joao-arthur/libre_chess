use crate::{board::pos::Pos, game::board::GameBoard, geometry::poligon::rect::RectU8};

pub fn menace(board: &GameBoard, bounds: &RectU8, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let modifiers: [[i8; 2]; 4] = [[1, 1], [-1, 1], [-1, -1], [1, -1]];
    for modifier in modifiers {
        let mut rel_row: i8 = 0;
        let mut rel_col: i8 = 0;
        loop {
            rel_row += modifier[0];
            rel_col += modifier[1];
            if let Some(curr_pos) = pos.try_of_rel_idx(rel_row, rel_col) {
                if curr_pos.col < bounds.x1
                    || curr_pos.col > bounds.x2
                    || curr_pos.row < bounds.y1
                    || curr_pos.row > bounds.y2
                {
                    break;
                }
                let piece = board.get(&curr_pos);
                result.push(curr_pos);
                if piece.is_some() {
                    break;
                }
            } else {
                break;
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
        geometry::poligon::rect::RectU8,
    };

    use super::menace;

    #[test]
    fn menace_lonely_piece() {
        let board = HashMap::from([piece::of_str("C5", "♝")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace(&board, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice(["D6", "E7", "F8", "D4", "E3", "F2", "G1", "B4", "A3", "B6", "A7"])
        );
    }

    #[test]
    fn menace_edge() {
        let top_right = HashMap::from([piece::of_str("H8", "♝")]);
        let bottom_right = HashMap::from([piece::of_str("H1", "♝")]);
        let bottom_left = HashMap::from([piece::of_str("A1", "♝")]);
        let top_left = HashMap::from([piece::of_str("A8", "♝")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace(&top_right, &bounds, &Pos::of_str("H8")),
            pos_of_str_slice(["G7", "F6", "E5", "D4", "C3", "B2", "A1"])
        );
        assert_eq!(
            menace(&bottom_right, &bounds, &Pos::of_str("H1")),
            pos_of_str_slice(["G2", "F3", "E4", "D5", "C6", "B7", "A8"])
        );
        assert_eq!(
            menace(&bottom_left, &bounds, &Pos::of_str("A1")),
            pos_of_str_slice(["B2", "C3", "D4", "E5", "F6", "G7", "H8"])
        );
        assert_eq!(
            menace(&top_left, &bounds, &Pos::of_str("A8")),
            pos_of_str_slice(["B7", "C6", "D5", "E4", "F3", "G2", "H1"])
        );
    }

    #[test]
    fn menace_small_bounds() {
        let board = HashMap::from([piece::of_str("F6", "♝")]);
        let bounds = RectU8 { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            menace(&board, &bounds, &Pos::of_str("F6")),
            pos_of_str_slice(["G7", "H8", "G5", "H4", "E5", "D4", "E7", "D8"])
        );
    }

    #[test]
    fn menace_with_capture() {
        let board_white_bishop = &board::of_str([
            "        ",
            "        ",
            "   ♜    ",
            "  ♗     ",
            "        ",
            "♜   ♖   ",
            "        ",
            "        ",
        ]);
        let board_black_bishop = &board::of_str([
            "        ",
            "        ",
            "   ♖    ",
            "  ♝     ",
            "        ",
            "♖   ♜   ",
            "        ",
            "        ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace(&board_white_bishop, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice(["D6", "D4", "E3", "B4", "A3", "B6", "A7"])
        );
        assert_eq!(
            menace(&board_black_bishop, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice(["D6", "D4", "E3", "B4", "A3", "B6", "A7"])
        );
    }
}
