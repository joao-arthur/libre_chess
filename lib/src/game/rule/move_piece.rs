use crate::{
    game::{
        board::GameBoard,
        capture::GameCapture,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{GameMove, GameMoveType, PieceMoveType},
        rule::{legal_moves::legal_moves_of_player, turn::evaluate_turn},
        selection::Selection,
    },
    mov::Mov,
    pos::Pos,
};

pub fn move_piece(
    board: &mut GameBoard,
    history: &mut GameHistory,
    players: &mut GamePlayers,
    bounds: &GameBounds,
    selection: &Selection,
    to: &Pos,
) {
    let _: Option<()> = (|| {
        let turn = evaluate_turn(history);
        let from = selection.selected_pos.clone()?;
        let selected_piece = board.get(&from)?;
        let selected_player = players.get_mut(&selected_piece.color)?;
        if selected_player.color == turn {
            let selected_piece_moves = selected_player.moves.get(&from).cloned()?;
            let game_move = selected_piece_moves.get(to)?;
            match *game_move {
                PieceMoveType::Default => {
                    let piece = board.remove(&from)?;
                    let maybe_captured_piece = board.insert(to.clone(), piece);
                    if let Some(captured_piece) = maybe_captured_piece {
                        selected_player
                            .captures
                            .push(GameCapture { piece: captured_piece, at: history.len() as u16 });
                        history.push(GameMove {
                            mov: Mov { from: from.clone(), to: to.clone(), piece },
                            typ: GameMoveType::Capture,
                        });
                        if let Some(affected_player) = players.get_mut(&captured_piece.color) {
                            affected_player.moves.remove(to);
                        }
                    } else {
                        history.push(GameMove {
                            mov: Mov { from: from.clone(), to: to.clone(), piece },
                            typ: GameMoveType::Default,
                        });
                    }
                }
                PieceMoveType::EnPassant => {
                    let pawn = board.remove(&from)?;
                    board.insert(to.clone(), pawn);
                    let capture_pos = Pos::of(from.row, to.col);
                    let captured_piece = board.remove(&capture_pos)?;
                    selected_player
                        .captures
                        .push(GameCapture { piece: captured_piece, at: history.len() as u16 });
                    history.push(GameMove {
                        mov: Mov { from: from.clone(), to: to.clone(), piece: pawn },
                        typ: GameMoveType::EnPassant,
                    });
                    if let Some(affected_player) = players.get_mut(&captured_piece.color) {
                        affected_player.moves.remove(&capture_pos);
                    }
                }
                PieceMoveType::ShortCastling => {
                    let king = board.remove(&from)?;
                    let new_king_pos = Pos::of(from.row, 6);
                    board.insert(new_king_pos, king);
                    let rook = board.remove(to)?;
                    let new_rook_pos = Pos::of(from.row, 5);
                    board.insert(new_rook_pos, rook);
                    history.push(GameMove {
                        mov: Mov { from: from.clone(), to: to.clone(), piece: king },
                        typ: GameMoveType::ShortCastling,
                    });
                }
                PieceMoveType::LongCastling => {
                    let king = board.remove(&from)?;
                    let new_king_pos = Pos::of(from.row, 2);
                    board.insert(new_king_pos, king);
                    let rook = board.remove(to)?;
                    let new_rook_pos = Pos::of(from.row, 3);
                    board.insert(new_rook_pos, rook);
                    history.push(GameMove {
                        mov: Mov { from: from.clone(), to: to.clone(), piece: king },
                        typ: GameMoveType::LongCastling,
                    });
                }
                PieceMoveType::PromotionToQueen => {
                    // board.insert(game_move.mov.to.clone(), promotion.piece);
                    // edit the pawn mov
                }
                PieceMoveType::PromotionToKnight => {
                    // board.insert(game_move.mov.to.clone(), promotion.piece);
                    // edit the pawn mov
                }
                PieceMoveType::PromotionToRook => {
                    // board.insert(game_move.mov.to.clone(), promotion.piece);
                    // edit the pawn mov
                }
                PieceMoveType::PromotionToBishop => {
                    // board.insert(game_move.mov.to.clone(), promotion.piece);
                    // edit the pawn mov
                }
            }
            let new_moves = legal_moves_of_player(board, bounds, history, &players.clone(), &turn);
            players.get_mut(&turn).unwrap().moves = new_moves;
        }
        None
    })();
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        color::Color,
        game::{
            board::board_of_str,
            capture::GameCapture,
            mode::standard_chess,
            mov::{GameMove, PieceMoveType},
            player::GamePlayer,
            selection::Selection,
        },
        piece::Piece,
        pos::pos_of,
    };

    use super::move_piece;

    #[test]
    fn move_piece_default_move() {
        let mode = standard_chess();
        let selection =
            Selection { selected_pos: Some(pos_of("A2")), selected_squares: HashSet::new() };

        let mut board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "♟       ",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙       ",
                "    ♔   ",
            ],
        );
        let mut history = Vec::new();
        let mut players = [
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
                                (pos_of("E7"), PieceMoveType::Default),
                                (pos_of("D7"), PieceMoveType::Default),
                                (pos_of("D8"), PieceMoveType::Default),
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
                    ]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [
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
                },
            ),
        ]
        .into();

        move_piece(&mut board, &mut history, &mut players, &mode.bounds, &selection, &pos_of("A4"));

        let board_after = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "♟       ",
                "        ",
                "        ",
                "♙       ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let history_after = vec![GameMove::default_of('♙', "A2", "A4")];
        let players_after = [
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
                                (pos_of("E7"), PieceMoveType::Default),
                                (pos_of("D7"), PieceMoveType::Default),
                                (pos_of("D8"), PieceMoveType::Default),
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
                    ]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [
                        (pos_of("A4"), [(pos_of("A5"), PieceMoveType::Default)].into()),
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
                },
            ),
        ]
        .into();

        assert_eq!(board, board_after);
        assert_eq!(history, history_after);
        assert_eq!(players, players_after);
    }

    #[test]
    fn move_piece_capture_move() {
        let mode = standard_chess();
        let selection =
            Selection { selected_pos: Some(pos_of("E5")), selected_squares: HashSet::new() };

        let mut board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "   ♟    ",
                "    ♙   ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let mut history = Vec::new();
        let mut players = [
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
                                (pos_of("E7"), PieceMoveType::Default),
                                (pos_of("D7"), PieceMoveType::Default),
                                (pos_of("D8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("D6"),
                            [
                                (pos_of("D5"), PieceMoveType::Default),
                                (pos_of("E5"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("E5"),
                            [
                                (pos_of("E6"), PieceMoveType::Default),
                                (pos_of("D6"), PieceMoveType::Default),
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
                },
            ),
        ]
        .into();

        move_piece(&mut board, &mut history, &mut players, &mode.bounds, &selection, &pos_of("D6"));

        let board_after = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "   ♙    ",
                "        ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let history_after = vec![GameMove::capture_of('♙', "E5", "D6")];
        let players_after = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [(
                        pos_of("E8"),
                        [
                            (pos_of("F8"), PieceMoveType::Default),
                            (pos_of("F7"), PieceMoveType::Default),
                            (pos_of("E7"), PieceMoveType::Default),
                            (pos_of("D7"), PieceMoveType::Default),
                            (pos_of("D8"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: vec![GameCapture { at: 0, piece: Piece::of('♟') }],
                    moves: [
                        (pos_of("D6"), [(pos_of("D7"), PieceMoveType::Default)].into()),
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
                },
            ),
        ]
        .into();

        assert_eq!(board, board_after);
        assert_eq!(history, history_after);
        assert_eq!(players, players_after);
    }

    #[test]
    fn move_piece_capture_en_passant_move() {
        let mode = standard_chess();
        let selection =
            Selection { selected_pos: Some(pos_of("E5")), selected_squares: HashSet::new() };

        let mut board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "        ",
                "   ♟♙   ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let mut history = Vec::new();
        let mut players = [
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
                                (pos_of("E7"), PieceMoveType::Default),
                                (pos_of("D7"), PieceMoveType::Default),
                                (pos_of("D8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (pos_of("D5"), [(pos_of("D5"), PieceMoveType::Default)].into()),
                    ]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("E5"),
                            [
                                (pos_of("E6"), PieceMoveType::Default),
                                (pos_of("D6"), PieceMoveType::EnPassant),
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
                },
            ),
        ]
        .into();

        move_piece(&mut board, &mut history, &mut players, &mode.bounds, &selection, &pos_of("D6"));

        let board_after = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "   ♙    ",
                "        ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let history_after = vec![GameMove::en_passant_of('♙', "E5", "D6")];
        let players_after = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [(
                        pos_of("E8"),
                        [
                            (pos_of("F8"), PieceMoveType::Default),
                            (pos_of("F7"), PieceMoveType::Default),
                            (pos_of("E7"), PieceMoveType::Default),
                            (pos_of("D7"), PieceMoveType::Default),
                            (pos_of("D8"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: vec![GameCapture { at: 0, piece: Piece::of('♟') }],
                    moves: [
                        (pos_of("D6"), [(pos_of("D7"), PieceMoveType::Default)].into()),
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
                },
            ),
        ]
        .into();

        assert_eq!(board, board_after);
        assert_eq!(history, history_after);
        assert_eq!(players, players_after);
    }

    #[test]
    fn move_piece_capture_short_castling_move() {
        let mode = standard_chess();
        let selection =
            Selection { selected_pos: Some(pos_of("E1")), selected_squares: HashSet::new() };

        let mut board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "     ♙ ♙",
                "    ♔  ♖",
            ],
        );
        let mut history = Vec::new();
        let mut players = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [(
                        pos_of("E8"),
                        [
                            (pos_of("F8"), PieceMoveType::Default),
                            (pos_of("F7"), PieceMoveType::Default),
                            (pos_of("E7"), PieceMoveType::Default),
                            (pos_of("D7"), PieceMoveType::Default),
                            (pos_of("D8"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("F2"),
                            [
                                (pos_of("F3"), PieceMoveType::Default),
                                (pos_of("F4"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("H2"),
                            [
                                (pos_of("H3"), PieceMoveType::Default),
                                (pos_of("H4"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("E1"),
                            [
                                (pos_of("F1"), PieceMoveType::Default),
                                (pos_of("D1"), PieceMoveType::Default),
                                (pos_of("D2"), PieceMoveType::Default),
                                (pos_of("E2"), PieceMoveType::Default),
                                (pos_of("H1"), PieceMoveType::ShortCastling),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("H1"),
                            [
                                (pos_of("G1"), PieceMoveType::Default),
                                (pos_of("F1"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
        ]
        .into();

        move_piece(&mut board, &mut history, &mut players, &mode.bounds, &selection, &pos_of("H1"));

        let board_after = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "     ♙ ♙",
                "     ♖♔ ",
            ],
        );
        let history_after = vec![GameMove::short_castling_of('♔', "E1", "H1")];
        let players_after = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [(
                        pos_of("E8"),
                        [
                            (pos_of("F8"), PieceMoveType::Default),
                            (pos_of("F7"), PieceMoveType::Default),
                            (pos_of("E7"), PieceMoveType::Default),
                            (pos_of("D7"), PieceMoveType::Default),
                            (pos_of("D8"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("F2"),
                            [
                                (pos_of("F3"), PieceMoveType::Default),
                                (pos_of("F4"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("H2"),
                            [
                                (pos_of("H3"), PieceMoveType::Default),
                                (pos_of("H4"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("G1"),
                            [
                                (pos_of("H1"), PieceMoveType::Default),
                                (pos_of("G2"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("F1"),
                            [
                                (pos_of("E1"), PieceMoveType::Default),
                                (pos_of("D1"), PieceMoveType::Default),
                                (pos_of("C1"), PieceMoveType::Default),
                                (pos_of("B1"), PieceMoveType::Default),
                                (pos_of("A1"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
        ]
        .into();

        assert_eq!(board, board_after);
        assert_eq!(history, history_after);
        assert_eq!(players, players_after);
    }

    #[test]
    fn move_piece_capture_long_castling_move() {
        let mode = standard_chess();
        let selection =
            Selection { selected_pos: Some(pos_of("E1")), selected_squares: HashSet::new() };

        let mut board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙  ♙    ",
                "♖   ♔   ",
            ],
        );
        let mut history = Vec::new();
        let mut players = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [(
                        pos_of("E8"),
                        [
                            (pos_of("F8"), PieceMoveType::Default),
                            (pos_of("F7"), PieceMoveType::Default),
                            (pos_of("E7"), PieceMoveType::Default),
                            (pos_of("D7"), PieceMoveType::Default),
                            (pos_of("D8"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("A2"),
                            [
                                (pos_of("A3"), PieceMoveType::Default),
                                (pos_of("A4"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("D2"),
                            [
                                (pos_of("D3"), PieceMoveType::Default),
                                (pos_of("D4"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("A1"),
                            [
                                (pos_of("B1"), PieceMoveType::Default),
                                (pos_of("C1"), PieceMoveType::Default),
                                (pos_of("D1"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("E1"),
                            [
                                (pos_of("F2"), PieceMoveType::Default),
                                (pos_of("F1"), PieceMoveType::Default),
                                (pos_of("D1"), PieceMoveType::Default),
                                (pos_of("E2"), PieceMoveType::Default),
                                (pos_of("A1"), PieceMoveType::LongCastling),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
        ]
        .into();

        move_piece(&mut board, &mut history, &mut players, &mode.bounds, &selection, &pos_of("A1"));

        let board_after = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙  ♙    ",
                "  ♔♖    ",
            ],
        );
        let history_after = vec![GameMove::long_castling_of('♔', "E1", "A1")];
        let players_after = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [(
                        pos_of("E8"),
                        [
                            (pos_of("F8"), PieceMoveType::Default),
                            (pos_of("F7"), PieceMoveType::Default),
                            (pos_of("E7"), PieceMoveType::Default),
                            (pos_of("D7"), PieceMoveType::Default),
                            (pos_of("D8"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("A2"),
                            [
                                (pos_of("A3"), PieceMoveType::Default),
                                (pos_of("A4"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("D2"),
                            [
                                (pos_of("D3"), PieceMoveType::Default),
                                (pos_of("D4"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("C1"),
                            [
                                (pos_of("B1"), PieceMoveType::Default),
                                (pos_of("B2"), PieceMoveType::Default),
                                (pos_of("C2"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("D1"),
                            [
                                (pos_of("E1"), PieceMoveType::Default),
                                (pos_of("F1"), PieceMoveType::Default),
                                (pos_of("G1"), PieceMoveType::Default),
                                (pos_of("H1"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
        ]
        .into();

        assert_eq!(board, board_after);
        assert_eq!(history, history_after);
        assert_eq!(players, players_after);
    }
}
