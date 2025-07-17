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
            player::GamePlayer,
        },
        pos::Pos,
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
                    Pos::of("A2"),
                    [
                        (Pos::of("A3"), PieceMoveType::Default),
                        (Pos::of("A4"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (
                    Pos::of("E1"),
                    [
                        (Pos::of("F2"), PieceMoveType::Default),
                        (Pos::of("F1"), PieceMoveType::Default),
                        (Pos::of("D1"), PieceMoveType::Default),
                        (Pos::of("D2"), PieceMoveType::Default),
                        (Pos::of("E2"), PieceMoveType::Default),
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
                    Pos::of("A8"),
                    [
                        (Pos::of("B8"), PieceMoveType::Default),
                        (Pos::of("C8"), PieceMoveType::Default),
                        (Pos::of("D8"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (
                    Pos::of("H8"),
                    [
                        (Pos::of("G8"), PieceMoveType::Default),
                        (Pos::of("F8"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (
                    Pos::of("E8"),
                    [
                        (Pos::of("F8"), PieceMoveType::Default),
                        (Pos::of("F7"), PieceMoveType::Default),
                        (Pos::of("E7"), PieceMoveType::Default),
                        (Pos::of("D7"), PieceMoveType::Default),
                        (Pos::of("D8"), PieceMoveType::Default),
                        (Pos::of("H8"), PieceMoveType::ShortCastling),
                        (Pos::of("A8"), PieceMoveType::LongCastling),
                    ]
                    .into(),
                ),
                (
                    Pos::of("A7"),
                    [
                        (Pos::of("A6"), PieceMoveType::Default),
                        (Pos::of("A5"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (Pos::of("H7"), [(Pos::of("H6"), PieceMoveType::Default)].into()),
                (
                    Pos::of("H5"),
                    [
                        (Pos::of("G6"), PieceMoveType::Default),
                        (Pos::of("F7"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (
                    Pos::of("G4"),
                    [
                        (Pos::of("G3"), PieceMoveType::Default),
                        (Pos::of("H3"), PieceMoveType::EnPassant),
                    ]
                    .into(),
                ),
            ]
            .into(),
        );
    }
}
