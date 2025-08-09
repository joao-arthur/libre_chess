use std::collections::HashMap;

use manfredo::matrix::{
    point::point_i8::PointI8, point::point_u8::checked_translated, rect::rect_u8::contains,
};

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
                if let Some(curr_pos) = checked_translated(pos, &PointI8::of(rel_row, rel_col)) {
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

    use super::bishop_moves;

    #[test]
    fn bishop_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(bishop_moves(&board_empty(), &mode.bounds, &pos_of("A1")), HashMap::new());
    }

    #[test]
    fn bishop_moves_lonely_piece() {
        let mode = standard_chess();
        let board = [(pos_of("C5"), Piece::of('♝'))].into();
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &pos_of("C5")),
            [
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("E7"), PieceMoveType::Default),
                (pos_of("F8"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("E3"), PieceMoveType::Default),
                (pos_of("F2"), PieceMoveType::Default),
                (pos_of("G1"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("B6"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn bishop_moves_small_bounds() {
        let board = [(pos_of("F6"), Piece::of('♝'))].into();
        let bounds = GameBounds::of(3, 3, 7, 7);
        assert_eq!(
            bishop_moves(&board, &bounds, &pos_of("F6")),
            [
                (pos_of("G7"), PieceMoveType::Default),
                (pos_of("H8"), PieceMoveType::Default),
                (pos_of("G5"), PieceMoveType::Default),
                (pos_of("H4"), PieceMoveType::Default),
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("E7"), PieceMoveType::Default),
                (pos_of("D8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn bishop_moves_top_right_edge() {
        let mode = standard_chess();
        let board = [(pos_of("H8"), Piece::of('♝'))].into();
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &pos_of("H8")),
            [
                (pos_of("G7"), PieceMoveType::Default),
                (pos_of("F6"), PieceMoveType::Default),
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("B2"), PieceMoveType::Default),
                (pos_of("A1"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn bishop_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = [(pos_of("H1"), Piece::of('♝'))].into();
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &pos_of("H1")),
            [
                (pos_of("G2"), PieceMoveType::Default),
                (pos_of("F3"), PieceMoveType::Default),
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default),
                (pos_of("B7"), PieceMoveType::Default),
                (pos_of("A8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn bishop_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = [(pos_of("A1"), Piece::of('♝'))].into();
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &pos_of("A1")),
            [
                (pos_of("B2"), PieceMoveType::Default),
                (pos_of("C3"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("E5"), PieceMoveType::Default),
                (pos_of("F6"), PieceMoveType::Default),
                (pos_of("G7"), PieceMoveType::Default),
                (pos_of("H8"), PieceMoveType::Default),
            ]
            .into()
        );
    }

    #[test]
    fn bishop_moves_top_left_edge() {
        let mode = standard_chess();
        let board = [(pos_of("A8"), Piece::of('♝'))].into();
        assert_eq!(
            bishop_moves(&board, &mode.bounds, &pos_of("A8")),
            [
                (pos_of("B7"), PieceMoveType::Default),
                (pos_of("C6"), PieceMoveType::Default),
                (pos_of("D5"), PieceMoveType::Default),
                (pos_of("E4"), PieceMoveType::Default),
                (pos_of("F3"), PieceMoveType::Default),
                (pos_of("G2"), PieceMoveType::Default),
                (pos_of("H1"), PieceMoveType::Default),
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
            bishop_moves(&board, &mode.bounds, &pos_of("C5")),
            [
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("B6"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
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
            bishop_moves(&board, &mode.bounds, &pos_of("C5")),
            [
                (pos_of("D6"), PieceMoveType::Default),
                (pos_of("D4"), PieceMoveType::Default),
                (pos_of("B4"), PieceMoveType::Default),
                (pos_of("A3"), PieceMoveType::Default),
                (pos_of("B6"), PieceMoveType::Default),
                (pos_of("A7"), PieceMoveType::Default),
            ]
            .into()
        );
    }
}
