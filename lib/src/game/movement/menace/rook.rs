use crate::{
    board::pos::Pos,
    game::{board::GameBoard, game::GameBounds},
};

pub fn menace(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    let modifiers: [[i8; 2]; 4] = [[0, 1], [-1, 0], [0, -1], [1, 0]];
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
        game::{board::board_of_str, game::GameBounds, mode::standard_chess, piece::piece_of_str},
    };

    use super::menace;

    #[test]
    fn menace_lonely_piece() {
        let board = HashMap::from([piece_of_str("D4", '♜')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace(&board, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice([
                "E4", "F4", "G4", "H4", "D3", "D2", "D1", "C4", "B4", "A4", "D5", "D6", "D7", "D8",
            ])
        );
    }

    #[test]
    fn menace_edge() {
        let top_right = HashMap::from([piece_of_str("H8", '♜')]);
        let bottom_right = HashMap::from([piece_of_str("H1", '♜')]);
        let bottom_left = HashMap::from([piece_of_str("A1", '♜')]);
        let top_left = HashMap::from([piece_of_str("A8", '♜')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace(&top_right, &bounds, &Pos::of_str("H8")),
            pos_of_str_slice([
                "H7", "H6", "H5", "H4", "H3", "H2", "H1", "G8", "F8", "E8", "D8", "C8", "B8", "A8",
            ])
        );
        assert_eq!(
            menace(&bottom_right, &bounds, &Pos::of_str("H1")),
            pos_of_str_slice([
                "G1", "F1", "E1", "D1", "C1", "B1", "A1", "H2", "H3", "H4", "H5", "H6", "H7", "H8"
            ])
        );
        assert_eq!(
            menace(&bottom_left, &bounds, &Pos::of_str("A1")),
            pos_of_str_slice([
                "B1", "C1", "D1", "E1", "F1", "G1", "H1", "A2", "A3", "A4", "A5", "A6", "A7", "A8",
            ])
        );
        assert_eq!(
            menace(&top_left, &bounds, &Pos::of_str("A8")),
            pos_of_str_slice([
                "B8", "C8", "D8", "E8", "F8", "G8", "H8", "A7", "A6", "A5", "A4", "A3", "A2", "A1",
            ])
        );
    }

    #[test]
    fn menace_small_bounds() {
        let board = HashMap::from([piece_of_str("F6", '♜')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            menace(&board, &bounds, &Pos::of_str("F6")),
            pos_of_str_slice(["G6", "H6", "F5", "F4", "E6", "D6", "F7", "F8"])
        );
    }

    #[test]
    fn menace_with_capture() {
        let mode = standard_chess();
        let board_white_rook = board_of_str(
            &mode,
            [
                "        ",
                "   ♝    ",
                "        ",
                "        ",
                "   ♖  ♗ ",
                "        ",
                "        ",
                "   ♝    ",
            ],
        );
        let board_black_rook = board_of_str(
            &mode,
            [
                "        ",
                "   ♗    ",
                "        ",
                "        ",
                "   ♜  ♝ ",
                "        ",
                "        ",
                "   ♗    ",
            ],
        );
        assert_eq!(
            menace(&board_white_rook, &mode.bounds, &Pos::of_str("D4")),
            pos_of_str_slice([
                "E4", "F4", "G4", "D3", "D2", "D1", "C4", "B4", "A4", "D5", "D6", "D7"
            ])
        );
        assert_eq!(
            menace(&board_black_rook, &mode.bounds, &Pos::of_str("D4")),
            pos_of_str_slice([
                "E4", "F4", "G4", "D3", "D2", "D1", "C4", "B4", "A4", "D5", "D6", "D7"
            ])
        );
    }
}
