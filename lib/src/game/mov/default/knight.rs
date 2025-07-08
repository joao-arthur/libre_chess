use crate::{
    game::{
        board::GameBoard,
        game::GameBounds,
        mov::{CaptureMov, DefaultMov, GameMov, MenaceMov},
    },
    mov::Mov,
    pos::Pos,
};

pub fn knight_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMov> {
    let mut result: Vec<GameMov> = Vec::new();
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
            piece::game_piece_of,
        },
        mov::Mov,
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
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "E6"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "F5"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "F3"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "E2"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "C2"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "B3"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "B5"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "C6"))),
            ]
        );
    }

    #[test]
    fn knight_moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H8", '♞')]);
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♞', "H8", "G6"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "H8", "F7")))
            ]
        );
    }

    #[test]
    fn knight_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H1", '♞')]);
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♞', "H1", "F2"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "H1", "G3")))
            ]
        );
    }

    #[test]
    fn knight_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A1", '♞')]);
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♞', "A1", "B3"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "A1", "C2")))
            ]
        );
    }

    #[test]
    fn knight_moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A8", '♞')]);
        assert_eq!(
            knight_moves(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♞', "A8", "C7"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "A8", "B6")))
            ]
        );
    }

    #[test]
    fn knight_moves_small_bounds_top_right_edge() {
        let board = HashMap::from([game_piece_of("G7", '♞')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            knight_moves(&board, &bounds, &Pos::of_str("G7")),
            [
                GameMov::from(DefaultMov::from(Mov::of('♞', "G7", "H5"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "G7", "F5"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "G7", "E6"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "G7", "E8")))
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
                GameMov::from(DefaultMov::from(Mov::of('♞', "G5", "H7"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "G5", "E4"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "G5", "E6"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "G5", "F7")))
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
                GameMov::from(DefaultMov::from(Mov::of('♞', "E5", "F7"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "E5", "G6"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "E5", "G4"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "E5", "D7")))
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
                GameMov::from(DefaultMov::from(Mov::of('♞', "E7", "G8"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "E7", "G6"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "E7", "F5"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "E7", "D5"))),
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
                GameMov::from(CaptureMov::from(Mov::of('♘', "D4", "E6"))),
                GameMov::from(CaptureMov::from(Mov::of('♘', "D4", "F5"))),
                GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "F3"))),
                GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "E2"))),
                GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "C2"))),
                GameMov::from(MenaceMov::from(Mov::of('♘', "D4", "B3"))),
                GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "B5"))),
                GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "C6"))),
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
                GameMov::from(CaptureMov::from(Mov::of('♞', "D4", "E6"))),
                GameMov::from(CaptureMov::from(Mov::of('♞', "D4", "F5"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "F3"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "E2"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "C2"))),
                GameMov::from(MenaceMov::from(Mov::of('♞', "D4", "B3"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "B5"))),
                GameMov::from(DefaultMov::from(Mov::of('♞', "D4", "C6")))
            ]
        );
    }
}
