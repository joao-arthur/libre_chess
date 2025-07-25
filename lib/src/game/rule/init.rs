use std::collections::{HashMap, HashSet};

use crate::{
    color::Color,
    game::{
        capture::GameCapture,
        game::{Game, GameHistory, empty_players},
        mode::GameMode,
        mov::GameMoveType,
        player::GamePlayer,
        rule::{
            legal_moves::legal_moves_of_player,
            move_piece::{self, move_piece},
            turn::evaluate_turn,
        },
        selection::Selection,
    },
    pos::Pos,
};

pub fn game_of_mode(mode: GameMode) -> Game {
    let board = mode.initial_board;
    let bounds = mode.bounds;
    let history = Vec::new();
    let players = [
        (
            Color::Black,
            GamePlayer {
                color: Color::Black,
                captures: Vec::new(),
                moves: legal_moves_of_player(
                    &board,
                    &bounds,
                    &history,
                    &empty_players(),
                    &Color::Black,
                ),
            },
        ),
        (
            Color::White,
            GamePlayer {
                color: Color::White,
                captures: Vec::new(),
                moves: legal_moves_of_player(
                    &board,
                    &bounds,
                    &history,
                    &empty_players(),
                    &Color::White,
                ),
            },
        ),
    ]
    .into();

    Game { board, bounds, players, history }
}

