use std::collections::HashMap;

use crate::{
    game::{board::GameBoard, game::GameBounds, mov::PieceMoveType},
    pos::Pos,
};

pub fn king_moves(
    board: &GameBoard,
    bounds: &GameBounds,
    pos: &Pos,
) -> HashMap<Pos, PieceMoveType> {
    let mut result = HashMap::new();
    if let Some(piece) = board.get(pos) {
        let base = [
            pos.try_rel_idx(1, 1),
            pos.try_rel_idx(0, 1),
            pos.try_rel_idx(-1, 1),
            pos.try_rel_idx(-1, 0),
            pos.try_rel_idx(-1, -1),
            pos.try_rel_idx(0, -1),
            pos.try_rel_idx(1, -1),
            pos.try_rel_idx(1, 0),
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
                    if curr_piece.color != piece.color {
                        result.insert(curr_pos, PieceMoveType::Default);
                    }
                } else {
                    result.insert(curr_pos, PieceMoveType::Default);
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
            piece::game_piece_of,
        },
        pos::Pos,
    };

    use super::king_moves;

    #[test]
    fn king_moves_empty_board() {
        let mode = standard_chess();
        assert_eq!(king_moves(&board_empty(), &mode.bounds, &Pos::of("A1")), HashMap::new());
    }

    #[test]
    fn king_moves_lonely_piece() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("D4", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of("D4")),
            HashMap::from([
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("E3"), PieceMoveType::Default),
                (Pos::of("D3"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("C5"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn king_moves_top_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H8", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of("H8")),
            HashMap::from([
                (Pos::of("H7"), PieceMoveType::Default),
                (Pos::of("G7"), PieceMoveType::Default),
                (Pos::of("G8"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn king_moves_bottom_right_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("H1", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of("H1")),
            HashMap::from([
                (Pos::of("G1"), PieceMoveType::Default),
                (Pos::of("G2"), PieceMoveType::Default),
                (Pos::of("H2"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn king_moves_bottom_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A1", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of("A1")),
            HashMap::from([
                (Pos::of("B2"), PieceMoveType::Default),
                (Pos::of("B1"), PieceMoveType::Default),
                (Pos::of("A2"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn king_moves_top_left_edge() {
        let mode = standard_chess();
        let board = HashMap::from([game_piece_of("A8", '♚')]);
        assert_eq!(
            king_moves(&board, &mode.bounds, &Pos::of("A8")),
            HashMap::from([
                (Pos::of("B8"), PieceMoveType::Default),
                (Pos::of("B7"), PieceMoveType::Default),
                (Pos::of("A7"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn king_moves_small_bounds_top_right_edge() {
        let board = HashMap::from([game_piece_of("H8", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of("H8")),
            HashMap::from([
                (Pos::of("H7"), PieceMoveType::Default),
                (Pos::of("G7"), PieceMoveType::Default),
                (Pos::of("G8"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn king_moves_small_bounds_bottom_right_edge() {
        let board = HashMap::from([game_piece_of("H4", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of("H4")),
            HashMap::from([
                (Pos::of("G4"), PieceMoveType::Default),
                (Pos::of("G5"), PieceMoveType::Default),
                (Pos::of("H5"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn king_moves_small_bounds_bottom_left_edge() {
        let board = HashMap::from([game_piece_of("D4 ", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of("D4")),
            HashMap::from([
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
            ])
        );
    }

    #[test]
    fn king_moves_small_bounds_top_left_edge() {
        let board = HashMap::from([game_piece_of("D8", '♚')]);
        let bounds = GameBounds { x1: 3, y1: 3, x2: 7, y2: 7 };
        assert_eq!(
            king_moves(&board, &bounds, &Pos::of("D8")),
            HashMap::from([
                (Pos::of("E8"), PieceMoveType::Default),
                (Pos::of("E7"), PieceMoveType::Default),
                (Pos::of("D7"), PieceMoveType::Default),
            ])
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
            king_moves(&board, &mode.bounds, &Pos::of("D4")),
            HashMap::from([
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("E3"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("C5"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
            ])
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
            king_moves(&board, &mode.bounds, &Pos::of("D4")),
            HashMap::from([
                (Pos::of("E5"), PieceMoveType::Default),
                (Pos::of("E4"), PieceMoveType::Default),
                (Pos::of("E3"), PieceMoveType::Default),
                (Pos::of("C3"), PieceMoveType::Default),
                (Pos::of("C4"), PieceMoveType::Default),
                (Pos::of("C5"), PieceMoveType::Default),
                (Pos::of("D5"), PieceMoveType::Default),
            ])
        );
    }
}
