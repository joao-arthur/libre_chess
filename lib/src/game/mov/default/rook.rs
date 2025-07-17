use std::collections::HashMap;

use crate::{
    game::{board::GameBoard, game::GameBounds, mov::PieceMoveType},
    pos::Pos,
};

pub fn rook_moves(
    board: &GameBoard,
    bounds: &GameBounds,
    pos: &Pos,
) -> HashMap<Pos, PieceMoveType> {
    let mut result = HashMap::new();
    if let Some(piece) = board.get(pos) {
        let modifiers: [[i8; 2]; 4] = [[0, 1], [-1, 0], [0, -1], [1, 0]];
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
                        if curr_piece.color != piece.color {
                            result.insert(curr_pos, PieceMoveType::Default);
                        }
                        break;
                    } else {
                        result.insert(curr_pos, PieceMoveType::Default);
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
            mov::PieceMoveType,
        },
        piece::Piece,
        pos::Pos,
    };

    use super::rook_moves;

    #[test]
    fn rook_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(rook_moves(&board_empty(), &mode.bounds, &Pos::of("A1")), HashMap::new());
    }

    #[test]
    fn rook_moves_lonely_piece() {
        let mode = standard_chess();
        let board = [(Pos::of("D4"), Piece::of('♜'))].into();
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of("D4")),
            [
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("F4"), PieceMoveType::Default),
                (Pos::of("G4"), PieceMoveType::Default),
                (Pos::of("H4"), PieceMoveType::Default),
                (Pos::of("D3"), PieceMoveType::Default),
                (Pos::of("D2"), PieceMoveType::Default),
                (Pos::of("D1"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A4"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("D7"), PieceMoveType::Default),
                (Pos::of("D8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn rook_moves_small_bounds() {
        let board = [(Pos::of("F6"), Piece::of('♜'))].into();
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            rook_moves(&board, &bounds, &Pos::of("F6")),
            [
                (Pos::of("G6"), PieceMoveType::Default),
                (Pos::of("H6"), PieceMoveType::Default),
                (Pos::of("F5"), PieceMoveType::Default),
                (Pos::of("F4"), PieceMoveType::Default),
                (Pos::of("E6"), PieceMoveType::Default),
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("F7"), PieceMoveType::Default),
                (Pos::of("F8"), PieceMoveType::Default)
            ]
            .into()
        );
    }

    #[test]
    fn rook_moves_top_right_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("H8"), Piece::of('♜'))].into();
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of("H8")),
            [
                (Pos::of("H7"), PieceMoveType::Default),
                (Pos::of("H6"), PieceMoveType::Default),
                (Pos::of("H5"), PieceMoveType::Default),
                (Pos::of("H4"), PieceMoveType::Default),
                (Pos::of("H3"), PieceMoveType::Default),
                (Pos::of("H2"), PieceMoveType::Default),
                (Pos::of("H1"), PieceMoveType::Default),
                (Pos::of("G8"), PieceMoveType::Default),
                (Pos::of("F8"), PieceMoveType::Default),
                (Pos::of("E8"), PieceMoveType::Default),
                (Pos::of("D8"), PieceMoveType::Default),
                (Pos::of("C8"), PieceMoveType::Default),
                (Pos::of("B8"), PieceMoveType::Default),
                (Pos::of("A8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn rook_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("H1"), Piece::of('♜'))].into();
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of("H1")),
            [
                (Pos::of("G1"), PieceMoveType::Default),
                (Pos::of("F1"), PieceMoveType::Default),
                (Pos::of("E1"), PieceMoveType::Default),
                (Pos::of("D1"), PieceMoveType::Default),
                (Pos::of("C1"), PieceMoveType::Default),
                (Pos::of("B1"), PieceMoveType::Default),
                (Pos::of("A1"), PieceMoveType::Default),
                (Pos::of("H2"), PieceMoveType::Default),
                (Pos::of("H3"), PieceMoveType::Default),
                (Pos::of("H4"), PieceMoveType::Default),
                (Pos::of("H5"), PieceMoveType::Default),
                (Pos::of("H6"), PieceMoveType::Default),
                (Pos::of("H7"), PieceMoveType::Default),
                (Pos::of("H8"), PieceMoveType::Default)
            ]
            .into()
        );
    }

    #[test]
    fn rook_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("A1"), Piece::of('♜'))].into();
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of("A1")),
            [
                (Pos::of("B1"), PieceMoveType::Default),
                (Pos::of("C1"), PieceMoveType::Default),
                (Pos::of("D1"), PieceMoveType::Default),
                (Pos::of("E1"), PieceMoveType::Default),
                (Pos::of("F1"), PieceMoveType::Default),
                (Pos::of("G1"), PieceMoveType::Default),
                (Pos::of("H1"), PieceMoveType::Default),
                (Pos::of("A2"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("A4"), PieceMoveType::Default),
                (Pos::of("A5"), PieceMoveType::Default),
                (Pos::of("A6"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
                (Pos::of("A8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn rook_moves_top_left_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("A8"), Piece::of('♜'))].into();
        assert_eq!(
            rook_moves(&board, &mode.bounds, &Pos::of("A8")),
            [
                (Pos::of("B8"), PieceMoveType::Default),
                (Pos::of("C8"), PieceMoveType::Default),
                (Pos::of("D8"), PieceMoveType::Default),
                (Pos::of("E8"), PieceMoveType::Default),
                (Pos::of("F8"), PieceMoveType::Default),
                (Pos::of("G8"), PieceMoveType::Default),
                (Pos::of("H8"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
                (Pos::of("A6"), PieceMoveType::Default),
                (Pos::of("A5"), PieceMoveType::Default),
                (Pos::of("A4"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("A2"), PieceMoveType::Default),
                (Pos::of("A1"), PieceMoveType::Default),
            ]
            .into()
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
            rook_moves(&board, &mode.bounds, &Pos::of("D4")),
            [
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("F4"), PieceMoveType::Default),
                (Pos::of("D3"), PieceMoveType::Default),
                (Pos::of("D2"), PieceMoveType::Default),
                (Pos::of("D1"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A4"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("D7"), PieceMoveType::Default),
            ]
            .into()
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
            rook_moves(&board, &mode.bounds, &Pos::of("D4")),
            [
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("F4"), PieceMoveType::Default),
                (Pos::of("D3"), PieceMoveType::Default),
                (Pos::of("D2"), PieceMoveType::Default),
                (Pos::of("D1"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A4"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("D7"), PieceMoveType::Default)
            ]
            .into()
        );
    }
}
