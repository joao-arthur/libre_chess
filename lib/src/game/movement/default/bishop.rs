use crate::{
    game::{
        board::GameBoard,
        game::GameBounds,
        movement::movement::{CaptureMove, DefaultMove, GameMove, MenaceMove},
    },
    movement::Movement,
    pos::Pos,
};

pub fn moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMove> {
    let mut result: Vec<GameMove> = Vec::new();
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
                            result.push(GameMove::from(MenaceMove::from(Movement {
                                piece: *piece,
                                from: pos.clone(),
                                to: curr_pos,
                            })));
                        } else {
                            result.push(GameMove::from(CaptureMove::from(Movement {
                                piece: *piece,
                                from: pos.clone(),
                                to: curr_pos,
                            })));
                        }
                        break;
                    } else {
                        result.push(GameMove::from(DefaultMove::from(Movement {
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
        game::{
            board::{board_empty, board_of_str},
            game::GameBounds,
            mode::standard_chess,
            movement::movement::{CaptureMove, DefaultMove, GameMove, MenaceMove},
            piece::piece_of_str,
        },
        movement::Movement,
        pos::Pos,
    };

    use super::moves;

    #[test]
    fn movements_empty_board() {
        let mode = standard_chess();
        assert_eq!(moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn movements_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("C5", '♝')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "D6"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "E7"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "F8"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "D4"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "E3"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "F2"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "G1"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "B4"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "A3"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "B6"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "A7"))),
            ]
        );
    }

    #[test]
    fn movements_small_bounds() {
        let board = HashMap::from([piece_of_str("F6", '♝')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            moves(&board, &bounds, &Pos::of_str("F6")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♝', "F6", "G7"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "F6", "H8"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "F6", "G5"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "F6", "H4"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "F6", "E5"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "F6", "D4"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "F6", "E7"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "F6", "D8"))),
            ]
        );
    }

    #[test]
    fn movements_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H8", '♝')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♝', "H8", "G7"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H8", "F6"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H8", "E5"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H8", "D4"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H8", "C3"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H8", "B2"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H8", "A1"))),
            ]
        );
    }

    #[test]
    fn movements_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H1", '♝')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♝', "H1", "G2"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H1", "F3"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H1", "E4"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H1", "D5"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H1", "C6"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H1", "B7"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "H1", "A8"))),
            ]
        );
    }

    #[test]
    fn movements_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A1", '♝')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♝', "A1", "B2"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A1", "C3"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A1", "D4"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A1", "E5"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A1", "F6"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A1", "G7"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A1", "H8"))),
            ]
        );
    }

    #[test]
    fn movements_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A8", '♝')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMove::from(DefaultMove::from(Movement::of('♝', "A8", "B7"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A8", "C6"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A8", "D5"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A8", "E4"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A8", "F3"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A8", "G2"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "A8", "H1"))),
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
            moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::from(CaptureMove::from(Movement::of('♗', "C5", "D6"))),
                GameMove::from(DefaultMove::from(Movement::of('♗', "C5", "D4"))),
                GameMove::from(MenaceMove::from(Movement::of('♗', "C5", "E3"))),
                GameMove::from(DefaultMove::from(Movement::of('♗', "C5", "B4"))),
                GameMove::from(CaptureMove::from(Movement::of('♗', "C5", "A3"))),
                GameMove::from(DefaultMove::from(Movement::of('♗', "C5", "B6"))),
                GameMove::from(DefaultMove::from(Movement::of('♗', "C5", "A7"))),
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
            moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMove::from(CaptureMove::from(Movement::of('♝', "C5", "D6"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "D4"))),
                GameMove::from(MenaceMove::from(Movement::of('♝', "C5", "E3"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "B4"))),
                GameMove::from(CaptureMove::from(Movement::of('♝', "C5", "A3"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "B6"))),
                GameMove::from(DefaultMove::from(Movement::of('♝', "C5", "A7"))),
            ]
        );
    }
}
