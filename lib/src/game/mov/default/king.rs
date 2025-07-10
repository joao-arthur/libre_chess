use crate::{
    game::{
        board::GameBoard,
        game::GameBounds,
        mov::{CaptureMovOld, DefaultMovOld, GameMovOld, MenaceMovOld},
    },
    mov::Mov,
    pos::Pos,
};

pub fn king_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMovOld> {
    let mut result: Vec<GameMovOld> = Vec::new();
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
                    if curr_piece.color == piece.color {
                        result.push(GameMovOld::from(MenaceMovOld::from(Mov {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        })));
                    } else {
                        result.push(GameMovOld::from(CaptureMovOld::from(Mov {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        })));
                    }
                } else {
                    result.push(GameMovOld::from(DefaultMovOld::from(Mov {
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
        game::{
            board::{board_empty, board_of_str},
            game::GameBounds,
            mode::standard_chess,
            mov::{CaptureMovOld, DefaultMovOld, GameMovOld, MenaceMovOld},
            piece::game_piece_of,
        },
        mov::Mov,
        pos::Pos,
    };

    use super::king_moves;

    #[test]
    fn king_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(king_moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn king_moves_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "E5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "E3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "D3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "C3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "C4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "C5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "D5"))),
            ]
        );
    }

    #[test]
    fn king_moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H8", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H8", "H7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H8", "G7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H8", "G8"))),
            ]
        );
    }

    #[test]
    fn king_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H1", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H1", "G1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H1", "G2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H1", "H2"))),
            ]
        );
    }

    #[test]
    fn king_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A1", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "A1", "B2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "A1", "B1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "A1", "A2"))),
            ]
        );
    }

    #[test]
    fn king_moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A8", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "A8", "B8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "A8", "B7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "A8", "A7"))),
            ]
        );
    }

    #[test]
    fn king_moves_small_bounds_top_right_edge() {
        let board = HashMap::from([game_piece_of("H8", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of_str("H8")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H8", "H7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H8", "G7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H8", "G8"))),
            ]
        );
    }

    #[test]
    fn king_moves_small_bounds_bottom_right_edge() {
        let board = HashMap::from([game_piece_of("H4", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of_str("H4")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H4", "G4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H4", "G5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "H4", "H5"))),
            ]
        );
    }

    #[test]
    fn king_moves_small_bounds_bottom_left_edge() {
        let board = HashMap::from([game_piece_of("D4 ", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of_str("D4")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "E5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "D5"))),
            ]
        );
    }

    #[test]
    fn king_moves_small_bounds_top_left_edge() {
        let board = HashMap::from([game_piece_of("D8", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of_str("D8")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D8", "E8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D8", "E7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D8", "D7"))),
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
            king_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovOld::from(CaptureMovOld::from(Mov::of('♔', "D4", "E5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♔', "D4", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♔', "D4", "E3"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♔', "D4", "D3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♔', "D4", "C3"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♔', "D4", "C4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♔', "D4", "C5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♔', "D4", "D5"))),
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
            king_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovOld::from(CaptureMovOld::from(Mov::of('♚', "D4", "E5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "E3"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♚', "D4", "D3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "C3"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♚', "D4", "C4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "C5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♚', "D4", "D5"))),
            ]
        );
    }
}
