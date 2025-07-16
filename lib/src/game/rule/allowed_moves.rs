use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        board::GameBoard,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{
            GameMove, GameMoveType,
            default::default_moves,
            special::{castling::castling_moves, en_passant::en_passant_moves},
        },
    },
    piece::PieceType,
    pos::Pos,
};

fn allowed_moves_of_piece(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    players: &GamePlayers,
    pos: &Pos,
) -> Vec<GameMove> {
    if let Some(piece) = board.get(pos) {
        match piece.typ {
            PieceType::Pawn => [
                default_moves(board, bounds, pos).into_iter().collect::<Vec<GameMove>>(),
                en_passant_moves(board, history, pos).into_iter().collect::<Vec<GameMove>>(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            PieceType::King => {
                let mut moves = [
                    default_moves(board, bounds, pos).into_iter().collect::<Vec<GameMove>>(),
                    castling_moves(board, history, players, pos)
                        .into_iter()
                        .collect::<Vec<GameMove>>(),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<GameMove>>();
                for (curr_color, curr_player) in players {
                    if curr_color != &piece.color {
                        let piece_moves_it = curr_player.moves.iter();
                        for (_, piece_moves) in piece_moves_it {
                            for menace_game_move in piece_moves {
                                if menace_game_move.typ == GameMoveType::Default
                                    || menace_game_move.typ == GameMoveType::Capture
                                    || menace_game_move.typ == GameMoveType::Menace
                                {
                                    moves.retain(|game_move| {
                                        game_move.mov.to != menace_game_move.mov.to
                                    });
                                }
                            }
                        }
                    }
                }
                moves
            }
            _ => default_moves(board, bounds, pos).into_iter().collect::<Vec<GameMove>>(),
        }
    } else {
        Vec::new()
    }
}

pub fn allowed_moves_of_player(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    players: &GamePlayers,
    color: &Color,
) -> HashMap<Pos, Vec<GameMove>> {
    // is in check?
    let mut result = HashMap::new();
    for (pos, piece) in board {
        if &piece.color == color {
            let moves = allowed_moves_of_piece(board, bounds, history, players, pos);
            result.insert(pos.clone(), moves);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        color::Color,
        game::{
            board::board_of_str, game::Game, mode::standard_chess, mov::GameMove,
            player::GamePlayer,
        },
        pos::Pos,
    };

    use super::allowed_moves_of_player;

    #[test]
    fn test_allowed_moves_of_player_standard_moves() {
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
        let color = Color::White;
        assert_eq!(
            allowed_moves_of_player(&board, &bounds, &history, &players, &color),
            HashMap::from([
                (
                    Pos::of("A2"),
                    vec![
                        GameMove::default_of('♙', "A2", "A3"),
                        GameMove::default_of('♙', "A2", "A4"),
                        GameMove::menace_of('♙', "A2", "B3"),
                    ]
                ),
                (
                    Pos::of("E1"),
                    vec![
                        GameMove::default_of('♔', "E1", "F2"),
                        GameMove::default_of('♔', "E1", "F1"),
                        GameMove::default_of('♔', "E1", "D1"),
                        GameMove::default_of('♔', "E1", "D2"),
                        GameMove::default_of('♔', "E1", "E2"),
                    ]
                ),
            ])
        )
    }

    #[test]
    fn test_allowed_moves_of_player_free_king() {
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
        let color = Color::White;
        assert_eq!(
            allowed_moves_of_player(&board, &bounds, &history, &players, &color),
            HashMap::from([(
                Pos::of("D4"),
                vec![
                    GameMove::default_of('♔', "D4", "E5"),
                    GameMove::default_of('♔', "D4", "E4"),
                    GameMove::default_of('♔', "D4", "E3"),
                    GameMove::default_of('♔', "D4", "D3"),
                    GameMove::default_of('♔', "D4", "C3"),
                    GameMove::default_of('♔', "D4", "C4"),
                    GameMove::default_of('♔', "D4", "C5"),
                    GameMove::default_of('♔', "D4", "D5"),
                ]
            )])
        )
    }

    #[test]
    fn test_allowed_moves_of_player_king_blocked_by_menace() {
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
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("D5"),
                            vec![
                                GameMove::menace_of('♟', "D5", "C4"),
                                GameMove::menace_of('♟', "D5", "E4"),
                            ],
                        ),
                        (
                            Pos::of("D6"),
                            vec![
                                GameMove::menace_of('♟', "D6", "C5"),
                                GameMove::menace_of('♟', "D6", "E5"),
                            ],
                        ),
                        (
                            Pos::of("B4"),
                            vec![
                                GameMove::default_of('♟', "B4", "B3"),
                                GameMove::menace_of('♟', "B4", "A3"),
                                GameMove::menace_of('♟', "B4", "C3"),
                            ],
                        ),
                        (
                            Pos::of("F4"),
                            vec![
                                GameMove::default_of('♟', "F4", "F3"),
                                GameMove::menace_of('♟', "F4", "E3"),
                                GameMove::menace_of('♟', "F4", "G3"),
                            ],
                        ),
                    ]),
                },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ]);
        let color = Color::White;
        assert_eq!(
            allowed_moves_of_player(&board, &bounds, &history, &players, &color),
            HashMap::from([(
                Pos::of("D4"),
                vec![GameMove::default_of('♔', "D4", "D3"), GameMove::capture_of('♔', "D4", "D5")]
            )])
        );
    }

    #[test]
    fn test_allowed_moves_of_player_king_blocked_by_default() {
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
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("C8"),
                            vec![
                                GameMove::default_of('♜', "C8", "D8"),
                                GameMove::default_of('♜', "C8", "E8"),
                                GameMove::default_of('♜', "C8", "F8"),
                                GameMove::default_of('♜', "C8", "G8"),
                                GameMove::default_of('♜', "C8", "H8"),
                                GameMove::default_of('♜', "C8", "C7"),
                                GameMove::default_of('♜', "C8", "C6"),
                                GameMove::default_of('♜', "C8", "C5"),
                                GameMove::default_of('♜', "C8", "C4"),
                                GameMove::default_of('♜', "C8", "C3"),
                                GameMove::default_of('♜', "C8", "C2"),
                                GameMove::default_of('♜', "C8", "C1"),
                                GameMove::default_of('♜', "C8", "B8"),
                                GameMove::default_of('♜', "C8", "A8"),
                            ],
                        ),
                        (
                            Pos::of("H5"),
                            vec![
                                GameMove::default_of('♜', "H5", "H4"),
                                GameMove::default_of('♜', "H5", "H3"),
                                GameMove::default_of('♜', "H5", "H2"),
                                GameMove::default_of('♜', "H5", "H1"),
                                GameMove::default_of('♜', "H5", "G5"),
                                GameMove::default_of('♜', "H5", "F5"),
                                GameMove::default_of('♜', "H5", "E5"),
                                GameMove::default_of('♜', "H5", "D5"),
                                GameMove::default_of('♜', "H5", "C5"),
                                GameMove::default_of('♜', "H5", "B5"),
                                GameMove::default_of('♜', "H5", "A5"),
                                GameMove::default_of('♜', "H5", "H6"),
                                GameMove::default_of('♜', "H5", "H7"),
                                GameMove::default_of('♜', "H5", "H8"),
                            ],
                        ),
                        (
                            Pos::of("A3"),
                            vec![
                                GameMove::default_of('♜', "A3", "B3"),
                                GameMove::default_of('♜', "A3", "C3"),
                                GameMove::default_of('♜', "A3", "D3"),
                                GameMove::default_of('♜', "A3", "E3"),
                                GameMove::default_of('♜', "A3", "F3"),
                                GameMove::default_of('♜', "A3", "G3"),
                                GameMove::default_of('♜', "A3", "H3"),
                                GameMove::default_of('♜', "A3", "A2"),
                                GameMove::default_of('♜', "A3", "A1"),
                                GameMove::default_of('♜', "A3", "A4"),
                                GameMove::default_of('♜', "A3", "A5"),
                                GameMove::default_of('♜', "A3", "A6"),
                                GameMove::default_of('♜', "A3", "A7"),
                                GameMove::default_of('♜', "A3", "A8"),
                            ],
                        ),
                        (
                            Pos::of("E1"),
                            vec![
                                GameMove::default_of('♜', "E1", "F1"),
                                GameMove::default_of('♜', "E1", "G1"),
                                GameMove::default_of('♜', "E1", "H1"),
                                GameMove::default_of('♜', "E1", "D1"),
                                GameMove::default_of('♜', "E1", "C1"),
                                GameMove::default_of('♜', "E1", "B1"),
                                GameMove::default_of('♜', "E1", "A1"),
                                GameMove::default_of('♜', "E1", "E2"),
                                GameMove::default_of('♜', "E1", "E3"),
                                GameMove::default_of('♜', "E1", "E4"),
                                GameMove::default_of('♜', "E1", "E5"),
                                GameMove::default_of('♜', "E1", "E6"),
                                GameMove::default_of('♜', "E1", "E7"),
                                GameMove::default_of('♜', "E1", "E8"),
                            ],
                        ),
                    ]),
                },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ]);
        let color = Color::White;
        assert_eq!(
            allowed_moves_of_player(&board, &bounds, &history, &players, &color),
            HashMap::from([(Pos::of("D4"), vec![])])
        );
    }

    #[test]
    fn test_allowed_moves_of_player_pawn_en_passant() {
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
        let history = Vec::from([
            GameMove::default_of('♙', "D2", "D4"),
            GameMove::default_of('♟', "D7", "D6"),
            GameMove::default_of('♙', "D4", "D5"),
            GameMove::default_of('♟', "E7", "E5"),
        ]);
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("E8"),
                            vec![
                                GameMove::default_of('♚', "E8", "F8"),
                                GameMove::default_of('♚', "E8", "F7"),
                                GameMove::default_of('♚', "E8", "D7"),
                                GameMove::default_of('♚', "E8", "E7"),
                                GameMove::default_of('♚', "E8", "E8"),
                            ],
                        ),
                        (
                            Pos::of("D6"),
                            vec![
                                GameMove::menace_of('♟', "D6", "C5"),
                                GameMove::menace_of('♟', "D6", "E5"),
                            ],
                        ),
                        (
                            Pos::of("E5"),
                            vec![
                                GameMove::menace_of('♟', "E5", "D4"),
                                GameMove::menace_of('♟', "E5", "F4"),
                                GameMove::default_of('♟', "E5", "E4"),
                            ],
                        ),
                    ]),
                },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ]);
        let color = Color::White;
        assert_eq!(
            allowed_moves_of_player(&board, &bounds, &history, &players, &color),
            HashMap::from([
                (
                    Pos::of("E1"),
                    vec![
                        GameMove::default_of('♔', "E1", "F2"),
                        GameMove::default_of('♔', "E1", "F1"),
                        GameMove::default_of('♔', "E1", "D1"),
                        GameMove::default_of('♔', "E1", "D2"),
                        GameMove::default_of('♔', "E1", "E2"),
                    ],
                ),
                (
                    Pos::of("D5"),
                    vec![
                        GameMove::menace_of('♙', "D5", "C6"),
                        GameMove::menace_of('♙', "D5", "E6"),
                        GameMove::en_passant_of('♙', "D5", "E6"),
                    ],
                ),
            ])
        );
    }
}
