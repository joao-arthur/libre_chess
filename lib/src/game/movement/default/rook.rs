use crate::{
    board::pos::Pos,
    game::{board::GameBoard, game::GameBounds, movement::movement::DefaultMovement},
    movement::Movement,
};

pub fn movements(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<DefaultMovement> {
    let mut result: Vec<DefaultMovement> = Vec::new();
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
                            result.push(DefaultMovement::from(Movement {
                                piece: *piece,
                                from: pos.clone(),
                                to: curr_pos,
                            }));
                            break;
                        }
                    } else {
                        result.push(DefaultMovement::from(Movement {
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
        game::{
            board::{board_empty, board_of_str},
            game::GameBounds,
            mode::standard_chess,
            movement::movement::DefaultMovement,
            piece::piece_of_str,
        },
        movement::Movement,
    };

    use super::movements;

    #[test]
    fn movements_empty_board() {
        let mode = standard_chess();
        assert_eq!(movements(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("D4", '♜')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                DefaultMovement::from(Movement::of_str('♜', "D4", "E4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "F4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "G4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "H4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D3")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D2")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D1")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "C4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "B4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "A4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D5")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D6")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D7")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D8")),
            ]
        );
    }

    #[test]
    fn movements_small_bounds() {
        let board = HashMap::from([piece_of_str("F6", '♜')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("F6")),
            [
                DefaultMovement::from(Movement::of_str('♜', "F6", "G6")),
                DefaultMovement::from(Movement::of_str('♜', "F6", "H6")),
                DefaultMovement::from(Movement::of_str('♜', "F6", "F5")),
                DefaultMovement::from(Movement::of_str('♜', "F6", "F4")),
                DefaultMovement::from(Movement::of_str('♜', "F6", "E6")),
                DefaultMovement::from(Movement::of_str('♜', "F6", "D6")),
                DefaultMovement::from(Movement::of_str('♜', "F6", "F7")),
                DefaultMovement::from(Movement::of_str('♜', "F6", "F8"))
            ]
        );
    }

    #[test]
    fn movements_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H8", '♜')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                DefaultMovement::from(Movement::of_str('♜', "H8", "H7")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "H6")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "H5")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "H4")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "H3")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "H2")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "H1")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "G8")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "F8")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "E8")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "D8")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "C8")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "B8")),
                DefaultMovement::from(Movement::of_str('♜', "H8", "A8")),
            ]
        );
    }

    #[test]
    fn movements_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H1", '♜')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                DefaultMovement::from(Movement::of_str('♜', "H1", "G1")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "F1")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "E1")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "D1")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "C1")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "B1")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "A1")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "H2")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "H3")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "H4")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "H5")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "H6")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "H7")),
                DefaultMovement::from(Movement::of_str('♜', "H1", "H8"))
            ]
        );
    }

    #[test]
    fn movements_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A1", '♜')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                DefaultMovement::from(Movement::of_str('♜', "A1", "B1")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "C1")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "D1")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "E1")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "F1")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "G1")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "H1")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "A2")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "A3")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "A4")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "A5")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "A6")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "A7")),
                DefaultMovement::from(Movement::of_str('♜', "A1", "A8")),
            ]
        );
    }

    #[test]
    fn movements_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A8", '♜')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                DefaultMovement::from(Movement::of_str('♜', "A8", "B8")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "C8")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "D8")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "E8")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "F8")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "G8")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "H8")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "A7")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "A6")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "A5")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "A4")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "A3")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "A2")),
                DefaultMovement::from(Movement::of_str('♜', "A8", "A1")),
            ]
        );
    }

    #[test]
    fn movements_white_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
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
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                DefaultMovement::from(Movement::of_str('♖', "D4", "E4")),
                DefaultMovement::from(Movement::of_str('♖', "D4", "F4")),
                DefaultMovement::from(Movement::of_str('♖', "D4", "D3")),
                DefaultMovement::from(Movement::of_str('♖', "D4", "D2")),
                DefaultMovement::from(Movement::of_str('♖', "D4", "D1")),
                DefaultMovement::from(Movement::of_str('♖', "D4", "C4")),
                DefaultMovement::from(Movement::of_str('♖', "D4", "B4")),
                DefaultMovement::from(Movement::of_str('♖', "D4", "A4")),
                DefaultMovement::from(Movement::of_str('♖', "D4", "D5")),
                DefaultMovement::from(Movement::of_str('♖', "D4", "D6")),
                DefaultMovement::from(Movement::of_str('♖', "D4", "D7")),
            ]
        );
    }

    #[test]
    fn movements_black_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
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
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                DefaultMovement::from(Movement::of_str('♜', "D4", "E4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "F4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D3")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D2")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D1")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "C4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "B4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "A4")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D5")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D6")),
                DefaultMovement::from(Movement::of_str('♜', "D4", "D7"))
            ]
        );
    }
}
