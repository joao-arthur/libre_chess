use crate::{
    game::{
        board::GameBoard,
        game::GameBounds,
        mov::{CaptureMovOld, DefaultMovOld, GameMovOld, MenaceMovOld},
    },
    mov::Mov,
    pos::Pos,
};

pub fn bishop_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMovOld> {
    let mut result: Vec<GameMovOld> = Vec::new();
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
                        break;
                    } else {
                        result.push(GameMovOld::from(DefaultMovOld::from(Mov {
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
            mov::{CaptureMovOld, DefaultMovOld, GameMovOld, MenaceMovOld},
            piece::game_piece_of,
        },
        mov::Mov,
        pos::Pos,
    };

    use super::bishop_moves;

    #[test]
    fn bishop_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(bishop_moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn bishop_moves_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("C5", '♝')]);
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "D6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "E7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "F8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "D4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "E3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "F2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "G1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "B4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "A3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "B6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "A7"))),
            ]
        );
    }

    #[test]
    fn bishop_moves_small_bounds() {
        let board = HashMap::from([game_piece_of("F6", '♝')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            bishop_moves(&board, &bounds, &Pos::of_str("F6")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "F6", "G7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "F6", "H8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "F6", "G5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "F6", "H4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "F6", "E5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "F6", "D4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "F6", "E7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "F6", "D8"))),
            ]
        );
    }

    #[test]
    fn bishop_moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H8", '♝')]);
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H8", "G7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H8", "F6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H8", "E5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H8", "D4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H8", "C3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H8", "B2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H8", "A1"))),
            ]
        );
    }

    #[test]
    fn bishop_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H1", '♝')]);
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H1", "G2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H1", "F3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H1", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H1", "D5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H1", "C6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H1", "B7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "H1", "A8"))),
            ]
        );
    }

    #[test]
    fn bishop_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A1", '♝')]);
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A1", "B2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A1", "C3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A1", "D4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A1", "E5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A1", "F6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A1", "G7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A1", "H8"))),
            ]
        );
    }

    #[test]
    fn bishop_moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A8", '♝')]);
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A8", "B7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A8", "C6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A8", "D5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A8", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A8", "F3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A8", "G2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "A8", "H1"))),
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
            bishop_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovOld::from(CaptureMovOld::from(Mov::of('♗', "C5", "D6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♗', "C5", "D4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♗', "C5", "E3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♗', "C5", "B4"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♗', "C5", "A3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♗', "C5", "B6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♗', "C5", "A7"))),
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
            bishop_moves(&board, &mode.bounds, &Pos::of_str("C5")),
            [
                GameMovOld::from(CaptureMovOld::from(Mov::of('♝', "C5", "D6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "D4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♝', "C5", "E3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "B4"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♝', "C5", "A3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "B6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♝', "C5", "A7"))),
            ]
        );
    }
}
