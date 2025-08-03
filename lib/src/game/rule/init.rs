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

    for game_move in base_history.iter() {
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
        pos::pos_of,
    };

    use super::{game_of_mode, game_of_mode_and_history};

    #[test]
    fn game_of_mode_standard_chess() {
        let game = game_of_mode(standard_chess());
        assert_eq!(
            game,
            Game {
                board: [
                    (pos_of("A8"), Piece::of('♜')),
                    (pos_of("B8"), Piece::of('♞')),
                    (pos_of("C8"), Piece::of('♝')),
                    (pos_of("D8"), Piece::of('♛')),
                    (pos_of("E8"), Piece::of('♚')),
                    (pos_of("F8"), Piece::of('♝')),
                    (pos_of("G8"), Piece::of('♞')),
                    (pos_of("H8"), Piece::of('♜')),
                    (pos_of("A7"), Piece::of('♟')),
                    (pos_of("B7"), Piece::of('♟')),
                    (pos_of("C7"), Piece::of('♟')),
                    (pos_of("D7"), Piece::of('♟')),
                    (pos_of("E7"), Piece::of('♟')),
                    (pos_of("F7"), Piece::of('♟')),
                    (pos_of("G7"), Piece::of('♟')),
                    (pos_of("H7"), Piece::of('♟')),
                    (pos_of("A2"), Piece::of('♙')),
                    (pos_of("B2"), Piece::of('♙')),
                    (pos_of("C2"), Piece::of('♙')),
                    (pos_of("D2"), Piece::of('♙')),
                    (pos_of("E2"), Piece::of('♙')),
                    (pos_of("F2"), Piece::of('♙')),
                    (pos_of("G2"), Piece::of('♙')),
                    (pos_of("H2"), Piece::of('♙')),
                    (pos_of("A1"), Piece::of('♖')),
                    (pos_of("B1"), Piece::of('♘')),
                    (pos_of("C1"), Piece::of('♗')),
                    (pos_of("D1"), Piece::of('♕')),
                    (pos_of("E1"), Piece::of('♔')),
                    (pos_of("F1"), Piece::of('♗')),
                    (pos_of("G1"), Piece::of('♘')),
                    (pos_of("H1"), Piece::of('♖')),
                ]
                .into(),
                bounds: GameBounds::of(0, 0, 7, 7),
                players: [
                    (
                        Color::Black,
                        GamePlayer {
                            color: Color::Black,
                            captures: Vec::new(),
                            moves: [
                                (
                                    pos_of("B8"),
                                    [
                                        (pos_of("A6"), PieceMoveType::Default),
                                        (pos_of("C6"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("G8"),
                                    [
                                        (pos_of("F6"), PieceMoveType::Default),
                                        (pos_of("H6"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("A7"),
                                    [
                                        (pos_of("A6"), PieceMoveType::Default),
                                        (pos_of("A5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("B7"),
                                    [
                                        (pos_of("B6"), PieceMoveType::Default),
                                        (pos_of("B5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("C7"),
                                    [
                                        (pos_of("C6"), PieceMoveType::Default),
                                        (pos_of("C5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("D7"),
                                    [
                                        (pos_of("D6"), PieceMoveType::Default),
                                        (pos_of("D5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("E7"),
                                    [
                                        (pos_of("E6"), PieceMoveType::Default),
                                        (pos_of("E5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("F7"),
                                    [
                                        (pos_of("F6"), PieceMoveType::Default),
                                        (pos_of("F5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("G7"),
                                    [
                                        (pos_of("G6"), PieceMoveType::Default),
                                        (pos_of("G5"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("H7"),
                                    [
                                        (pos_of("H6"), PieceMoveType::Default),
                                        (pos_of("H5"), PieceMoveType::Default)
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
                                    pos_of("B1"),
                                    [
                                        (pos_of("A3"), PieceMoveType::Default),
                                        (pos_of("C3"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("G1"),
                                    [
                                        (pos_of("F3"), PieceMoveType::Default),
                                        (pos_of("H3"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("A2"),
                                    [
                                        (pos_of("A3"), PieceMoveType::Default),
                                        (pos_of("A4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("B2"),
                                    [
                                        (pos_of("B3"), PieceMoveType::Default),
                                        (pos_of("B4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("C2"),
                                    [
                                        (pos_of("C3"), PieceMoveType::Default),
                                        (pos_of("C4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("D2"),
                                    [
                                        (pos_of("D3"), PieceMoveType::Default),
                                        (pos_of("D4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("E2"),
                                    [
                                        (pos_of("E3"), PieceMoveType::Default),
                                        (pos_of("E4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("F2"),
                                    [
                                        (pos_of("F3"), PieceMoveType::Default),
                                        (pos_of("F4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("G2"),
                                    [
                                        (pos_of("G3"), PieceMoveType::Default),
                                        (pos_of("G4"), PieceMoveType::Default)
                                    ]
                                    .into()
                                ),
                                (
                                    pos_of("H2"),
                                    [
                                        (pos_of("H3"), PieceMoveType::Default),
                                        (pos_of("H4"), PieceMoveType::Default)
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
    //            bounds: GameBounds::of(0, 0, 7, 7),
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
    //                                pos_of("E2"),
    //                                [
    //                                    (pos_of("F3"), PieceMoveType::Default),
    //                                    (pos_of("F2"), PieceMoveType::Default),
    //                                    (pos_of("D1"), PieceMoveType::Default),
    //                                    (pos_of("E3"), PieceMoveType::Default),
    //                                ]
    //                                .into()
    //                            ),
    //                            (
    //                                pos_of("D8"),
    //                                [
    //                                    (pos_of("E8"), PieceMoveType::Default),
    //                                    (pos_of("F8"), PieceMoveType::Default),
    //                                    (pos_of("G8"), PieceMoveType::Default),
    //                                    (pos_of("H8"), PieceMoveType::Default),
    //                                    //
    //                                    (pos_of("D7"), PieceMoveType::Default),
    //                                    (pos_of("D6"), PieceMoveType::Default),
    //                                    (pos_of("D5"), PieceMoveType::Default),
    //                                    (pos_of("D4"), PieceMoveType::Default),
    //                                    (pos_of("D3"), PieceMoveType::Default),
    //                                    (pos_of("D2"), PieceMoveType::Default),
    //                                    (pos_of("D1"), PieceMoveType::Default),
    //                                    //
    //                                    (pos_of("C8"), PieceMoveType::Default),
    //                                    (pos_of("B8"), PieceMoveType::Default),
    //                                    (pos_of("A8"), PieceMoveType::Default),
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
