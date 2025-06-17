use crate::{board::pos::Pos, game::board::Board, geometry::poligon::rect::RectU8};

pub fn movements(board: &Board, bounds: &RectU8, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let base = [
            pos.try_of_rel_idx(2, 1),
            pos.try_of_rel_idx(1, 2),
            pos.try_of_rel_idx(-1, 2),
            pos.try_of_rel_idx(-2, 1),
            pos.try_of_rel_idx(-2, -1),
            pos.try_of_rel_idx(-1, -2),
            pos.try_of_rel_idx(1, -2),
            pos.try_of_rel_idx(2, -1),
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
        geometry::poligon::rect::RectU8,
    };

    use super::movements;

    #[test]
    fn movements_empty_board() {
        let bounds = standard_chess().bounds;
        assert_eq!(movements(&board::empty(), &bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_piece() {
        let board = HashMap::from([piece::of_str("D4", "♞")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E6", "F5", "F3", "E2", "C2", "B3", "B5", "C6"])
        );
    }

    #[test]
    fn movements_edge() {
        let top_right = HashMap::from([piece::of_str("H8", "♞")]);
        let bottom_right = HashMap::from([piece::of_str("H1", "♞")]);
        let bottom_left = HashMap::from([piece::of_str("A1", "♞")]);
        let top_left = HashMap::from([piece::of_str("A8", "♞")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&top_right, &bounds, &Pos::of_str("H8")),
            pos_of_str_slice(["G6", "F7"])
        );
        assert_eq!(
            movements(&bottom_right, &bounds, &Pos::of_str("H1")),
            pos_of_str_slice(["F2", "G3"])
        );
        assert_eq!(
            movements(&bottom_left, &bounds, &Pos::of_str("A1")),
            pos_of_str_slice(["B3", "C2"])
        );
        assert_eq!(
            movements(&top_left, &bounds, &Pos::of_str("A8")),
            pos_of_str_slice(["C7", "B6"])
        );
    }

    #[test]
    fn movements_small_bounds() {
        let top_right = HashMap::from([piece::of_str("G7", "♞")]);
        let bottom_right = HashMap::from([piece::of_str("G5", "♞")]);
        let bottom_left = HashMap::from([piece::of_str("E5", "♞")]);
        let top_left = HashMap::from([piece::of_str("E7", "♞")]);
        let bounds = RectU8 { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&top_right, &bounds, &Pos::of_str("G7")),
            pos_of_str_slice(["H5", "F5", "E6", "E8"])
        );
        assert_eq!(
            movements(&bottom_right, &bounds, &Pos::of_str("G5")),
            pos_of_str_slice(["H7", "E4", "E6", "F7",])
        );
        assert_eq!(
            movements(&bottom_left, &bounds, &Pos::of_str("E5")),
            pos_of_str_slice(["F7", "G6", "G4", "D7"])
        );
        assert_eq!(
            movements(&top_left, &bounds, &Pos::of_str("E7")),
            pos_of_str_slice(["G8", "G6", "F5", "D5"])
        );
    }

    #[test]
    fn movements_with_capture() {
        let board_white_knight = &board::of_str([
            "        ",
            "        ",
            "    ♜   ",
            "     ♜  ",
            "   ♘    ",
            " ♖      ",
            "        ",
            "        ",
        ]);
        let board_black_knight = &board::of_str([
            "        ",
            "        ",
            "    ♖   ",
            "     ♖  ",
            "   ♞    ",
            " ♜      ",
            "        ",
            "        ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board_white_knight, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E6", "F5", "F3", "E2", "C2", "B5", "C6"])
        );
        assert_eq!(
            movements(&board_black_knight, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E6", "F5", "F3", "E2", "C2", "B5", "C6"])
        );
    }
}