pub fn game_of_mode_and_history(mode: GameMode, base_history: GameHistory) -> Game {
    let bounds = mode.bounds.clone();
    let mut game = game_of_mode(mode);

    let history_it = base_history.iter();
    for game_move in history_it {
        move_piece(
            &mut game.board,
            &mut game.history,
            &mut game.players,
            &bounds,
            &Selection {
                selected_pos: Some(game_move.mov.from.clone()),
                selected_squares: HashSet::new(),
            },
            &game_move.mov.to,
        );
    }

    game
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        color::Color,
        game::{
            board::board_of_str,
            capture::GameCapture,
            game::{Game, GameBounds},
            mode::standard_chess,
            mov::{GameMove, PieceMoveType},
            player::GamePlayer,
        },
        piece::Piece,
        pos::Pos,
    };

    use super::{game_of_mode, game_of_mode_and_history};

    #[test]
    fn game_of_mode_standard_chess() {
        let game = game_of_mode(standard_chess());
        assert_eq!(
            game,
            Game {
                board: [
                    (Pos::of("A8"), Piece::of('♜')),
                    (Pos::of("B8"), Piece::of('♞')),
                    (Pos::of("C8"), Piece::of('♝')),
                    (Pos::of("D8"), Piece::of('♛')),
                    (Pos::of("E8"), Piece::of('♚')),
                    (Pos::of("F8"), Piece::of('♝')),
                    (Pos::of("G8"), Piece::of('♞')),
                    (Pos::of("H8"), Piece::of('♜')),
                    (Pos::of("A7"), Piece::of('♟')),
                    (Pos::of("B7"), Piece::of('♟')),
                    (Pos::of("C7"), Piece::of('♟')),
                    (Pos::of("D7"), Piece::of('♟')),
                    (Pos::of("E7"), Piece::of('♟')),
                    (Pos::of("F7"), Piece::of('♟')),
                    (Pos::of("G7"), Piece::of('♟')),
                    (Pos::of("H7"), Piece::of('♟')),
                    (Pos::of("A2"), Piece::of('♙')),
                    (Pos::of("B2"), Piece::of('♙')),
                    (Pos::of("C2"), Piece::of('♙')),
                    (Pos::of("D2"), Piece::of('♙')),
                    (Pos::of("E2"), Piece::of('♙')),
                    (Pos::of("F2"), Piece::of('♙')),
                    (Pos::of("G2"), Piece::of('♙')),
                    (Pos::of("H2"), Piece::of('♙')),
                    (Pos::of("A1"), Piece::of('♖')),
                    (Pos::of("B1"), Piece::of('♘')),
                    (Pos::of("C1"), Piece::of('♗')),
                    (Pos::of("D1"), Piece::of('♕')),
                    (Pos::of("E1"), Piece::of('♔')),
                    (Pos::of("F1"), Piece::of('♗')),
                    (Pos::of("G1"), Piece::of('♘')),
                    (Pos::of("H1"), Piece::of('♖')),
                ]
                .into(),
                bounds: GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 },
                players: [
                    (
                        Color::Black,
                        GamePlayer {
                            color: Color::Black,
                            captures: Vec::new(),
                            moves: [
                                (
                                    Pos::of("B8"),
                                    [
                                        (Pos::of("A6"), PieceMoveType::Default),
                                        (Pos::of("C6"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("G8"),
                                    [
                                        (Pos::of("F6"), PieceMoveType::Default),
                                        (Pos::of("H6"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("A7"),
                                    [
                                        (Pos::of("A6"), PieceMoveType::Default),
                                        (Pos::of("A5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("B7"),
                                    [
                                        (Pos::of("B6"), PieceMoveType::Default),
                                        (Pos::of("B5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("C7"),
                                    [
                                        (Pos::of("C6"), PieceMoveType::Default),
                                        (Pos::of("C5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("D7"),
                                    [
                                        (Pos::of("D6"), PieceMoveType::Default),
                                        (Pos::of("D5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("E7"),
                                    [
                                        (Pos::of("E6"), PieceMoveType::Default),
                                        (Pos::of("E5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("F7"),
                                    [
                                        (Pos::of("F6"), PieceMoveType::Default),
                                        (Pos::of("F5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("G7"),
                                    [
                                        (Pos::of("G6"), PieceMoveType::Default),
                                        (Pos::of("G5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("H7"),
                                    [
                                        (Pos::of("H6"), PieceMoveType::Default),
                                        (Pos::of("H5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                            ]
                            .into(),
                        }
                    ),
                    (
                        Color::White,
                        GamePlayer {
                            color: Color::White,
                            captures: Vec::new(),
                            moves: [
                                (
                                    Pos::of("B1"),
                                    [
                                        (Pos::of("A3"), PieceMoveType::Default),
                                        (Pos::of("C3"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("G1"),
                                    [
                                        (Pos::of("F3"), PieceMoveType::Default),
                                        (Pos::of("H3"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("A2"),
                                    [
                                        (Pos::of("A3"), PieceMoveType::Default),
                                        (Pos::of("A4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("B2"),
                                    [
                                        (Pos::of("B3"), PieceMoveType::Default),
                                        (Pos::of("B4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("C2"),
                                    [
                                        (Pos::of("C3"), PieceMoveType::Default),
                                        (Pos::of("C4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("D2"),
                                    [
                                        (Pos::of("D3"), PieceMoveType::Default),
                                        (Pos::of("D4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("E2"),
                                    [
                                        (Pos::of("E3"), PieceMoveType::Default),
                                        (Pos::of("E4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("F2"),
                                    [
                                        (Pos::of("F3"), PieceMoveType::Default),
                                        (Pos::of("F4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("G2"),
                                    [
                                        (Pos::of("G3"), PieceMoveType::Default),
                                        (Pos::of("G4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    Pos::of("H2"),
                                    [
                                        (Pos::of("H3"), PieceMoveType::Default),
                                        (Pos::of("H4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                            ]
                            .into(),
                        }
                    ),
                ]
                .into(),
                history: Vec::new(),
            }
        );
    }

    //#[test]
    //fn game_of_mode_and_history_standard_chess() {
    //    let mode = standard_chess();
    //    let history = vec![
    //        GameMove::default_of('♙', "A2", "A4"), //00
    //        GameMove::default_of('♟', "A7", "A5"), //01
    //        GameMove::default_of('♙', "B2", "B4"), //02
    //        GameMove::default_of('♟', "B7", "B5"), //03
    //        GameMove::default_of('♙', "C2", "C4"), //04
    //        GameMove::default_of('♟', "C7", "C5"), //05
    //        GameMove::default_of('♙', "D2", "D4"), //06
    //        GameMove::default_of('♟', "D7", "D5"), //07
    //        GameMove::default_of('♙', "E2", "E4"), //08
    //        GameMove::default_of('♟', "E7", "E5"), //09
    //        GameMove::default_of('♙', "F2", "F4"), //10
    //        GameMove::default_of('♟', "F7", "F5"), //11
    //        GameMove::default_of('♙', "G2", "G4"), //12
    //        GameMove::default_of('♟', "G7", "G5"), //13
    //        GameMove::default_of('♙', "H2", "H4"), //14
    //        GameMove::default_of('♟', "H7", "H5"), //15
    //        GameMove::capture_of('♙', "B4", "A5"), //16
    //        GameMove::capture_of('♟', "B5", "A4"), //17
    //        GameMove::capture_of('♙', "D4", "C5"), //18
    //        GameMove::capture_of('♟', "D5", "C4"), //19
    //        GameMove::capture_of('♙', "E4", "F5"), //20
    //        GameMove::capture_of('♟', "E5", "F4"), //21
    //        GameMove::capture_of('♙', "G4", "H5"), //22
    //        GameMove::capture_of('♟', "G5", "H4"), //23
    //        GameMove::capture_of('♖', "A1", "A4"), //24
    //        GameMove::capture_of('♜', "A8", "A5"), //25
    //        GameMove::capture_of('♖', "H1", "H4"), //26
    //        GameMove::capture_of('♜', "H8", "H5"), //27
    //        GameMove::capture_of('♗', "C1", "F4"), //28
    //        GameMove::capture_of('♝', "F8", "C5"), //29
    //        GameMove::capture_of('♗', "F1", "C4"), //30
    //        GameMove::capture_of('♝', "C8", "F5"), //31
    //        GameMove::capture_of('♗', "F4", "B8"), //32
    //        GameMove::capture_of('♝', "C5", "G1"), //33
    //        GameMove::capture_of('♗', "C4", "G8"), //34
    //        GameMove::capture_of('♝', "F5", "B1"), //35
    //        GameMove::default_of('♖', "A4", "A1"), //36
    //        GameMove::default_of('♜', "A5", "A8"), //37
    //        GameMove::default_of('♖', "H4", "H1"), //38
    //        GameMove::default_of('♜', "H5", "H8"), //39
    //        GameMove::capture_of('♖', "A1", "B1"), //40
    //        GameMove::capture_of('♜', "A8", "B8"), //41
    //        GameMove::capture_of('♖', "H1", "G1"), //42
    //        GameMove::capture_of('♜', "H8", "G8"), //43
    //        GameMove::default_of('♔', "E1", "E2"), //44
    //        GameMove::default_of('♚', "E8", "E7"), //45
    //        GameMove::capture_of('♖', "G1", "G8"), //46
    //        GameMove::capture_of('♜', "B8", "B1"), //47
    //        GameMove::capture_of('♖', "G8", "D8"), //48
    //        GameMove::capture_of('♜', "B1", "D1"), //49
    //    ];
    //    assert_eq!(
    //        game_of_mode_and_history(standard_chess(), history),
    //        Game {
    //            board: board_of_str(
    //                &mode.bounds,
    //                [
    //                    "   ♖    ",
    //                    "    ♚   ",
    //                    "        ",
    //                    "        ",
    //                    "        ",
    //                    "        ",
    //                    "    ♔   ",
    //                    "   ♜    ",
    //                ]
    //            ),
    //            bounds: GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 },
    //            players: [
    //                (
    //                    Color::Black,
    //                    GamePlayer {
    //                        color: Color::Black,
    //                        captures: vec![
    //                            GameCapture { at:16, piece: Piece::of('♟') },
    //                            GameCapture { at:18, piece: Piece::of('♟') },
    //                            GameCapture { at:20, piece: Piece::of('♟') },
    //                            GameCapture { at:22, piece: Piece::of('♟') },
    //                            GameCapture { at:24, piece: Piece::of('♟') },
    //                            GameCapture { at:26, piece: Piece::of('♟') },
    //                            GameCapture { at:28, piece: Piece::of('♟') },
    //                            GameCapture { at:30, piece: Piece::of('♟') },
    //                            //
    //                            GameCapture { at:, piece: Piece::of('♝') },
    //                            GameCapture { at:, piece: Piece::of('♝') },
    //                            GameCapture { at:, piece: Piece::of('♜') },
    //                            GameCapture { at:, piece: Piece::of('♜') },
    //                            GameCapture { at:, piece: Piece::of('♛') },
    //                        ],
    //                        moves: HashMap::new(),
    //                    },
    //                ),
    //                (
    //                    Color::White,
    //                    GamePlayer {
    //                        color: Color::White,
    //                        captures: Vec::new(),
    //                        moves: [
    //                            (
    //                                Pos::of("E2"),
    //                                [
    //                                    (Pos::of("F3"), PieceMoveType::Default),
    //                                    (Pos::of("F2"), PieceMoveType::Default),
    //                                    (Pos::of("D1"), PieceMoveType::Default),
    //                                    (Pos::of("E3"), PieceMoveType::Default),
    //                                ]
    //                                .into()
    //                            ),
    //                            (
    //                                Pos::of("D8"),
    //                                [
    //                                    (Pos::of("E8"), PieceMoveType::Default),
    //                                    (Pos::of("F8"), PieceMoveType::Default),
    //                                    (Pos::of("G8"), PieceMoveType::Default),
    //                                    (Pos::of("H8"), PieceMoveType::Default),
    //                                    //
    //                                    (Pos::of("D7"), PieceMoveType::Default),
    //                                    (Pos::of("D6"), PieceMoveType::Default),
    //                                    (Pos::of("D5"), PieceMoveType::Default),
    //                                    (Pos::of("D4"), PieceMoveType::Default),
    //                                    (Pos::of("D3"), PieceMoveType::Default),
    //                                    (Pos::of("D2"), PieceMoveType::Default),
    //                                    (Pos::of("D1"), PieceMoveType::Default),
    //                                    //
    //                                    (Pos::of("C8"), PieceMoveType::Default),
    //                                    (Pos::of("B8"), PieceMoveType::Default),
    //                                    (Pos::of("A8"), PieceMoveType::Default),
    //                                ]
    //                                .into()
    //                            ),
    //                        ]
    //                        .into(),
    //                    },
    //                ),
    //            ]
    //            .into(),
    //            history: vec![
    //                GameMove::default_of('♙', "A2", "A4"),
    //                GameMove::default_of('♟', "A7", "A5"),
    //                GameMove::default_of('♙', "B2", "B4"),
    //                GameMove::default_of('♟', "B7", "B5"),
    //                GameMove::default_of('♙', "C2", "C4"),
    //                GameMove::default_of('♟', "C7", "C5"),
    //                GameMove::default_of('♙', "D2", "D4"),
    //                GameMove::default_of('♟', "D7", "D5"),
    //                GameMove::default_of('♙', "E2", "E4"),
    //                GameMove::default_of('♟', "E7", "E5"),
    //                GameMove::default_of('♙', "F2", "F4"),
    //                GameMove::default_of('♟', "F7", "F5"),
    //                GameMove::default_of('♙', "G2", "G4"),
    //                GameMove::default_of('♟', "G7", "G5"),
    //                GameMove::default_of('♙', "H2", "H4"),
    //                GameMove::default_of('♟', "H7", "H5"),
    //                GameMove::capture_of('♙', "B4", "A5"),
    //                GameMove::capture_of('♟', "B5", "A4"),
    //                GameMove::capture_of('♙', "D4", "C5"),
    //                GameMove::capture_of('♟', "D5", "C4"),
    //                GameMove::capture_of('♙', "E4", "F5"),
    //                GameMove::capture_of('♟', "E5", "F4"),
    //                GameMove::capture_of('♙', "G4", "H5"),
    //                GameMove::capture_of('♟', "G5", "H4"),
    //                GameMove::capture_of('♖', "A1", "A4"),
    //                GameMove::capture_of('♜', "A8", "A5"),
    //                GameMove::capture_of('♖', "H1", "H4"),
    //                GameMove::capture_of('♜', "H8", "H5"),
    //                GameMove::capture_of('♗', "C1", "F4"),
    //                GameMove::capture_of('♝', "F8", "C5"),
    //                GameMove::capture_of('♗', "F1", "C4"),
    //                GameMove::capture_of('♝', "C8", "F5"),
    //                GameMove::capture_of('♗', "F4", "B8"),
    //                GameMove::capture_of('♝', "C5", "G1"),
    //                GameMove::capture_of('♗', "C4", "G8"),
    //                GameMove::capture_of('♝', "F5", "B1"),
    //                GameMove::default_of('♖', "A4", "A1"),
    //                GameMove::default_of('♜', "A5", "A8"),
    //                GameMove::default_of('♖', "H4", "H1"),
    //                GameMove::default_of('♜', "H5", "H8"),
    //                GameMove::capture_of('♖', "A1", "B1"),
    //                GameMove::capture_of('♜', "A8", "B8"),
    //                GameMove::capture_of('♖', "H1", "G1"),
    //                GameMove::capture_of('♜', "H8", "G8"),
    //                GameMove::default_of('♔', "E1", "E2"),
    //                GameMove::default_of('♚', "E8", "E7"),
    //                GameMove::capture_of('♖', "G1", "G8"),
    //                GameMove::capture_of('♜', "B8", "B1"),
    //                GameMove::capture_of('♖', "G8", "D8"),
    //                GameMove::capture_of('♜', "B1", "D1"),
    //            ],
    //        }
    //    );
    //}
}
