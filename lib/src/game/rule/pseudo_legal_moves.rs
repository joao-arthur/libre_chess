use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        board::GameBoard,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{
            default::default_moves,
            special::{castling::castling_moves, en_passant::en_passant_moves},
        },
        player::PlayerMoves,
    },
    piece::PieceType,
};

pub fn pseudo_legal_moves_of_player(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    players: &GamePlayers,
    color: &Color,
) -> PlayerMoves {
    let mut result = HashMap::new();
    for (pos, piece) in board {
        if &piece.color == color {
            let mut moves = default_moves(board, bounds, pos);
            if piece.typ == PieceType::Pawn {
                moves.extend(en_passant_moves(board, history, pos));
            }
            if piece.typ == PieceType::King {
                moves.extend(castling_moves(board, history, players, pos));
            }
            if !moves.is_empty() {
                result.insert(pos.clone(), moves);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use crate::{
        color::Color,
        game::{
            board::board_of_str,
            game::empty_players,
            mode::standard_chess,
            mov::{GameMove, PieceMoveType},
        },
        pos::pos_of,
    };

    use super::pseudo_legal_moves_of_player;

    #[test]
    fn pseudo_legal_moves_of_player_standard_moves() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "       ♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙       ",
                "    ♔   ",
            ],
        );
        let bounds = mode.bounds;
        let history = Vec::new();
        let players = empty_players();
        let color = Color::White;
        assert_eq!(
            pseudo_legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [
                (
                    pos_of("A2"),
                    [
                        (pos_of("A3"), PieceMoveType::Default),
                        (pos_of("A4"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (
                    pos_of("E1"),
                    [
                        (pos_of("F2"), PieceMoveType::Default),
                        (pos_of("F1"), PieceMoveType::Default),
                        (pos_of("D1"), PieceMoveType::Default),
                        (pos_of("D2"), PieceMoveType::Default),
                        (pos_of("E2"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
            ]
            .into(),
        );
    }

    #[test]
    fn pseudo_legal_moves_of_player_special_moves() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "♜   ♚  ♜",
                "♟      ♟",
                "        ",
                "       ♝",
                "      ♟♙",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let bounds = mode.bounds;
        let history = vec![GameMove::default_of('♙', "H2", "H4")];
        let players = empty_players();
        let color = Color::Black;
        assert_eq!(
            pseudo_legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [
                (
                    pos_of("A8"),
                    [
                        (pos_of("B8"), PieceMoveType::Default),
                        (pos_of("C8"), PieceMoveType::Default),
                        (pos_of("D8"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (
                    pos_of("H8"),
                    [
                        (pos_of("G8"), PieceMoveType::Default),
                        (pos_of("F8"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (
                    pos_of("E8"),
                    [
                        (pos_of("F8"), PieceMoveType::Default),
                        (pos_of("F7"), PieceMoveType::Default),
                        (pos_of("E7"), PieceMoveType::Default),
                        (pos_of("D7"), PieceMoveType::Default),
                        (pos_of("D8"), PieceMoveType::Default),
                        (pos_of("H8"), PieceMoveType::ShortCastling),
                        (pos_of("A8"), PieceMoveType::LongCastling),
                    ]
                    .into(),
                ),
                (
                    pos_of("A7"),
                    [
                        (pos_of("A6"), PieceMoveType::Default),
                        (pos_of("A5"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (pos_of("H7"), [(pos_of("H6"), PieceMoveType::Default)].into()),
                (
                    pos_of("H5"),
                    [
                        (pos_of("G6"), PieceMoveType::Default),
                        (pos_of("F7"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (
                    pos_of("G4"),
                    [
                        (pos_of("G3"), PieceMoveType::Default),
                        (pos_of("H3"), PieceMoveType::EnPassant),
                    ]
                    .into(),
                ),
            ]
            .into(),
        );
    }
}
