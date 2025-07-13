use crate::{
    game::{
        board::GameBoard,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{GameMove, GameMoveType, LongCastlingMove, ShortCastlingMove},
    },
    mov::Mov,
    piece::PieceType,
    pos::Pos,
};

pub fn castling_moves(
    board: &GameBoard,
    history: &GameHistory,
    players: &GamePlayers,
    king_pos: &Pos,
) -> Vec<GameMove> {
    let mut result: Vec<GameMove> = Vec::new();
    if let Some(short) = short_castling(board, history, players, king_pos) {
        result.push(short);
    }
    if let Some(long) = long_castling(board, history, players, king_pos) {
        result.push(long);
    }
    result
}

fn short_castling(
    board: &GameBoard,
    history: &GameHistory,
    players: &GamePlayers,
    king_pos: &Pos,
) -> Option<GameMove> {
    let king = board.get(&king_pos)?;
    let (rook_pos, _) = board.iter().find(|(pos, piece)| {
        piece.color == king.color && piece.typ == PieceType::Rook && pos.col > king_pos.col
    })?;
    let maybe_moved = history
        .iter()
        .find(|game_move| &game_move.mov.from == king_pos || &game_move.mov.from == rook_pos);
    if maybe_moved.is_some() {
        return None;
    }
    for col in (king_pos.col + 1)..rook_pos.col {
        if board.get(&Pos { row: king_pos.row, col }).is_some() {
            return None;
        }
    }
    for col in king_pos.col..=6 {
        for (color, player) in players {
            if color != &king.color {
                let player_moves_it = player.moves.iter();
                for (_, player_moves) in player_moves_it {
                    for game_move in player_moves {
                        match game_move.typ {
                            GameMoveType::Default(_)
                            | GameMoveType::Menace(_)
                            | GameMoveType::Capture(_) => {
                                if game_move.mov.to.col == col {
                                    return None;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    return Some(GameMove {
        mov: Mov { from: king_pos.clone(), to: rook_pos.clone(), piece: *king },
        typ: GameMoveType::ShortCastling(ShortCastlingMove),
    });
}

fn long_castling(
    board: &GameBoard,
    history: &GameHistory,
    players: &GamePlayers,
    king_pos: &Pos,
) -> Option<GameMove> {
    let king = board.get(&king_pos)?;
    let (rook_pos, _) = board.iter().find(|(pos, piece)| {
        piece.color == king.color && piece.typ == PieceType::Rook && pos.col < king_pos.col
    })?;
    let maybe_moved = history
        .iter()
        .find(|game_move| &game_move.mov.from == king_pos || &game_move.mov.from == rook_pos);
    if maybe_moved.is_some() {
        return None;
    }
    for col in (rook_pos.col + 1)..king_pos.col {
        if board.get(&Pos { row: king_pos.row, col }).is_some() {
            return None;
        }
    }
    for col in 2..=king_pos.col {
        for (color, player) in players {
            if color != &king.color {
                let player_moves_it = player.moves.iter();
                for (_, player_moves) in player_moves_it {
                    for game_move in player_moves {
                        match game_move.typ {
                            GameMoveType::Default(_)
                            | GameMoveType::Menace(_)
                            | GameMoveType::Capture(_) => {
                                if game_move.mov.to.col == col {
                                    return None;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    return Some(GameMove {
        mov: Mov { from: king_pos.clone(), to: rook_pos.clone(), piece: *king },
        typ: GameMoveType::LongCastling(LongCastlingMove),
    });
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        color::Color,
        game::{board::board_of_str, mode::standard_chess, mov::GameMove, player::GamePlayer},
        pos::Pos,
    };

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
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ]);
        let pos = Pos::of("E1");
        assert_eq!(
            castling_moves(&board, &history, &players, &pos),
            [GameMove::short_castling_of('♔', "E1", "H1")]
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
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ]);
        let pos = Pos::of("E1");
        assert_eq!(
            castling_moves(&board, &history, &players, &pos),
            [GameMove::long_castling_of('♔', "E1", "A1")]
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
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ]);
        let pos = Pos::of("E1");
        assert_eq!(
            castling_moves(&board, &history, &players, &pos),
            [
                GameMove::short_castling_of('♔', "E1", "H1"),
                GameMove::long_castling_of('♔', "E1", "A1"),
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
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ]);
        let pos = Pos::of("E1");
        assert_eq!(castling_moves(&board, &history, &players, &pos), []);
    }

    #[test]
    fn white_king_moved_rooks() {
        let mode = standard_chess();
        let history = vec![
            GameMove::default_of('♙', "A2", "A4"),
            GameMove::default_of('♟', "A7", "A6"),
            GameMove::default_of('♙', "H2", "H4"),
            GameMove::default_of('♟', "B7", "B6"),
            GameMove::default_of('♖', "A1", "A3"),
            GameMove::default_of('♟', "C7", "C6"),
            GameMove::default_of('♖', "H1", "H3"),
            GameMove::default_of('♟', "F7", "F6"),
            GameMove::default_of('♖', "A3", "A1"),
            GameMove::default_of('♟', "G7", "G6"),
            GameMove::default_of('♖', "H3", "H1"),
            GameMove::default_of('♟', "H7", "H6"),
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
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ]);
        let pos = Pos::of("E1");
        assert_eq!(castling_moves(&board, &history, &players, &pos), []);
    }

    #[test]
    fn white_king_moved_king() {
        let mode = standard_chess();
        let history = vec![
            GameMove::default_of('♔', "E1", "D1"),
            GameMove::default_of('♟', "A7", "A6"),
            GameMove::default_of('♔', "D1", "E1"),
            GameMove::default_of('♟', "H7", "H6"),
        ];
        let board = board_of_str(
            &mode.bounds,
            [
                "♜♞♝♛♚♝♞♜",
                " ♟♟♟♟♟♟ ",
                "♟      ♟",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖   ♔  ♖",
            ],
        );
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ]);
        let pos = Pos::of("E1");
        assert_eq!(castling_moves(&board, &history, &players, &pos), []);
    }
}
/*
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
