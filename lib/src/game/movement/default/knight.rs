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
                } else {
                    result.push(GameMovement::from(DefaultMovement::from(Movement {
                        piece: *piece,
                        from: pos.clone(),
                        to: curr_pos,
                    })));
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
        let board = HashMap::from([piece_of_str("D4", '♞')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "E6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "F5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "F3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "E2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "C2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "B3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "B5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "C6"))),
            ]
        );
    }

    #[test]
    fn movements_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H8", '♞')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "H8", "G6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "H8", "F7")))
            ]
        );
    }

    #[test]
    fn movements_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H1", '♞')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "H1", "F2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "H1", "G3")))
            ]
        );
    }

    #[test]
    fn movements_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A1", '♞')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "A1", "B3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "A1", "C2")))
            ]
        );
    }

    #[test]
    fn movements_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A8", '♞')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "A8", "C7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "A8", "B6")))
            ]
        );
    }

    #[test]
    fn movements_small_bounds_top_right_edge() {
        let board = HashMap::from([piece_of_str("G7", '♞')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("G7")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "G7", "H5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "G7", "F5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "G7", "E6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "G7", "E8")))
            ]
        );
    }

    #[test]
    fn movements_small_bounds_bottom_right_edge() {
        let board = HashMap::from([piece_of_str("G5", '♞')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("G5")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "G5", "H7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "G5", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "G5", "E6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "G5", "F7")))
            ]
        );
    }

    #[test]
    fn movements_small_bounds_bottom_left_edge() {
        let board = HashMap::from([piece_of_str("E5", '♞')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("E5")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "E5", "F7"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "E5", "G6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "E5", "G4"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "E5", "D7")))
            ]
        );
    }

    #[test]
    fn movements_small_bounds_top_left_edge() {
        let board = HashMap::from([piece_of_str("E7", '♞')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("E7")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "E7", "G8"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "E7", "G6"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "E7", "F5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "E7", "D5"))),
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
                "    ♜   ",
                "     ♜  ",
                "   ♘    ",
                " ♖      ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovement::from(CaptureMovement::from(Movement::of('♘', "D4", "E6"))),
                GameMovement::from(CaptureMovement::from(Movement::of('♘', "D4", "F5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♘', "D4", "F3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♘', "D4", "E2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♘', "D4", "C2"))),
                GameMovement::from(MenaceMovement::from(Movement::of('♘', "D4", "B3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♘', "D4", "B5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♘', "D4", "C6"))),
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
                "    ♖   ",
                "     ♖  ",
                "   ♞    ",
                " ♜      ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovement::from(CaptureMovement::from(Movement::of('♞', "D4", "E6"))),
                GameMovement::from(CaptureMovement::from(Movement::of('♞', "D4", "F5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "F3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "E2"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "C2"))),
                GameMovement::from(MenaceMovement::from(Movement::of('♞', "D4", "B3"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "B5"))),
                GameMovement::from(DefaultMovement::from(Movement::of('♞', "D4", "C6")))
            ]
        );
    }
}
