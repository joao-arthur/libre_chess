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

    //for (from, piece_moves) in pseudo_legal_moves.clone() {
    //    for (to, move_type) in piece_moves {
    //        let mut temp_board = board.clone();
    //        if let Some(piece) = temp_board.remove(&from) {
    //            temp_board.insert(to.clone(), piece);
    //            if is_in_check(&temp_board, players, history) {
    //                pseudo_legal_moves.get_mut(&from).unwrap().remove(&to);
    //            }
    //        }
    //    }
    //}

    //let _: Option<()> = (|| {
    //    let (king_pos, _) =
    //        board.iter().find(|(_, piece)| piece.typ == PieceType::King && piece.color == turn)?;
    //    let king_moves = pseudo_legal_moves.get_mut(&king_pos)?;
    //    for (curr_color, curr_player) in players {
    //        if curr_color != color {
    //            let piece_moves_it = curr_player.moves.iter();
    //            for (_, curr_piece_moves) in piece_moves_it {
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
            mode::standard_chess,
            mov::{GameMove, PieceMoveType},
            player::GamePlayer,
        },
        piece::Piece,
        pos::Pos,
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let color = Color::White;
        assert_eq!(
            legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [
                (
                    Pos::of("A2"),
                    [
                        (Pos::of("A3"), PieceMoveType::Default),
                        (Pos::of("A4"), PieceMoveType::Default),
                    ]
                    .into()
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let color = Color::Black;
        assert_eq!(
            legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [
                (
                    Pos::of("A8"),
                    [
                        (Pos::of("B8"), PieceMoveType::Default),
                        (Pos::of("C8"), PieceMoveType::Default),
                        (Pos::of("D8"), PieceMoveType::Default),
                    ]
                    .into()
                ),
                (
                    Pos::of("H8"),
                    [
                        (Pos::of("G8"), PieceMoveType::Default),
                        (Pos::of("F8"), PieceMoveType::Default),
                    ]
                    .into()
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
                    .into()
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]
        .into();
        let color = Color::White;
        assert_eq!(
            legal_moves_of_player(&board, &bounds, &history, &players, &color),
            [(
                Pos::of("D4"),
                [
                    (Pos::of("E5"), PieceMoveType::Default),
                    (Pos::of("E4"), PieceMoveType::Default),
                    (Pos::of("E3"), PieceMoveType::Default),
                    (Pos::of("D3"), PieceMoveType::Default),
                    (Pos::of("C3"), PieceMoveType::Default),
                    (Pos::of("C4"), PieceMoveType::Default),
                    (Pos::of("C5"), PieceMoveType::Default),
                    (Pos::of("D5"), PieceMoveType::Default),
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
                        (Pos::of("B4"), [(Pos::of("B3"), PieceMoveType::Default)].into()),
                        (Pos::of("F4"), [(Pos::of("F3"), PieceMoveType::Default)].into()),
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
                Pos::of("D4"),
                [(Pos::of("D3"), PieceMoveType::Default), (Pos::of("D5"), PieceMoveType::Default)]
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
                            Pos::of("C8"),
                            [
                                (Pos::of("D8"), PieceMoveType::Default),
                                (Pos::of("E8"), PieceMoveType::Default),
                                (Pos::of("F8"), PieceMoveType::Default),
                                (Pos::of("G8"), PieceMoveType::Default),
                                (Pos::of("H8"), PieceMoveType::Default),
                                (Pos::of("C7"), PieceMoveType::Default),
                                (Pos::of("C6"), PieceMoveType::Default),
                                (Pos::of("C5"), PieceMoveType::Default),
                                (Pos::of("C4"), PieceMoveType::Default),
                                (Pos::of("C3"), PieceMoveType::Default),
                                (Pos::of("C2"), PieceMoveType::Default),
                                (Pos::of("C1"), PieceMoveType::Default),
                                (Pos::of("B8"), PieceMoveType::Default),
                                (Pos::of("A8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            Pos::of("H5"),
                            [
                                (Pos::of("H4"), PieceMoveType::Default),
                                (Pos::of("H3"), PieceMoveType::Default),
                                (Pos::of("H2"), PieceMoveType::Default),
                                (Pos::of("H1"), PieceMoveType::Default),
                                (Pos::of("G5"), PieceMoveType::Default),
                                (Pos::of("F5"), PieceMoveType::Default),
                                (Pos::of("E5"), PieceMoveType::Default),
                                (Pos::of("D5"), PieceMoveType::Default),
                                (Pos::of("C5"), PieceMoveType::Default),
                                (Pos::of("B5"), PieceMoveType::Default),
                                (Pos::of("A5"), PieceMoveType::Default),
                                (Pos::of("H6"), PieceMoveType::Default),
                                (Pos::of("H7"), PieceMoveType::Default),
                                (Pos::of("H8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            Pos::of("A3"),
                            [
                                (Pos::of("B3"), PieceMoveType::Default),
                                (Pos::of("C3"), PieceMoveType::Default),
                                (Pos::of("D3"), PieceMoveType::Default),
                                (Pos::of("E3"), PieceMoveType::Default),
                                (Pos::of("F3"), PieceMoveType::Default),
                                (Pos::of("G3"), PieceMoveType::Default),
                                (Pos::of("H3"), PieceMoveType::Default),
                                (Pos::of("A2"), PieceMoveType::Default),
                                (Pos::of("A1"), PieceMoveType::Default),
                                (Pos::of("A4"), PieceMoveType::Default),
                                (Pos::of("A5"), PieceMoveType::Default),
                                (Pos::of("A6"), PieceMoveType::Default),
                                (Pos::of("A7"), PieceMoveType::Default),
                                (Pos::of("A8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            Pos::of("E1"),
                            [
                                (Pos::of("F1"), PieceMoveType::Default),
                                (Pos::of("G1"), PieceMoveType::Default),
                                (Pos::of("H1"), PieceMoveType::Default),
                                (Pos::of("D1"), PieceMoveType::Default),
                                (Pos::of("C1"), PieceMoveType::Default),
                                (Pos::of("B1"), PieceMoveType::Default),
                                (Pos::of("A1"), PieceMoveType::Default),
                                (Pos::of("E2"), PieceMoveType::Default),
                                (Pos::of("E3"), PieceMoveType::Default),
                                (Pos::of("E4"), PieceMoveType::Default),
                                (Pos::of("E5"), PieceMoveType::Default),
                                (Pos::of("E6"), PieceMoveType::Default),
                                (Pos::of("E7"), PieceMoveType::Default),
                                (Pos::of("E8"), PieceMoveType::Default),
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
            [(Pos::of("D4"), HashMap::new())].into()
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
                            Pos::of("E8"),
                            [
                                (Pos::of("F8"), PieceMoveType::Default),
                                (Pos::of("F7"), PieceMoveType::Default),
                                (Pos::of("D7"), PieceMoveType::Default),
                                (Pos::of("E7"), PieceMoveType::Default),
                                (Pos::of("E8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (Pos::of("D6"), [].into()),
                        (Pos::of("E5"), [(Pos::of("E4"), PieceMoveType::Default)].into()),
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
                (Pos::of("D5"), [(Pos::of("E6"), PieceMoveType::EnPassant)].into()),
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
                            Pos::of("E8"),
                            [
                                (Pos::of("F8"), PieceMoveType::Default),
                                (Pos::of("F7"), PieceMoveType::Default),
                                (Pos::of("D7"), PieceMoveType::Default),
                                (Pos::of("E7"), PieceMoveType::Default),
                                (Pos::of("E8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            Pos::of("E6"),
                            [
                                (Pos::of("F6"), PieceMoveType::Default),
                                (Pos::of("G6"), PieceMoveType::Default),
                                (Pos::of("H6"), PieceMoveType::Default),
                                (Pos::of("E5"), PieceMoveType::Default),
                                (Pos::of("E4"), PieceMoveType::Default),
                                (Pos::of("E3"), PieceMoveType::Default),
                                (Pos::of("E2"), PieceMoveType::Default),
                                (Pos::of("E1"), PieceMoveType::Default),
                                //
                                (Pos::of("D6"), PieceMoveType::Default),
                                (Pos::of("C6"), PieceMoveType::Default),
                                (Pos::of("B6"), PieceMoveType::Default),
                                (Pos::of("A6"), PieceMoveType::Default),
                                //
                                (Pos::of("E7"), PieceMoveType::Default),
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
                    Pos::of("C4"),
                    [
                        (Pos::of("E6"), PieceMoveType::Default),
                        (Pos::of("E2"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (
                    Pos::of("F4"),
                    [
                        (Pos::of("E5"), PieceMoveType::Default),
                        (Pos::of("E3"), PieceMoveType::Default),
                    ]
                    .into(),
                ),
                (Pos::of("A3"), [(Pos::of("E3"), PieceMoveType::Default)].into()),
                (
                    Pos::of("E1"),
                    [
                        (Pos::of("F2"), PieceMoveType::Default),
                        (Pos::of("F1"), PieceMoveType::Default),
                        (Pos::of("D1"), PieceMoveType::Default),
                        (Pos::of("D2"), PieceMoveType::Default),
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
