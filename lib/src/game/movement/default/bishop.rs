use crate::{
    pos::Pos,
    game::{
        board::GameBoard,
        game::GameBounds,
        movement::movement::{CaptureMovement, DefaultMovement, GameMovement, MenaceMovement},
    },
    movement::Movement,
};

pub fn movements(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMovement> {
    let mut result: Vec<GameMovement> = Vec::new();
    if let Some(piece) = board.get(pos) {
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
                    if let Some(curr_piece) = board.get(&curr_pos) {
                        if curr_piece.color == piece.color {
                            result.push(GameMovement::from(MenaceMovement::from(Movement {
                                piece: *piece,
                                from: pos.clone(),
                                to: curr_pos,
                            })));
                        } else {
                            result.push(GameMovement::from(CaptureMovement::from(Movement {
                                piece: *piece,
                                from: pos.clone(),
                                to: curr_pos,
                            })));
                        }
                        break;
                    } else {
                        result.push(GameMovement::from(DefaultMovement::from(Movement {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        })));
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
        pos::Pos,
        game::{
            board::{board_empty, board_of_str},
            game::GameBounds,
            mode::standard_chess,
            movement::movement::{CaptureMovement, DefaultMovement, GameMovement, MenaceMovement},
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
        let board = HashMap::from([piece_of_str("C5", '♝')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "D6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "E7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "F8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "F2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "G1"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "B4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "B6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "A7"))),
            ]
        );
    }

    #[test]
    fn movements_small_bounds() {
        let board = HashMap::from([piece_of_str("F6", '♝')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("F6")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "F6", "G7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "F6", "H8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "F6", "G5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "F6", "H4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "F6", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "F6", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "F6", "E7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "F6", "D8"))),
            ]
        );
    }

    #[test]
    fn movements_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H8", '♝')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H8", "G7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H8", "F6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H8", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H8", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H8", "C3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H8", "B2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H8", "A1"))),
            ]
        );
    }

    #[test]
    fn movements_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H1", '♝')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H1", "G2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H1", "F3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H1", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H1", "D5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H1", "C6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H1", "B7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "H1", "A8"))),
            ]
        );
    }

    #[test]
    fn movements_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A1", '♝')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A1", "B2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A1", "C3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A1", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A1", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A1", "F6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A1", "G7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A1", "H8"))),
            ]
        );
    }

    #[test]
    fn movements_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A8", '♝')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A8", "B7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A8", "C6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A8", "D5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A8", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A8", "F3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A8", "G2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "A8", "H1"))),
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
                "        ",
                "   ♜    ",
                "  ♗     ",
                "        ",
                "♜   ♖   ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(CaptureMovement::from(Movement::of('♗', "C5", "D6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♗', "C5", "D4"))),
                GameMovement::from(MenaceMovement::from(Movement::of('♗', "C5", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♗', "C5", "B4"))),
                GameMovement::from(CaptureMovement::from(Movement::of('♗', "C5", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♗', "C5", "B6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♗', "C5", "A7"))),
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
                "        ",
                "   ♖    ",
                "  ♝     ",
                "        ",
                "♖   ♜   ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovement::from(CaptureMovement::from(Movement::of('♝', "C5", "D6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "D4"))),
                GameMovement::from(MenaceMovement::from(Movement::of('♝', "C5", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "B4"))),
                GameMovement::from(CaptureMovement::from(Movement::of('♝', "C5", "A3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "B6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♝', "C5", "A7"))),
            ]
        );
    }
}
