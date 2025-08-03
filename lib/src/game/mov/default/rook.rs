use std::collections::HashMap;

use manfredo::matrix::rect::rect_u8::contains;

use crate::{
    game::{board::GameBoard, game::GameBounds, mov::PieceMoveType},
    pos::{pos_try_rel_idx, Pos},
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
                if let Some(curr_pos) = pos_try_rel_idx(pos, rel_row, rel_col) {
                    if !contains(bounds, &curr_pos) {
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
        pos::pos_of,
    };

    use super::rook_moves;

    #[test]
    fn rook_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(rook_moves(&board_empty(), &mode.bounds, &pos_of("A1")), HashMap::new());
    }

    #[test]
    fn rook_moves_lonely_piece() {
        let mode = standard_chess();
        let board = [(pos_of("D4"), Piece::of('♜'))].into();
        assert_eq!(
            rook_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("F4"), PieceMoveType::Default),
                (pos_of("G4"), PieceMoveType::Default),
                (pos_of("H4"), PieceMoveType::Default),
                (pos_of("D3"), PieceMoveType::Default),
                (pos_of("D2"), PieceMoveType::Default),
                (pos_of("D1"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A4"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("D7"), PieceMoveType::Default),
                (pos_of("D8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn rook_moves_small_bounds() {
        let board = [(pos_of("F6"), Piece::of('♜'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            rook_moves(&board, &bounds, &pos_of("F6")),
            [
                (pos_of("G6"), PieceMoveType::Default),
                (pos_of("H6"), PieceMoveType::Default),
                (pos_of("F5"), PieceMoveType::Default),
                (pos_of("F4"), PieceMoveType::Default),
                (pos_of("E6"), PieceMoveType::Default),
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("F7"), PieceMoveType::Default),
                (pos_of("F8"), PieceMoveType::Default)
            ]
            .into()
        );
    }

    #[test]
    fn rook_moves_top_right_edge() {
        let mode = standard_chess();
        let board = [(pos_of("H8"), Piece::of('♜'))].into();
        assert_eq!(
            rook_moves(&board, &mode.bounds, &pos_of("H8")),
            [
                (pos_of("H7"), PieceMoveType::Default),
                (pos_of("H6"), PieceMoveType::Default),
                (pos_of("H5"), PieceMoveType::Default),
                (pos_of("H4"), PieceMoveType::Default),
                (pos_of("H3"), PieceMoveType::Default),
                (pos_of("H2"), PieceMoveType::Default),
                (pos_of("H1"), PieceMoveType::Default),
                (pos_of("G8"), PieceMoveType::Default),
                (pos_of("F8"), PieceMoveType::Default),
                (pos_of("E8"), PieceMoveType::Default),
                (pos_of("D8"), PieceMoveType::Default),
                (pos_of("C8"), PieceMoveType::Default),
                (pos_of("B8"), PieceMoveType::Default),
                (pos_of("A8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn rook_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = [(pos_of("H1"), Piece::of('♜'))].into();
        assert_eq!(
            rook_moves(&board, &mode.bounds, &pos_of("H1")),
            [
                (pos_of("G1"), PieceMoveType::Default),
                (pos_of("F1"), PieceMoveType::Default),
                (pos_of("E1"), PieceMoveType::Default),
                (pos_of("D1"), PieceMoveType::Default),
                (pos_of("C1"), PieceMoveType::Default),
                (pos_of("B1"), PieceMoveType::Default),
                (pos_of("A1"), PieceMoveType::Default),
                (pos_of("H2"), PieceMoveType::Default),
                (pos_of("H3"), PieceMoveType::Default),
                (pos_of("H4"), PieceMoveType::Default),
                (pos_of("H5"), PieceMoveType::Default),
                (pos_of("H6"), PieceMoveType::Default),
                (pos_of("H7"), PieceMoveType::Default),
                (pos_of("H8"), PieceMoveType::Default)
            ]
            .into()
        );
    }

    #[test]
    fn rook_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = [(pos_of("A1"), Piece::of('♜'))].into();
        assert_eq!(
            rook_moves(&board, &mode.bounds, &pos_of("A1")),
            [
                (pos_of("B1"), PieceMoveType::Default),
                (pos_of("C1"), PieceMoveType::Default),
                (pos_of("D1"), PieceMoveType::Default),
                (pos_of("E1"), PieceMoveType::Default),
                (pos_of("F1"), PieceMoveType::Default),
                (pos_of("G1"), PieceMoveType::Default),
                (pos_of("H1"), PieceMoveType::Default),
                (pos_of("A2"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("A4"), PieceMoveType::Default),
                (pos_of("A5"), PieceMoveType::Default),
                (pos_of("A6"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
                (pos_of("A8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn rook_moves_top_left_edge() {
        let mode = standard_chess();
        let board = [(pos_of("A8"), Piece::of('♜'))].into();
        assert_eq!(
            rook_moves(&board, &mode.bounds, &pos_of("A8")),
            [
                (pos_of("B8"), PieceMoveType::Default),
                (pos_of("C8"), PieceMoveType::Default),
                (pos_of("D8"), PieceMoveType::Default),
                (pos_of("E8"), PieceMoveType::Default),
                (pos_of("F8"), PieceMoveType::Default),
                (pos_of("G8"), PieceMoveType::Default),
                (pos_of("H8"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
                (pos_of("A6"), PieceMoveType::Default),
                (pos_of("A5"), PieceMoveType::Default),
                (pos_of("A4"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("A2"), PieceMoveType::Default),
                (pos_of("A1"), PieceMoveType::Default),
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
            rook_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("F4"), PieceMoveType::Default),
                (pos_of("D3"), PieceMoveType::Default),
                (pos_of("D2"), PieceMoveType::Default),
                (pos_of("D1"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A4"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("D7"), PieceMoveType::Default),
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
            rook_moves(&board, &mode.bounds, &pos_of("D4")),
            [
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("F4"), PieceMoveType::Default),
                (pos_of("D3"), PieceMoveType::Default),
                (pos_of("D2"), PieceMoveType::Default),
                (pos_of("D1"), PieceMoveType::Default),
                (pos_of("C4"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A4"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("D7"), PieceMoveType::Default)
            ]
            .into()
        );
    }
}
