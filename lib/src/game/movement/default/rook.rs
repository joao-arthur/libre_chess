use crate::{
    board::pos::Pos,
    game::{board::GameBoard, movement::movement::GameMovementOld},
    geometry::poligon::rect::RectU8,
    movement::Movement,
};

pub fn movements(board: &GameBoard, bounds: &RectU8, pos: &Pos) -> Vec<GameMovementOld> {
    let mut result: Vec<GameMovementOld> = Vec::new();
    if let Some(piece) = board.get(pos) {
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
                    if let Some(curr_piece) = board.get(&curr_pos) {
                        if curr_piece.color == piece.color {
                            break;
                        } else {
                            result.push(GameMovementOld::from(Movement {
                                piece: *piece,
                                from: pos.clone(),
                                to: curr_pos,
                            }));
                            break;
                        }
                    } else {
                        result.push(GameMovementOld::from(Movement {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        }));
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
        board::pos::Pos,
        game::{board, mode::standard_chess, movement::movement::GameMovementOld, piece},
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
        let board = HashMap::from([piece::of_str("D4", "♜")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("D4")),
            [
                GameMovementOld::from(Movement::of_str("♜", "D4", "E4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "F4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "G4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "H4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D3")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D2")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D1")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "C4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "B4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "A4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D5")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D6")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D7")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D8")),
            ]
        );
    }

    #[test]
    fn movements_small_bounds() {
        let board = HashMap::from([piece::of_str("F6", "♜")]);
        let bounds = RectU8 { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("F6")),
            [
                GameMovementOld::from(Movement::of_str("♜", "F6", "G6")),
                GameMovementOld::from(Movement::of_str("♜", "F6", "H6")),
                GameMovementOld::from(Movement::of_str("♜", "F6", "F5")),
                GameMovementOld::from(Movement::of_str("♜", "F6", "F4")),
                GameMovementOld::from(Movement::of_str("♜", "F6", "E6")),
                GameMovementOld::from(Movement::of_str("♜", "F6", "D6")),
                GameMovementOld::from(Movement::of_str("♜", "F6", "F7")),
                GameMovementOld::from(Movement::of_str("♜", "F6", "F8"))
            ]
        );
    }

    #[test]
    fn movements_top_right_edge() {
        let board = HashMap::from([piece::of_str("H8", "♜")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("H8")),
            [
                GameMovementOld::from(Movement::of_str("♜", "H8", "H7")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "H6")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "H5")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "H4")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "H3")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "H2")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "H1")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "G8")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "F8")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "E8")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "D8")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "C8")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "B8")),
                GameMovementOld::from(Movement::of_str("♜", "H8", "A8")),
            ]
        );
    }

    #[test]
    fn movements_bottom_right_edge() {
        let board = HashMap::from([piece::of_str("H1", "♜")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("H1")),
            [
                GameMovementOld::from(Movement::of_str("♜", "H1", "G1")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "F1")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "E1")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "D1")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "C1")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "B1")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "A1")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "H2")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "H3")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "H4")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "H5")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "H6")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "H7")),
                GameMovementOld::from(Movement::of_str("♜", "H1", "H8"))
            ]
        );
    }

    #[test]
    fn movements_bottom_left_edge() {
        let board = HashMap::from([piece::of_str("A1", "♜")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("A1")),
            [
                GameMovementOld::from(Movement::of_str("♜", "A1", "B1")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "C1")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "D1")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "E1")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "F1")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "G1")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "H1")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "A2")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "A3")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "A4")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "A5")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "A6")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "A7")),
                GameMovementOld::from(Movement::of_str("♜", "A1", "A8")),
            ]
        );
    }

    #[test]
    fn movements_top_left_edge() {
        let board = HashMap::from([piece::of_str("A8", "♜")]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("A8")),
            [
                GameMovementOld::from(Movement::of_str("♜", "A8", "B8")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "C8")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "D8")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "E8")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "F8")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "G8")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "H8")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "A7")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "A6")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "A5")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "A4")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "A3")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "A2")),
                GameMovementOld::from(Movement::of_str("♜", "A8", "A1")),
            ]
        );
    }

    #[test]
    fn movements_white_capture() {
        let board = board::of_str([
            "        ",
            "   ♝    ",
            "        ",
            "        ",
            "   ♖  ♗ ",
            "        ",
            "        ",
            "   ♝    ",
        ]);
        let bounds = standard_chess().bounds;
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("D4")),
            [
                GameMovementOld::from(Movement::of_str("♖", "D4", "E4")),
                GameMovementOld::from(Movement::of_str("♖", "D4", "F4")),
                GameMovementOld::from(Movement::of_str("♖", "D4", "D3")),
                GameMovementOld::from(Movement::of_str("♖", "D4", "D2")),
                GameMovementOld::from(Movement::of_str("♖", "D4", "D1")),
                GameMovementOld::from(Movement::of_str("♖", "D4", "C4")),
                GameMovementOld::from(Movement::of_str("♖", "D4", "B4")),
                GameMovementOld::from(Movement::of_str("♖", "D4", "A4")),
                GameMovementOld::from(Movement::of_str("♖", "D4", "D5")),
                GameMovementOld::from(Movement::of_str("♖", "D4", "D6")),
                GameMovementOld::from(Movement::of_str("♖", "D4", "D7")),
            ]
        );
    }

    #[test]
    fn movements_black_capture() {
        let board = board::of_str([
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
            movements(&board, &bounds, &Pos::of_str("D4")),
            [
                GameMovementOld::from(Movement::of_str("♜", "D4", "E4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "F4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D3")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D2")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D1")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "C4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "B4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "A4")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D5")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D6")),
                GameMovementOld::from(Movement::of_str("♜", "D4", "D7"))
            ]
        );
    }
}
