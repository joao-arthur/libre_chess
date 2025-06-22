use crate::{
    board::pos::Pos, color::Color, game::board::GameBoard, geometry::poligon::rect::RectU8,
};

pub fn menace(board: &GameBoard, bounds: &RectU8, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let capture_base = match &piece.color {
            Color::White => [pos.try_of_rel_idx(1, -1), pos.try_of_rel_idx(1, 1)],
            Color::Black => [pos.try_of_rel_idx(-1, -1), pos.try_of_rel_idx(-1, 1)],
        };
        for curr_pos in capture_base {
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

    use super::menace;

    #[test]
    fn menace_lonely_piece() {
        let board_white_pawn = HashMap::from([piece::of_str("C5", "♙")]);
        let board_black_pawn = HashMap::from([piece::of_str("C5", "♟")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace(&board_white_pawn, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice(["B6", "D6"])
        );
        assert_eq!(
            menace(&board_black_pawn, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice(["B4", "D4"])
        );
    }

    #[test]
    fn menace_capture() {
        let board_white_pawn = board::of_str([
            "        ",
            "        ",
            " ♟ ♙    ",
            "  ♙     ",
            "        ",
            "        ",
            "        ",
            "        ",
        ]);
        let board_black_pawn = board::of_str([
            "        ",
            "        ",
            "        ",
            "  ♟     ",
            " ♟ ♙    ",
            "        ",
            "        ",
            "        ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            menace(&board_white_pawn, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice(["B6", "D6"])
        );
        assert_eq!(
            menace(&board_black_pawn, &bounds, &Pos::of_str("C5")),
            pos_of_str_slice(["B4", "D4"])
        );
    }
}
