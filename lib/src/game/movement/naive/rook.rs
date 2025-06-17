use crate::{board::pos::Pos, game::board::Board, geometry::poligon::rect::RectU8};

pub fn movements(board: &Board, bounds: &RectU8, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let modifiers: [[i8; 2]; 4] = [[0, 1], [-1, 0], [0, -1], [1, 0]];
        for modifier in modifiers {
            let mut rel_row: i8 = 0;
            let mut rel_col: i8 = 0;
            loop {
                rel_row += modifier[0];
                rel_col += modifier[1];
                if let Some(curr_pos) = pos.try_of_rel_idx(rel_row, rel_col) {
                    if curr_pos.col > 7 || curr_pos.row > 7 {
                        break;
                    }
                    if let Some(curr_piece) = board.get(&curr_pos) {
                        if curr_piece.color == piece.color {
                            break;
                        } else {
                            result.push(curr_pos);
                            break;
                        }
                    } else {
                        result.push(curr_pos);
                    }
                } else {
                    break;
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
        let board = HashMap::from([piece::of_str("D4", "♜")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice([
                "E4", "F4", "G4", "H4", "D3", "D2", "D1", "C4", "B4", "A4", "D5", "D6", "D7", "D8",
            ])
        );
    }

    #[test]
    fn movements_edge() {
        let top_right = HashMap::from([piece::of_str("H8", "♜")]);
        let bottom_right = HashMap::from([piece::of_str("H1", "♜")]);
        let bottom_left = HashMap::from([piece::of_str("A1", "♜")]);
        let top_left = HashMap::from([piece::of_str("A8", "♜")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&top_right, &bounds, &Pos::of_str("H8")),
            pos_of_str_slice([
                "H7", "H6", "H5", "H4", "H3", "H2", "H1", "G8", "F8", "E8", "D8", "C8", "B8", "A8",
            ])
        );
        assert_eq!(
            movements(&bottom_right, &bounds, &Pos::of_str("H1")),
            pos_of_str_slice([
                "G1", "F1", "E1", "D1", "C1", "B1", "A1", "H2", "H3", "H4", "H5", "H6", "H7", "H8"
            ])
        );
        assert_eq!(
            movements(&bottom_left, &bounds, &Pos::of_str("A1")),
            pos_of_str_slice([
                "B1", "C1", "D1", "E1", "F1", "G1", "H1", "A2", "A3", "A4", "A5", "A6", "A7", "A8",
            ])
        );
        assert_eq!(
            movements(&top_left, &bounds, &Pos::of_str("A8")),
            pos_of_str_slice([
                "B8", "C8", "D8", "E8", "F8", "G8", "H8", "A7", "A6", "A5", "A4", "A3", "A2", "A1",
            ])
        );
    }

    #[test]
    fn movements_with_capture() {
        let board_white_rook = &board::of_str([
            "        ",
            "   ♝    ",
            "        ",
            "        ",
            "   ♖  ♗ ",
            "        ",
            "        ",
            "   ♝    ",
        ]);
        let board_black_rook = &board::of_str([
            "        ",
            "   ♗    ",
            "        ",
            "        ",
            "   ♜  ♝ ",
            "        ",
            "        ",
            "   ♗    ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board_white_rook, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E4", "F4", "D3", "D2", "D1", "C4", "B4", "A4", "D5", "D6", "D7"])
        );
        assert_eq!(
            movements(&board_black_rook, &bounds, &Pos::of_str("D4")),
            pos_of_str_slice(["E4", "F4", "D3", "D2", "D1", "C4", "B4", "A4", "D5", "D6", "D7"])
        );
    }
}
