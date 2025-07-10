use crate::{
    game::{
        board::GameBoard,
        game::{GameBounds, GameHistory},
        mov::CastlingMovOld,
    },
    mov::Mov,
    piece::Type,
    pos::Pos,
};

// alterar castling pra verificar de c1/8 e king g1/8

pub fn castling_moves(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    pos: &Pos,
) -> Vec<CastlingMovOld> {
    let mut result: Vec<CastlingMovOld> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let mut col_index = 0;
        loop {
            col_index += 1;
            let curr_pos = Pos { row: pos.row, col: pos.col + col_index };
            if curr_pos.col < bounds.x1
                || curr_pos.col > bounds.x2
                || curr_pos.row < bounds.y1
                || curr_pos.row > bounds.y2
            {
                break;
            }
            if let Some(maybe_rook) = board.get(&curr_pos) {
                if maybe_rook.t == Type::Rook
                    && maybe_rook.color == piece.color
                    && !history.iter().any(|mov| mov.piece == *maybe_rook)
                {
                    result.push(CastlingMovOld::from(Mov {
                        piece: *piece,
                        from: pos.clone(),
                        to: curr_pos,
                    }));
                }
                break;
            }
        }
        loop {
            col_index += 1;
            if col_index > pos.col {
                break;
            }
            let curr_pos = Pos { row: pos.row, col: pos.col - col_index };
            if curr_pos.col < bounds.x1
                || curr_pos.col > bounds.x2
                || curr_pos.row < bounds.y1
                || curr_pos.row > bounds.y2
            {
                continue;
            }
            if let Some(maybe_rook) = board.get(&curr_pos) {
                if maybe_rook.t == Type::Rook
                    && maybe_rook.color == piece.color
                    && !history.iter().any(|mov| mov.piece == *maybe_rook)
                {
                    result.push(CastlingMovOld::from(Mov {
                        piece: *piece,
                        from: pos.clone(),
                        to: curr_pos,
                    }));
                }
                break;
            }
        }
    }
    result
}
/*
#[cfg(test)]
mod tests {
    use crate::{game::{board::board_of_str, mode::standard_chess, mov::{CastlingMovOld, DefaultMovOld, GameMovOld}}, mov::Mov, pos::Pos};

    use super::castling_moves;

    #[test]
    fn white_king_short_castling() {
        let mode = standard_chess();
        let history = Vec::new();
        let board = board_of_str(
            &mode.bounds,
            [
                "♜♞♝♛♚♝♞♜",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖♘♗♕♔  ♖",
            ],
        );
        let pos = Pos::of_str("E1");
        assert_eq!(
            castling_moves(&board, &mode.bounds, &history, &pos),
            [CastlingMovOld::from(Mov::of('♔', "E1", "H1"))]
        );
    }

    #[test]
    fn white_king_long_castling() {
        let mode = standard_chess();
        let history = Vec::new();
        let board = board_of_str(
            &mode.bounds,
            [
                "♜♞♝♛♚♝♞♜",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖   ♔♗♘♖",
            ],
        );
        let pos = Pos::of_str("E1");
        assert_eq!(
            castling_moves(&board, &mode.bounds, &history, &pos),
            [CastlingMovOld::from(Mov::of('♔', "E1", "A1"))]
        );
    }

    #[test]
    fn white_king_both_castlings() {
        let mode = standard_chess();
        let history = Vec::new();
        let board = board_of_str(
            &mode.bounds,
            [
                "♜♞♝♛♚♝♞♜",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖   ♔  ♖",
            ],
        );
        let pos = Pos::of_str("E1");
        assert_eq!(
            castling_moves(&board, &mode.bounds, &history, &pos),
            [
                CastlingMovOld::from(Mov::of('♔', "E1", "A1")),
                CastlingMovOld::from(Mov::of('♔', "E1", "H1")),
            ]
        );
    }

    #[test]
    fn white_king_initial_board() {
        let mode = standard_chess();
        let history = Vec::new();
        let board = board_of_str(
            &mode.bounds,
            [
                "♜♞♝♛♚♝♞♜",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖♘♗♕♔♗♘♖",
            ],
        );
        let pos = Pos::of_str("E1");
        assert_eq!(castling_moves(&board, &mode.bounds, &history, &pos), []);
    }

    #[test]
    fn white_king_moved_rooks() {
        let mode = standard_chess();
        let history = vec![
            GameMovOld::from(DefaultMovOld::from(Mov::of('♙', "A2", "A4"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "A7", "A6"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♙', "H2", "H4"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "B7", "B6"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "A1", "A3"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "C7", "C6"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "H1", "H3"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "F7", "F6"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "A3", "A1"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "G7", "G6"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♖', "H3", "H1"))),
            GameMovOld::from(DefaultMovOld::from(Mov::of('♟', "H7", "H6"))),
        ];
        let board = board_of_str(
            &mode.bounds,
            [
                "♜♞♝♛♚♝♞♜",
                "   ♟♟   ",
                "♟♟♟  ♟♟♟",
                "        ",
                "♙      ♙",
                "        ",
                " ♙♙♙♙♙♙ ",
                "♖   ♔  ♖",
            ],
        );
        let pos = Pos::of_str("E1");
        assert_eq!(castling_moves(&board, &mode.bounds, &history, &pos), []);
    }
    #[test]
    fn white_king_moved_king() {
        let mode = standard_chess();
        let history = Vec::new();
        let board = board_of_str(
            &mode.bounds,
            [
                "♜♞♝♛♚♝♞♜",
                " ♟♟♟♟♟♟ ",
                "♟      ♟",
                "        ",
                "♙      ♙",
                "        ",
                " ♙♙♙♙♙♙ ",
                "♖   ♔  ♖",
            ],
        );
        let pos = Pos::of_str("E1");
        assert_eq!(castling_moves(&board, &mode.bounds, &history, &pos), []);
    }

    // fn menace_b1
    // fn menace_c1
    // fn menace_d1

    // fn menace_f1
    // fn menace_g1


    // fn king_b1_menace_b1
    // fn king_b1_menace_c1
    // fn king_b1_menace_d1

    // fn king_b1_menace_f1
    // fn king_b1_menace_g1


    // fn king_g1_menace_b1
    // fn king_g1_menace_c1
    // fn king_g1_menace_d1

    // fn king_g1_menace_f1
    // fn king_g1_menace_g1


}
 */