use std::collections::HashMap;

use crate::{
    game::{
        board::GameBoard,
        game::{GameHistory, GamePlayers},
        mov::PieceMoveType,
    },
    piece::PieceType,
    pos::Pos,
};

pub fn castling_moves(
    board: &GameBoard,
    history: &GameHistory,
    players: &GamePlayers,
    king_pos: &Pos,
) -> HashMap<Pos, PieceMoveType> {
    let mut result = HashMap::new();
    if let Some(rook_pos) = short_castling(board, history, players, king_pos) {
        result.insert(rook_pos, PieceMoveType::ShortCastling);
    }
    if let Some(rook_pos) = long_castling(board, history, players, king_pos) {
        result.insert(rook_pos, PieceMoveType::LongCastling);
    }
    result
}

fn short_castling(
    board: &GameBoard,
    history: &GameHistory,
    players: &GamePlayers,
    king_pos: &Pos,
) -> Option<Pos> {
    let king = board.get(king_pos)?;
    let (rook_pos, rook) = board.iter().find(|(pos, piece)| {
        piece.color == king.color && piece.typ == PieceType::Rook && pos.col > king_pos.col
    })?;
    let maybe_moved = history
        .iter()
        .find(|game_move| &game_move.mov.piece == king || &game_move.mov.piece == rook);
    if maybe_moved.is_some() {
        return None;
    }
    for col in (king_pos.col + 1)..rook_pos.col {
        if board.contains_key(&Pos { row: king_pos.row, col }) {
            return None;
        }
    }
    for col in king_pos.col..=6 {
        for (color, player) in players {
            if color != &king.color {
                let player_moves_it = player.moves.iter();
                for (_, player_moves) in player_moves_it {
                    if player_moves.contains_key(&Pos { row: king_pos.row, col }) {
                        return None;
                    }
                }
            }
        }
    }
    Some(rook_pos.clone())
}

fn long_castling(
    board: &GameBoard,
    history: &GameHistory,
    players: &GamePlayers,
    king_pos: &Pos,
) -> Option<Pos> {
    let king = board.get(king_pos)?;
    let (rook_pos, rook) = board.iter().find(|(pos, piece)| {
        piece.color == king.color && piece.typ == PieceType::Rook && pos.col < king_pos.col
    })?;
    let maybe_moved = history
        .iter()
        .find(|game_move| &game_move.mov.piece == king || &game_move.mov.piece == rook);
    if maybe_moved.is_some() {
        return None;
    }
    for col in (rook_pos.col + 1)..king_pos.col {
        if board.contains_key(&Pos { row: king_pos.row, col }) {
            return None;
        }
    }
    for col in 2..=king_pos.col {
        for (color, player) in players {
            if color != &king.color {
                let player_moves_it = player.moves.iter();
                for (_, player_moves) in player_moves_it {
                    if player_moves.contains_key(&Pos { row: king_pos.row, col }) {
                        return None;
                    }
                }
            }
        }
    }
    Some(rook_pos.clone())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        color::Color,
        game::{
            board::board_of_str,
            mode::standard_chess,
            mov::{GameMove, PieceMoveType},
            player::GamePlayer,
        },
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let pos = Pos::of("E1");
        assert_eq!(
            castling_moves(&board, &history, &players, &pos),
            [(Pos::of("H1"), PieceMoveType::ShortCastling)].into()
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let pos = Pos::of("E1");
        assert_eq!(
            castling_moves(&board, &history, &players, &pos),
            [(Pos::of("A1"), PieceMoveType::LongCastling)].into()
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let pos = Pos::of("E1");
        assert_eq!(
            castling_moves(&board, &history, &players, &pos),
            [
                (Pos::of("H1"), PieceMoveType::ShortCastling),
                (Pos::of("A1"), PieceMoveType::LongCastling),
            ]
            .into()
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let pos = Pos::of("E1");
        assert_eq!(castling_moves(&board, &history, &players, &pos), HashMap::new());
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let pos = Pos::of("E1");
        assert_eq!(castling_moves(&board, &history, &players, &pos), HashMap::new());
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let pos = Pos::of("E1");
        assert_eq!(castling_moves(&board, &history, &players, &pos), HashMap::new());
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
