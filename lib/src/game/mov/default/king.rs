use crate::{
    game::{
        board::GameBoard,
        game::GameBounds,
        mov::{CaptureMove, DefaultMove, GameMove, GameMoveType, MenaceMove},
    },
    mov::Mov,
    pos::Pos,
};

pub fn king_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMove> {
    let mut result: Vec<GameMove> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let base = [
            pos.try_rel_idx(1, 1),
            pos.try_rel_idx(0, 1),
            pos.try_rel_idx(-1, 1),
            pos.try_rel_idx(-1, 0),
            pos.try_rel_idx(-1, -1),
            pos.try_rel_idx(0, -1),
            pos.try_rel_idx(1, -1),
            pos.try_rel_idx(1, 0),
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
                            mov: Mov { piece: *piece, from: pos.clone(), to: curr_pos },
                            typ: GameMoveType::Menace(MenaceMove),
                        });
                    } else {
                        result.push(GameMove {
                            mov: Mov { piece: *piece, from: pos.clone(), to: curr_pos },
                            typ: GameMoveType::Capture(CaptureMove),
                        });
                    }
                } else {
                    result.push(GameMove {
                        mov: Mov { piece: *piece, from: pos.clone(), to: curr_pos },
                        typ: GameMoveType::Default(DefaultMove),
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

    use super::king_moves;

    #[test]
    fn king_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(king_moves(&board_empty(), &mode.bounds, &Pos::of("A1")), []);
    }

    #[test]
    fn king_moves_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of("D4")),
            [
                GameMove::default_of('♚', "D4", "E5"),
                GameMove::default_of('♚', "D4", "E4"),
                GameMove::default_of('♚', "D4", "E3"),
                GameMove::default_of('♚', "D4", "D3"),
                GameMove::default_of('♚', "D4", "C3"),
                GameMove::default_of('♚', "D4", "C4"),
                GameMove::default_of('♚', "D4", "C5"),
                GameMove::default_of('♚', "D4", "D5"),
            ]
        );
    }

    #[test]
    fn king_moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H8", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of("H8")),
            [
                GameMove::default_of('♚', "H8", "H7"),
                GameMove::default_of('♚', "H8", "G7"),
                GameMove::default_of('♚', "H8", "G8"),
            ]
        );
    }

    #[test]
    fn king_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H1", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of("H1")),
            [
                GameMove::default_of('♚', "H1", "G1"),
                GameMove::default_of('♚', "H1", "G2"),
                GameMove::default_of('♚', "H1", "H2"),
            ]
        );
    }

    #[test]
    fn king_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A1", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of("A1")),
            [
                GameMove::default_of('♚', "A1", "B2"),
                GameMove::default_of('♚', "A1", "B1"),
                GameMove::default_of('♚', "A1", "A2"),
            ]
        );
    }

    #[test]
    fn king_moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A8", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of("A8")),
            [
                GameMove::default_of('♚', "A8", "B8"),
                GameMove::default_of('♚', "A8", "B7"),
                GameMove::default_of('♚', "A8", "A7"),
            ]
        );
    }

    #[test]
    fn king_moves_small_bounds_top_right_edge() {
        let board = HashMap::from([game_piece_of("H8", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of("H8")),
            [
                GameMove::default_of('♚', "H8", "H7"),
                GameMove::default_of('♚', "H8", "G7"),
                GameMove::default_of('♚', "H8", "G8"),
            ]
        );
    }

    #[test]
    fn king_moves_small_bounds_bottom_right_edge() {
        let board = HashMap::from([game_piece_of("H4", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of("H4")),
            [
                GameMove::default_of('♚', "H4", "G4"),
                GameMove::default_of('♚', "H4", "G5"),
                GameMove::default_of('♚', "H4", "H5"),
            ]
        );
    }

    #[test]
    fn king_moves_small_bounds_bottom_left_edge() {
        let board = HashMap::from([game_piece_of("D4 ", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of("D4")),
            [
                GameMove::default_of('♚', "D4", "E5"),
                GameMove::default_of('♚', "D4", "E4"),
                GameMove::default_of('♚', "D4", "D5"),
            ]
        );
    }

    #[test]
    fn king_moves_small_bounds_top_left_edge() {
        let board = HashMap::from([game_piece_of("D8", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of("D8")),
            [
                GameMove::default_of('♚', "D8", "E8"),
                GameMove::default_of('♚', "D8", "E7"),
                GameMove::default_of('♚', "D8", "D7"),
            ]
        );
    }

    #[test]
    fn king_moves_white_capture() {
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
            king_moves(&board, &mode.bounds, &Pos::of("D4")),
            [
                GameMove::capture_of('♔', "D4", "E5"),
                GameMove::default_of('♔', "D4", "E4"),
                GameMove::default_of('♔', "D4", "E3"),
                GameMove::menace_of('♔', "D4", "D3"),
                GameMove::default_of('♔', "D4", "C3"),
                GameMove::capture_of('♔', "D4", "C4"),
                GameMove::default_of('♔', "D4", "C5"),
                GameMove::default_of('♔', "D4", "D5"),
            ]
        );
    }

    #[test]
    fn king_moves_black_capture() {
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
            king_moves(&board, &mode.bounds, &Pos::of("D4")),
            [
                GameMove::capture_of('♚', "D4", "E5"),
                GameMove::default_of('♚', "D4", "E4"),
                GameMove::default_of('♚', "D4", "E3"),
                GameMove::menace_of('♚', "D4", "D3"),
                GameMove::default_of('♚', "D4", "C3"),
                GameMove::capture_of('♚', "D4", "C4"),
                GameMove::default_of('♚', "D4", "C5"),
                GameMove::default_of('♚', "D4", "D5"),
            ]
        );
    }
}
