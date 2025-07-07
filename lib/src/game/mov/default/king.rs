use crate::{
    game::{
        board::GameBoard,
        game::GameBounds,
        mov::{CaptureMov, DefaultMov, GameMov, MenaceMov},
    },
    mov::Mov,
    pos::Pos,
};

pub fn moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMov> {
    let mut result: Vec<GameMov> = Vec::new();
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
                        result.push(GameMov::from(MenaceMov::from(Mov {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        })));
                    } else {
                        result.push(GameMov::from(CaptureMov::from(Mov {
                            piece: *piece,
                            from: pos.clone(),
                            to: curr_pos,
                        })));
                    }
                } else {
                    result.push(GameMov::from(DefaultMov::from(Mov {
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
            mov::{CaptureMov, DefaultMov, GameMov, MenaceMov},
            piece::piece_of_str,
        },
        mov::Mov,
        pos::Pos,
    };

    use super::moves;

    #[test]
    fn moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn moves_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("D4", '♚')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "E5"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "E4"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "E3"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "D3"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "C3"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "C4"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "C5"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "D5"))),
            ]
        );
    }

    #[test]
    fn moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H8", '♚')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♚', "H8", "H7"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "H8", "G7"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "H8", "G8"))),
            ]
        );
    }

    #[test]
    fn moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("H1", '♚')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♚', "H1", "G1"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "H1", "G2"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "H1", "H2"))),
            ]
        );
    }

    #[test]
    fn moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A1", '♚')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♚', "A1", "B2"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "A1", "B1"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "A1", "A2"))),
            ]
        );
    }

    #[test]
    fn moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([piece_of_str("A8", '♚')]);
        assert_eq!(
            moves(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♚', "A8", "B8"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "A8", "B7"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "A8", "A7"))),
            ]
        );
    }

    #[test]
    fn moves_small_bounds_top_right_edge() {
        let board = HashMap::from([piece_of_str("H8", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            moves(&board, &bounds, &Pos::of_str("H8")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♚', "H8", "H7"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "H8", "G7"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "H8", "G8"))),
            ]
        );
    }

    #[test]
    fn moves_small_bounds_bottom_right_edge() {
        let board = HashMap::from([piece_of_str("H4", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            moves(&board, &bounds, &Pos::of_str("H4")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♚', "H4", "G4"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "H4", "G5"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "H4", "H5"))),
            ]
        );
    }

    #[test]
    fn moves_small_bounds_bottom_left_edge() {
        let board = HashMap::from([piece_of_str("D4 ", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            moves(&board, &bounds, &Pos::of_str("D4")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "E5"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "E4"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "D5"))),
            ]
        );
    }

    #[test]
    fn moves_small_bounds_top_left_edge() {
        let board = HashMap::from([piece_of_str("D8", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            moves(&board, &bounds, &Pos::of_str("D8")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♚', "D8", "E8"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D8", "E7"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D8", "D7"))),
            ]
        );
    }

    #[test]
    fn moves_white_capture() {
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
            moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMov::from(CaptureMov::from(Mov::of('♔', "D4", "E5"))),
                GameMov::from(DefaultMov::from(Mov::of('♔', "D4", "E4"))),
                GameMov::from(DefaultMov::from(Mov::of('♔', "D4", "E3"))),
                GameMov::from(MenaceMov::from(Mov::of('♔', "D4", "D3"))),
                GameMov::from(DefaultMov::from(Mov::of('♔', "D4", "C3"))),
                GameMov::from(CaptureMov::from(Mov::of('♔', "D4", "C4"))),
                GameMov::from(DefaultMov::from(Mov::of('♔', "D4", "C5"))),
                GameMov::from(DefaultMov::from(Mov::of('♔', "D4", "D5"))),
            ]
        );
    }

    #[test]
    fn moves_black_capture() {
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
            moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMov::from(CaptureMov::from(Mov::of('♚', "D4", "E5"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "E4"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "E3"))),
                GameMov::from(MenaceMov::from(Mov::of('♚', "D4", "D3"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "C3"))),
                GameMov::from(CaptureMov::from(Mov::of('♚', "D4", "C4"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "C5"))),
                GameMov::from(DefaultMov::from(Mov::of('♚', "D4", "D5"))),
            ]
        );
    }
}
