use crate::{
    game::{
        board::GameBoard,
        game::GameBounds,
        mov::{CaptureMovOld, DefaultMovOld, GameMovOld, MenaceMovOld},
    },
    mov::Mov,
    pos::Pos,
};

pub fn rook_moves(board: &GameBoard, bounds: &GameBounds, pos: &Pos) -> Vec<GameMovOld> {
    let mut result: Vec<GameMovOld> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let modifiers: [[i8; 2]; 4] = [[0, 1], [-1, 0], [0, -1], [1, 0]];
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

    use super::rook_moves;

    #[test]
    fn rook_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(rook_moves(&board_empty(), &mode.bounds, &Pos::of_str("A1")), []);
    }

    #[test]
    fn rook_moves_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♜')]);
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "F4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "G4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "H4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "C4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "B4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "A4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D8"))),
            ]
        );
    }

    #[test]
    fn rook_moves_small_bounds() {
        let board = HashMap::from([game_piece_of("F6", '♜')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            rook_moves(&board, &bounds, &Pos::of_str("F6")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "F6", "G6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "F6", "H6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "F6", "F5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "F6", "F4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "F6", "E6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "F6", "D6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "F6", "F7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "F6", "F8")))
            ]
        );
    }

    #[test]
    fn rook_moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H8", '♜')]);
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of_str("H8")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "H7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "H6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "H5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "H4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "H3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "H2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "H1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "G8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "F8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "E8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "D8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "C8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "B8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H8", "A8"))),
            ]
        );
    }

    #[test]
    fn rook_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H1", '♜')]);
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of_str("H1")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "G1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "F1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "E1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "D1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "C1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "B1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "A1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "H2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "H3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "H4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "H5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "H6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "H7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "H1", "H8")))
            ]
        );
    }

    #[test]
    fn rook_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A1", '♜')]);
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of_str("A1")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "B1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "C1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "D1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "E1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "F1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "G1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "H1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "A2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "A3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "A4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "A5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "A6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "A7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A1", "A8"))),
            ]
        );
    }

    #[test]
    fn rook_moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A8", '♜')]);
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of_str("A8")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "B8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "C8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "D8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "E8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "F8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "G8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "H8"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "A7"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "A6"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "A5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "A4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "A3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "A2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "A8", "A1"))),
            ]
        );
    }

    #[test]
    fn rook_moves_white_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "   ♝    ",
                "        ",
                "        ",
                "   ♖  ♗ ",
                "        ",
                "        ",
                "   ♝    ",
            ],
        );
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "D4", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "D4", "F4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♖', "D4", "G4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "D4", "D3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "D4", "D2"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♖', "D4", "D1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "D4", "C4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "D4", "B4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "D4", "A4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "D4", "D5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "D4", "D6"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♖', "D4", "D7"))),
            ]
        );
    }

    #[test]
    fn rook_moves_black_capture() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "   ♗    ",
                "        ",
                "        ",
                "   ♜  ♝ ",
                "        ",
                "        ",
                "   ♗    ",
            ],
        );
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of_str("D4")),
            [
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "E4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "F4"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♜', "D4", "G4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D2"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♜', "D4", "D1"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "C4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "B4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "A4"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♜', "D4", "D6"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♜', "D4", "D7")))
            ]
        );
    }
}
