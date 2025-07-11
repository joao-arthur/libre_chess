use crate::{
    game::{
        board::GameBoard,
        game::GameBounds,
        mov::{CaptureMove, DefaultMove, GameMove, GameMoveType, MenaceMove},
    },
    pos::Pos,
};

pub fn knight_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMove> {
    let mut result: Vec<GameMove> = Vec::new();
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
                        result.push(GameMove {
                            from: pos.clone(),
                            to: curr_pos,
                            t: GameMoveType::Menace(MenaceMove),
                        });
                    } else {
                        result.push(GameMove {
                            from: pos.clone(),
                            to: curr_pos,
                            t: GameMoveType::Capture(CaptureMove),
                        });
                    }
                } else {
                    result.push(GameMove {
                        from: pos.clone(),
                        to: curr_pos,
                        t: GameMoveType::Default(DefaultMove),
                    });
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
            mov::GameMove,
            piece::game_piece_of,
        },
        pos::Pos,
    };

    use super::knight_moves;

    #[test]
    fn knight_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(knight_moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn knight_moves_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♞')]);
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMove::default_of("D4", "E6"),
                GameMove::default_of("D4", "F5"),
                GameMove::default_of("D4", "F3"),
                GameMove::default_of("D4", "E2"),
                GameMove::default_of("D4", "C2"),
                GameMove::default_of("D4", "B3"),
                GameMove::default_of("D4", "B5"),
                GameMove::default_of("D4", "C6"),
            ]
        );
    }

    #[test]
    fn knight_moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H8", '♞')]);
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of_str("H8")),
            [GameMove::default_of("H8", "G6"), GameMove::default_of("H8", "F7")]
        );
    }

    #[test]
    fn knight_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H1", '♞')]);
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of_str("H1")),
            [GameMove::default_of("H1", "F2"), GameMove::default_of("H1", "G3")]
        );
    }

    #[test]
    fn knight_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A1", '♞')]);
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of_str("A1")),
            [GameMove::default_of("A1", "B3"), GameMove::default_of("A1", "C2")]
        );
    }

    #[test]
    fn knight_moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A8", '♞')]);
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of_str("A8")),
            [GameMove::default_of("A8", "C7"), GameMove::default_of("A8", "B6")]
        );
    }

    #[test]
    fn knight_moves_small_bounds_top_right_edge() {
        let board = HashMap::from([game_piece_of("G7", '♞')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            knight_moves(&board, &bounds, &Pos::of_str("G7")),
            [
                GameMove::default_of("G7", "H5"),
                GameMove::default_of("G7", "F5"),
                GameMove::default_of("G7", "E6"),
                GameMove::default_of("G7", "E8")
            ]
        );
    }

    #[test]
    fn knight_moves_small_bounds_bottom_right_edge() {
        let board = HashMap::from([game_piece_of("G5", '♞')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            knight_moves(&board, &bounds, &Pos::of_str("G5")),
            [
                GameMove::default_of("G5", "H7"),
                GameMove::default_of("G5", "E4"),
                GameMove::default_of("G5", "E6"),
                GameMove::default_of("G5", "F7")
            ]
        );
    }

    #[test]
    fn knight_moves_small_bounds_bottom_left_edge() {
        let board = HashMap::from([game_piece_of("E5", '♞')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            knight_moves(&board, &bounds, &Pos::of_str("E5")),
            [
                GameMove::default_of("E5", "F7"),
                GameMove::default_of("E5", "G6"),
                GameMove::default_of("E5", "G4"),
                GameMove::default_of("E5", "D7")
            ]
        );
    }

    #[test]
    fn knight_moves_small_bounds_top_left_edge() {
        let board = HashMap::from([game_piece_of("E7", '♞')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            knight_moves(&board, &bounds, &Pos::of_str("E7")),
            [
                GameMove::default_of("E7", "G8"),
                GameMove::default_of("E7", "G6"),
                GameMove::default_of("E7", "F5"),
                GameMove::default_of("E7", "D5"),
            ]
        );
    }

    #[test]
    fn knight_moves_white_capture() {
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
            knight_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMove::capture_of("D4", "E6"),
                GameMove::capture_of("D4", "F5"),
                GameMove::default_of("D4", "F3"),
                GameMove::default_of("D4", "E2"),
                GameMove::default_of("D4", "C2"),
                GameMove::menace_of("D4", "B3"),
                GameMove::default_of("D4", "B5"),
                GameMove::default_of("D4", "C6"),
            ]
        );
    }

    #[test]
    fn knight_moves_black_capture() {
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
            knight_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMove::capture_of("D4", "E6"),
                GameMove::capture_of("D4", "F5"),
                GameMove::default_of("D4", "F3"),
                GameMove::default_of("D4", "E2"),
                GameMove::default_of("D4", "C2"),
                GameMove::menace_of("D4", "B3"),
                GameMove::default_of("D4", "B5"),
                GameMove::default_of("D4", "C6")
            ]
        );
    }
}
