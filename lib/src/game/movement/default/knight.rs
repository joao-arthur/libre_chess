use crate::{
    board::pos::Pos,
    game::{board::GameBoard, movement::movement::DefaultMovement},
    geometry::poligon::rect::RectU8,
    movement::Movement,
};

pub fn movements(board: &GameBoard, bounds: &RectU8, pos: &Pos) -> Vec<DefaultMovement> {
    let mut result: Vec<DefaultMovement> = Vec::new();
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
                        result.push(DefaultMovement::from(Movement {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        }));
                    }
                } else {
                    result.push(DefaultMovement::from(Movement {
                        piece: *piece,
                        from: pos.clone(),
                        to: curr_pos,
                    }));
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
        board::pos::Pos,
        game::{board, mode::standard_chess, movement::movement::DefaultMovement, piece},
        geometry::poligon::rect::RectU8,
        movement::Movement,
    };

    use super::movements;

    #[test]
    fn movements_empty_board() {
        let bounds = standard_chess().bounds;
        assert_eq!(movements(&board::empty(), &bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_piece() {
        let board = HashMap::from([piece::of_str("D4", '♞')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("D4")),
            [
                DefaultMovement::from(Movement::of_str('♞', "D4", "E6")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "F5")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "F3")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "E2")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "C2")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "B3")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "B5")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "C6")),
            ]
        );
    }

    #[test]
    fn movements_top_right_edge() {
        let board = HashMap::from([piece::of_str("H8", '♞')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("H8")),
            [
                DefaultMovement::from(Movement::of_str('♞', "H8", "G6")),
                DefaultMovement::from(Movement::of_str('♞', "H8", "F7"))
            ]
        );
    }

    #[test]
    fn movements_bottom_right_edge() {
        let board = HashMap::from([piece::of_str("H1", '♞')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("H1")),
            [
                DefaultMovement::from(Movement::of_str('♞', "H1", "F2")),
                DefaultMovement::from(Movement::of_str('♞', "H1", "G3"))
            ]
        );
    }

    #[test]
    fn movements_bottom_left_edge() {
        let board = HashMap::from([piece::of_str("A1", '♞')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("A1")),
            [
                DefaultMovement::from(Movement::of_str('♞', "A1", "B3")),
                DefaultMovement::from(Movement::of_str('♞', "A1", "C2"))
            ]
        );
    }

    #[test]
    fn movements_top_left_edge() {
        let board = HashMap::from([piece::of_str("A8", '♞')]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("A8")),
            [
                DefaultMovement::from(Movement::of_str('♞', "A8", "C7")),
                DefaultMovement::from(Movement::of_str('♞', "A8", "B6"))
            ]
        );
    }

    #[test]
    fn movements_small_bounds_top_right_edge() {
        let board = HashMap::from([piece::of_str("G7", '♞')]);
        let bounds = RectU8 { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("G7")),
            [
                DefaultMovement::from(Movement::of_str('♞', "G7", "H5")),
                DefaultMovement::from(Movement::of_str('♞', "G7", "F5")),
                DefaultMovement::from(Movement::of_str('♞', "G7", "E6")),
                DefaultMovement::from(Movement::of_str('♞', "G7", "E8"))
            ]
        );
    }

    #[test]
    fn movements_small_bounds_bottom_right_edge() {
        let board = HashMap::from([piece::of_str("G5", '♞')]);
        let bounds = RectU8 { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("G5")),
            [
                DefaultMovement::from(Movement::of_str('♞', "G5", "H7")),
                DefaultMovement::from(Movement::of_str('♞', "G5", "E4")),
                DefaultMovement::from(Movement::of_str('♞', "G5", "E6")),
                DefaultMovement::from(Movement::of_str('♞', "G5", "F7"))
            ]
        );
    }

    #[test]
    fn movements_small_bounds_bottom_left_edge() {
        let board = HashMap::from([piece::of_str("E5", '♞')]);
        let bounds = RectU8 { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("E5")),
            [
                DefaultMovement::from(Movement::of_str('♞', "E5", "F7")),
                DefaultMovement::from(Movement::of_str('♞', "E5", "G6")),
                DefaultMovement::from(Movement::of_str('♞', "E5", "G4")),
                DefaultMovement::from(Movement::of_str('♞', "E5", "D7"))
            ]
        );
    }

    #[test]
    fn movements_small_bounds_top_left_edge() {
        let board = HashMap::from([piece::of_str("E7", '♞')]);
        let bounds = RectU8 { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("E7")),
            [
                DefaultMovement::from(Movement::of_str('♞', "E7", "G8")),
                DefaultMovement::from(Movement::of_str('♞', "E7", "G6")),
                DefaultMovement::from(Movement::of_str('♞', "E7", "F5")),
                DefaultMovement::from(Movement::of_str('♞', "E7", "D5")),
            ]
        );
    }

    #[test]
    fn movements_white_capture() {
        let board = board::of_str([
            "        ",
            "        ",
            "    ♜   ",
            "     ♜  ",
            "   ♘    ",
            " ♖      ",
            "        ",
            "        ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("D4")),
            [
                DefaultMovement::from(Movement::of_str('♘', "D4", "E6")),
                DefaultMovement::from(Movement::of_str('♘', "D4", "F5")),
                DefaultMovement::from(Movement::of_str('♘', "D4", "F3")),
                DefaultMovement::from(Movement::of_str('♘', "D4", "E2")),
                DefaultMovement::from(Movement::of_str('♘', "D4", "C2")),
                DefaultMovement::from(Movement::of_str('♘', "D4", "B5")),
                DefaultMovement::from(Movement::of_str('♘', "D4", "C6")),
            ]
        );
    }

    #[test]
    fn movements_black_capture() {
        let board = board::of_str([
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
            movements(&board, &bounds, &Pos::of_str("D4")),
            [
                DefaultMovement::from(Movement::of_str('♞', "D4", "E6")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "F5")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "F3")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "E2")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "C2")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "B5")),
                DefaultMovement::from(Movement::of_str('♞', "D4", "C6"))
            ]
        );
    }
}
