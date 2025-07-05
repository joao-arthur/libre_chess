use crate::{
    board::pos::Pos,
    game::{
        board::GameBoard,
        game::GameBounds,
        movement::movement::{DefaultMovement, GameMovement},
    },
    movement::Movement,
};

pub fn movements(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMovement> {
    let mut result: Vec<GameMovement> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let base = [
            pos.try_of_rel_idx(1, 1),
            pos.try_of_rel_idx(0, 1),
            pos.try_of_rel_idx(-1, 1),
            pos.try_of_rel_idx(-1, 0),
            pos.try_of_rel_idx(-1, -1),
            pos.try_of_rel_idx(0, -1),
            pos.try_of_rel_idx(1, -1),
            pos.try_of_rel_idx(1, 0),
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
                        result.push(GameMovement::from(DefaultMovement::from(Movement {
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
        board::pos::Pos,
        game::{
            board::{board_empty, board_of_str},
            game::GameBounds,
            mode::standard_chess,
            movement::movement::{DefaultMovement, GameMovement},
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
        let board = HashMap::from([piece_of_str("D4", '♚')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "D3"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "C3"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "C4"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "C5"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "D5"))),
            ]
        );
    }

    #[test]
    fn movements_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H8", '♚')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H8", "H7"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H8", "G7"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H8", "G8"))),
            ]
        );
    }

    #[test]
    fn movements_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H1", '♚')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H1", "G1"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H1", "G2"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H1", "H2"))),
            ]
        );
    }

    #[test]
    fn movements_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A1", '♚')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "A1", "B2"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "A1", "B1"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "A1", "A2"))),
            ]
        );
    }

    #[test]
    fn movements_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A8", '♚')]);
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "A8", "B8"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "A8", "B7"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "A8", "A7"))),
            ]
        );
    }

    #[test]
    fn movements_small_bounds_top_right_edge() {
        let board = HashMap::from([piece_of_str("H8", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("H8")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H8", "H7"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H8", "G7"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H8", "G8"))),
            ]
        );
    }

    #[test]
    fn movements_small_bounds_bottom_right_edge() {
        let board = HashMap::from([piece_of_str("H4", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("H4")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H4", "G4"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H4", "G5"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "H4", "H5"))),
            ]
        );
    }

    #[test]
    fn movements_small_bounds_bottom_left_edge() {
        let board = HashMap::from([piece_of_str("D4 ", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("D4")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "D5"))),
            ]
        );
    }

    #[test]
    fn movements_small_bounds_top_left_edge() {
        let board = HashMap::from([piece_of_str("D8", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            movements(&board, &bounds, &Pos::of_str("D8")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D8", "E8"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D8", "E7"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D8", "D7"))),
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
                "        ",
                "    ♜   ",
                "  ♝♔    ",
                "   ♖    ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "D4", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "D4", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "D4", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "D4", "C3"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "D4", "C4"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "D4", "C5"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "D4", "D5"))),
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
                "        ",
                "    ♖   ",
                "  ♗♚    ",
                "   ♜    ",
                "        ",
                "        ",
            ],
        );
        assert_eq!(
            movements(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "E4"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "E3"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "C3"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "C4"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "C5"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♚', "D4", "D5"))),
            ]
        );
    }
}
