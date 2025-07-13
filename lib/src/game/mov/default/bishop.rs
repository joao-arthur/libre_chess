use crate::{
    game::{
        board::GameBoard,
        game::GameBounds,
        mov::{GameMove, GameMoveType},
    },
    mov::Mov,
    pos::Pos,
};

pub fn bishop_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMove> {
    let mut result: Vec<GameMove> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let modifiers: [[i8; 2]; 4] = [[1, 1], [-1, 1], [-1, -1], [1, -1]];
        for modifier in modifiers {
            let mut rel_row: i8 = 0;
            let mut rel_col: i8 = 0;
            loop {
                rel_row += modifier[0];
                rel_col += modifier[1];
                if let Some(curr_pos) = pos.try_rel_idx(rel_row, rel_col) {
                    if curr_pos.col < bounds.x1
                        || curr_pos.col > bounds.x2
                        || curr_pos.row < bounds.y1
                        || curr_pos.row > bounds.y2
                    {
                        break;
                    }
                    if let Some(curr_piece) = board.get(&curr_pos) {
                        if curr_piece.color == piece.color {
                            result.push(GameMove {
                                mov: Mov { piece: *piece, from: pos.clone(), to: curr_pos },
                                typ: GameMoveType::Menace,
                            });
                        } else {
                            result.push(GameMove {
                                mov: Mov { piece: *piece, from: pos.clone(), to: curr_pos },
                                typ: GameMoveType::Capture,
                            });
                        }
                        break;
                    } else {
                        result.push(GameMove {
                            mov: Mov { piece: *piece, from: pos.clone(), to: curr_pos },
                            typ: GameMoveType::Default,
                        });
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
            mov::GameMove,
            piece::game_piece_of,
        },
        pos::Pos,
    };

    use super::bishop_moves;

    #[test]
    fn bishop_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(bishop_moves(&board_empty(), &mode.bounds, &Pos::of("A1")), []);
    }

    #[test]
    fn bishop_moves_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♝')]);
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of("C5")),
            [
                GameMove::default_of('♝', "C5", "D6"),
                GameMove::default_of('♝', "C5", "E7"),
                GameMove::default_of('♝', "C5", "F8"),
                GameMove::default_of('♝', "C5", "D4"),
                GameMove::default_of('♝', "C5", "E3"),
                GameMove::default_of('♝', "C5", "F2"),
                GameMove::default_of('♝', "C5", "G1"),
                GameMove::default_of('♝', "C5", "B4"),
                GameMove::default_of('♝', "C5", "A3"),
                GameMove::default_of('♝', "C5", "B6"),
                GameMove::default_of('♝', "C5", "A7"),
            ]
        );
    }

    #[test]
    fn bishop_moves_small_bounds() {
        let board = HashMap::from([game_piece_of("F6", '♝')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            bishop_moves(&board, &bounds, &Pos::of("F6")),
            [
                GameMove::default_of('♝', "F6", "G7"),
                GameMove::default_of('♝', "F6", "H8"),
                GameMove::default_of('♝', "F6", "G5"),
                GameMove::default_of('♝', "F6", "H4"),
                GameMove::default_of('♝', "F6", "E5"),
                GameMove::default_of('♝', "F6", "D4"),
                GameMove::default_of('♝', "F6", "E7"),
                GameMove::default_of('♝', "F6", "D8"),
            ]
        );
    }

    #[test]
    fn bishop_moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H8", '♝')]);
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of("H8")),
            [
                GameMove::default_of('♝', "H8", "G7"),
                GameMove::default_of('♝', "H8", "F6"),
                GameMove::default_of('♝', "H8", "E5"),
                GameMove::default_of('♝', "H8", "D4"),
                GameMove::default_of('♝', "H8", "C3"),
                GameMove::default_of('♝', "H8", "B2"),
                GameMove::default_of('♝', "H8", "A1"),
            ]
        );
    }

    #[test]
    fn bishop_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H1", '♝')]);
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of("H1")),
            [
                GameMove::default_of('♝', "H1", "G2"),
                GameMove::default_of('♝', "H1", "F3"),
                GameMove::default_of('♝', "H1", "E4"),
                GameMove::default_of('♝', "H1", "D5"),
                GameMove::default_of('♝', "H1", "C6"),
                GameMove::default_of('♝', "H1", "B7"),
                GameMove::default_of('♝', "H1", "A8"),
            ]
        );
    }

    #[test]
    fn bishop_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A1", '♝')]);
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of("A1")),
            [
                GameMove::default_of('♝', "A1", "B2"),
                GameMove::default_of('♝', "A1", "C3"),
                GameMove::default_of('♝', "A1", "D4"),
                GameMove::default_of('♝', "A1", "E5"),
                GameMove::default_of('♝', "A1", "F6"),
                GameMove::default_of('♝', "A1", "G7"),
                GameMove::default_of('♝', "A1", "H8"),
            ]
        );
    }

    #[test]
    fn bishop_moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A8", '♝')]);
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of("A8")),
            [
                GameMove::default_of('♝', "A8", "B7"),
                GameMove::default_of('♝', "A8", "C6"),
                GameMove::default_of('♝', "A8", "D5"),
                GameMove::default_of('♝', "A8", "E4"),
                GameMove::default_of('♝', "A8", "F3"),
                GameMove::default_of('♝', "A8", "G2"),
                GameMove::default_of('♝', "A8", "H1"),
            ]
        );
    }

    #[test]
    fn bishop_moves_white_capture() {
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
            bishop_moves(&board, &mode.bounds, &Pos::of("C5")),
            [
                GameMove::capture_of('♗', "C5", "D6"),
                GameMove::default_of('♗', "C5", "D4"),
                GameMove::menace_of('♗', "C5", "E3"),
                GameMove::default_of('♗', "C5", "B4"),
                GameMove::capture_of('♗', "C5", "A3"),
                GameMove::default_of('♗', "C5", "B6"),
                GameMove::default_of('♗', "C5", "A7"),
            ]
        );
    }

    #[test]
    fn bishop_moves_black_capture() {
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
            bishop_moves(&board, &mode.bounds, &Pos::of("C5")),
            [
                GameMove::capture_of('♝', "C5", "D6"),
                GameMove::default_of('♝', "C5", "D4"),
                GameMove::menace_of('♝', "C5", "E3"),
                GameMove::default_of('♝', "C5", "B4"),
                GameMove::capture_of('♝', "C5", "A3"),
                GameMove::default_of('♝', "C5", "B6"),
                GameMove::default_of('♝', "C5", "A7"),
            ]
        );
    }
}
