use std::collections::HashMap;

use manfredo::matrix::rect::rect_u8::contains;

use crate::{
    game::{board::GameBoard, game::GameBounds, mov::PieceMoveType},
    pos::Pos,
};

pub fn bishop_moves(
    board: &GameBoard,
    bounds: &GameBounds,
    pos: &Pos,
) -> HashMap<Pos, PieceMoveType> {
    let mut result = HashMap::new();
    if let Some(piece) = board.get(pos) {
        let modifiers: [[i8; 2]; 4] = [[1, 1], [-1, 1], [-1, -1], [1, -1]];
        for modifier in modifiers {
            let mut rel_row: i8 = 0;
            let mut rel_col: i8 = 0;
            loop {
                rel_row += modifier[0];
                rel_col += modifier[1];
                if let Some(curr_pos) = pos.try_rel_idx(rel_row, rel_col) {
                    if curr_pos.col < bounds.min.col
                        || curr_pos.col > bounds.max.col
                        || curr_pos.row < bounds.min.row
                        || curr_pos.row > bounds.max.row
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

    use super::bishop_moves;

    #[test]
    fn bishop_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(bishop_moves(&board_empty(), &mode.bounds, &Pos::of("A1")), HashMap::new());
    }

    #[test]
    fn bishop_moves_lonely_piece() {
        let mode = standard_chess();
        let board = [(Pos::of("C5"), Piece::of('♝'))].into();
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of("C5")),
            [
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("E7"), PieceMoveType::Default),
                (Pos::of("F8"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("E3"), PieceMoveType::Default),
                (Pos::of("F2"), PieceMoveType::Default),
                (Pos::of("G1"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("B6"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn bishop_moves_small_bounds() {
        let board = [(Pos::of("F6"), Piece::of('♝'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            bishop_moves(&board, &bounds, &Pos::of("F6")),
            [
                (Pos::of("G7"), PieceMoveType::Default),
                (Pos::of("H8"), PieceMoveType::Default),
                (Pos::of("G5"), PieceMoveType::Default),
                (Pos::of("H4"), PieceMoveType::Default),
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("E7"), PieceMoveType::Default),
                (Pos::of("D8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn bishop_moves_top_right_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("H8"), Piece::of('♝'))].into();
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of("H8")),
            [
                (Pos::of("G7"), PieceMoveType::Default),
                (Pos::of("F6"), PieceMoveType::Default),
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("B2"), PieceMoveType::Default),
                (Pos::of("A1"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn bishop_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("H1"), Piece::of('♝'))].into();
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of("H1")),
            [
                (Pos::of("G2"), PieceMoveType::Default),
                (Pos::of("F3"), PieceMoveType::Default),
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
                (Pos::of("B7"), PieceMoveType::Default),
                (Pos::of("A8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn bishop_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("A1"), Piece::of('♝'))].into();
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of("A1")),
            [
                (Pos::of("B2"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("F6"), PieceMoveType::Default),
                (Pos::of("G7"), PieceMoveType::Default),
                (Pos::of("H8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn bishop_moves_top_left_edge() {
        let mode = standard_chess();
        let board = [(Pos::of("A8"), Piece::of('♝'))].into();
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &Pos::of("A8")),
            [
                (Pos::of("B7"), PieceMoveType::Default),
                (Pos::of("C6"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("F3"), PieceMoveType::Default),
                (Pos::of("G2"), PieceMoveType::Default),
                (Pos::of("H1"), PieceMoveType::Default),
            ]
            .into()
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
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("B6"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
            ]
            .into()
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
                (Pos::of("D6"), PieceMoveType::Default),
                (Pos::of("D4"), PieceMoveType::Default),
                (Pos::of("B4"), PieceMoveType::Default),
                (Pos::of("A3"), PieceMoveType::Default),
                (Pos::of("B6"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
            ]
            .into()
        );
    }
}
