use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        board::{GameBoard, board_to_string},
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{GameMove, GameMoveType},
        player::PlayerMoves,
    },
    piece::PieceType,
    pos::Pos,
};

use super::{
    check::is_in_check, pseudo_legal_moves::pseudo_legal_moves_of_player, turn::evaluate_turn,
};

pub fn legal_moves_of_player(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    players: &GamePlayers,
    color: &Color,
) -> PlayerMoves {
    let turn = evaluate_turn(history);
    let in_check = is_in_check(board, players, history);
    let mut pseudo_legal_moves =
        pseudo_legal_moves_of_player(board, bounds, history, players, color);
    for (from, piece_moves) in pseudo_legal_moves.clone() {
        let actual_moves = pseudo_legal_moves.get_mut(&from).unwrap();
        for (to, move_type) in piece_moves {
            let mut temp_board = board.clone();
            if let Some(piece) = temp_board.remove(&from) {
                temp_board.insert(to.clone(), piece);
                let mut temp_players = players.clone();
                for player in temp_players.values_mut() {
                    if &player.color != color {
                        player.moves = pseudo_legal_moves_of_player(
                            &temp_board,
                            bounds,
                            history,
                            players,
                            &player.color,
                        );
                    }
                }
                if is_in_check(&temp_board, &temp_players, history) {
                    actual_moves.remove(&to);
                }
            }
        }
    }

    //let _: Option<()> = (|| {
    //    let (king_pos, _) =
    //        board.iter().find(|(_, piece)| piece.typ == PieceType::King && piece.color == turn)?;
    //    let king_moves = pseudo_legal_moves.get_mut(&king_pos)?;
    //    for (curr_color, curr_player) in players {
    //        if curr_color != color {
    //            for (_, curr_piece_moves) in curr_player.moves.iter() {
    //                for menace_game_move in curr_piece_moves {
    //                    if menace_game_move.typ == GameMoveType::Default
    //                        || menace_game_move.typ == GameMoveType::Capture
    //                        || menace_game_move.typ == GameMoveType::Menace
    //                    {
    //                        king_moves.retain(|game_move| game_move.mov.to != menace_game_move.mov.to);
    //                    }
    //                }
    //            }
    //        }
    //    }
    //    None
    //})();

    pseudo_legal_moves
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        color::Color,
        game::{
            board::board_of_str,
            game::empty_players,
            mode::standard_chess,
            mov::{GameMove, PieceMoveType},
            player::GamePlayer,
        },
        pos::pos_of,
    };

    use super::legal_moves_of_player;

    #[test]
    fn legal_moves_of_player_standard_moves() {
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
            legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [
                (
                    pos_of("A2"),
                    [
                        (pos_of("A3"), PieceMoveType::Default),
                        (pos_of("A4"), PieceMoveType::Default),
                    ]
                    .into()
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
                    .into()
                ),
            ]
            .into()
        )
    }

    #[test]
    fn legal_moves_of_player_special_moves() {
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
            legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [
                (
                    pos_of("A8"),
                    [
                        (pos_of("B8"), PieceMoveType::Default),
                        (pos_of("C8"), PieceMoveType::Default),
                        (pos_of("D8"), PieceMoveType::Default),
                    ]
                    .into()
                ),
                (
                    pos_of("H8"),
                    [
                        (pos_of("G8"), PieceMoveType::Default),
                        (pos_of("F8"), PieceMoveType::Default),
                    ]
                    .into()
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
                    .into()
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
            .into()
        );
    }

    #[test]
    fn legal_moves_of_player_free_king() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "        ",
                "        ",
                "        ",
                "   ♔    ",
                "        ",
                "        ",
                "        ",
            ],
        );
        let bounds = mode.bounds;
        let history = Vec::new();
        let players = empty_players();
        let color = Color::White;
        assert_eq!(
            legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [(
                pos_of("D4"),
                [
                    (pos_of("E5"), PieceMoveType::Default),
                    (pos_of("E4"), PieceMoveType::Default),
                    (pos_of("E3"), PieceMoveType::Default),
                    (pos_of("D3"), PieceMoveType::Default),
                    (pos_of("C3"), PieceMoveType::Default),
                    (pos_of("C4"), PieceMoveType::Default),
                    (pos_of("C5"), PieceMoveType::Default),
                    (pos_of("D5"), PieceMoveType::Default),
                ]
                .into()
            )]
            .into()
        )
    }

    #[test]
    fn legal_moves_of_player_king_blocked_by_menace() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "        ",
                "        ",
                "   ♟    ",
                "   ♟    ",
                " ♟ ♔ ♟  ",
                "        ",
                "        ",
                "        ",
            ],
        );
        let bounds = mode.bounds;
        let history = Vec::new();
        let players = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [
                        (pos_of("B4"), [(pos_of("B3"), PieceMoveType::Default)].into()),
                        (pos_of("F4"), [(pos_of("F3"), PieceMoveType::Default)].into()),
                    ]
                    .into(),
                },
            ),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let color = Color::White;
        assert_eq!(
            legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [(
                pos_of("D4"),
                [(pos_of("D3"), PieceMoveType::Default), (pos_of("D5"), PieceMoveType::Default)]
                    .into()
            )]
            .into()
        );
    }

    #[test]
    fn legal_moves_of_player_king_blocked_by_default() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "  ♜     ",
                "        ",
                "        ",
                "       ♜",
                "   ♔    ",
                "♜       ",
                "        ",
                "    ♜   ",
            ],
        );
        let bounds = mode.bounds;
        let history = Vec::new();
        let players = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("C8"),
                            [
                                (pos_of("D8"), PieceMoveType::Default),
                                (pos_of("E8"), PieceMoveType::Default),
                                (pos_of("F8"), PieceMoveType::Default),
                                (pos_of("G8"), PieceMoveType::Default),
                                (pos_of("H8"), PieceMoveType::Default),
                                (pos_of("C7"), PieceMoveType::Default),
                                (pos_of("C6"), PieceMoveType::Default),
                                (pos_of("C5"), PieceMoveType::Default),
                                (pos_of("C4"), PieceMoveType::Default),
                                (pos_of("C3"), PieceMoveType::Default),
                                (pos_of("C2"), PieceMoveType::Default),
                                (pos_of("C1"), PieceMoveType::Default),
                                (pos_of("B8"), PieceMoveType::Default),
                                (pos_of("A8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("H5"),
                            [
                                (pos_of("H4"), PieceMoveType::Default),
                                (pos_of("H3"), PieceMoveType::Default),
                                (pos_of("H2"), PieceMoveType::Default),
                                (pos_of("H1"), PieceMoveType::Default),
                                (pos_of("G5"), PieceMoveType::Default),
                                (pos_of("F5"), PieceMoveType::Default),
                                (pos_of("E5"), PieceMoveType::Default),
                                (pos_of("D5"), PieceMoveType::Default),
                                (pos_of("C5"), PieceMoveType::Default),
                                (pos_of("B5"), PieceMoveType::Default),
                                (pos_of("A5"), PieceMoveType::Default),
                                (pos_of("H6"), PieceMoveType::Default),
                                (pos_of("H7"), PieceMoveType::Default),
                                (pos_of("H8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("A3"),
                            [
                                (pos_of("B3"), PieceMoveType::Default),
                                (pos_of("C3"), PieceMoveType::Default),
                                (pos_of("D3"), PieceMoveType::Default),
                                (pos_of("E3"), PieceMoveType::Default),
                                (pos_of("F3"), PieceMoveType::Default),
                                (pos_of("G3"), PieceMoveType::Default),
                                (pos_of("H3"), PieceMoveType::Default),
                                (pos_of("A2"), PieceMoveType::Default),
                                (pos_of("A1"), PieceMoveType::Default),
                                (pos_of("A4"), PieceMoveType::Default),
                                (pos_of("A5"), PieceMoveType::Default),
                                (pos_of("A6"), PieceMoveType::Default),
                                (pos_of("A7"), PieceMoveType::Default),
                                (pos_of("A8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("E1"),
                            [
                                (pos_of("F1"), PieceMoveType::Default),
                                (pos_of("G1"), PieceMoveType::Default),
                                (pos_of("H1"), PieceMoveType::Default),
                                (pos_of("D1"), PieceMoveType::Default),
                                (pos_of("C1"), PieceMoveType::Default),
                                (pos_of("B1"), PieceMoveType::Default),
                                (pos_of("A1"), PieceMoveType::Default),
                                (pos_of("E2"), PieceMoveType::Default),
                                (pos_of("E3"), PieceMoveType::Default),
                                (pos_of("E4"), PieceMoveType::Default),
                                (pos_of("E5"), PieceMoveType::Default),
                                (pos_of("E6"), PieceMoveType::Default),
                                (pos_of("E7"), PieceMoveType::Default),
                                (pos_of("E8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let color = Color::White;
        assert_eq!(
            legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [(pos_of("D4"), HashMap::new())].into()
        );
    }

    #[test]
    fn legal_moves_of_player_pawn_en_passant() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "   ♟    ",
                "   ♙♟   ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let bounds = mode.bounds;
        let history = vec![
            GameMove::default_of('♙', "D2", "D4"),
            GameMove::default_of('♟', "D7", "D6"),
            GameMove::default_of('♙', "D4", "D5"),
            GameMove::default_of('♟', "E7", "E5"),
        ];
        let players = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("E8"),
                            [
                                (pos_of("F8"), PieceMoveType::Default),
                                (pos_of("F7"), PieceMoveType::Default),
                                (pos_of("D7"), PieceMoveType::Default),
                                (pos_of("E7"), PieceMoveType::Default),
                                (pos_of("E8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (pos_of("D6"), [].into()),
                        (pos_of("E5"), [(pos_of("E4"), PieceMoveType::Default)].into()),
                    ]
                    .into(),
                },
            ),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let color = Color::White;
        assert_eq!(
            legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [
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
                (pos_of("D5"), [(pos_of("E6"), PieceMoveType::EnPassant)].into()),
            ]
            .into()
        );
    }

    #[test]
    fn legal_moves_of_player_in_check_rook() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "    ♜   ",
                "        ",
                "  ♗  ♗  ",
                "♖       ",
                "        ",
                "    ♔   ",
            ],
        );
        let bounds = mode.bounds;
        let history = Vec::new();
        let players = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("E8"),
                            [
                                (pos_of("F8"), PieceMoveType::Default),
                                (pos_of("F7"), PieceMoveType::Default),
                                (pos_of("D7"), PieceMoveType::Default),
                                (pos_of("E7"), PieceMoveType::Default),
                                (pos_of("E8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("E6"),
                            [
                                (pos_of("F6"), PieceMoveType::Default),
                                (pos_of("G6"), PieceMoveType::Default),
                                (pos_of("H6"), PieceMoveType::Default),
                                (pos_of("E5"), PieceMoveType::Default),
                                (pos_of("E4"), PieceMoveType::Default),
                                (pos_of("E3"), PieceMoveType::Default),
                                (pos_of("E2"), PieceMoveType::Default),
                                (pos_of("E1"), PieceMoveType::Default),
                                //
                                (pos_of("D6"), PieceMoveType::Default),
                                (pos_of("C6"), PieceMoveType::Default),
                                (pos_of("B6"), PieceMoveType::Default),
                                (pos_of("A6"), PieceMoveType::Default),
                                //
                                (pos_of("E7"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let color = Color::White;
        assert_eq!(
            legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [
                (
                    pos_of("C4"),
                    [
                        (pos_of("E6"), PieceMoveType::Default),
                        (pos_of("E2"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (
                    pos_of("F4"),
                    [
                        (pos_of("E5"), PieceMoveType::Default),
                        (pos_of("E3"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (pos_of("A3"), [(pos_of("E3"), PieceMoveType::Default)].into()),
                (
                    pos_of("E1"),
                    [
                        (pos_of("F2"), PieceMoveType::Default),
                        (pos_of("F1"), PieceMoveType::Default),
                        (pos_of("D1"), PieceMoveType::Default),
                        (pos_of("D2"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
            ]
            .into()
        );
    }

    #[test]
    fn legal_moves_of_player_in_double_check() {}
}
