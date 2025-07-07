use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        Game,
        game::GameHistory,
        mode::GameMode,
        player::GamePlayer,
        rule::{allowed_moves::allowed_moves_of_player, turn::evaluate_turn},
    },
};

pub fn game_of_mode(mode: GameMode) -> Game {
    let board = mode.initial_board;
    let bounds = mode.bounds;
    let history = Vec::new();
    let players = HashMap::from([
        (
            Color::Black,
            GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
        ),
        (
            Color::White,
            GamePlayer {
                color: Color::White,
                captures: Vec::new(),
                moves: allowed_moves_of_player(&board, &bounds, &history, &Color::White),
            },
        ),
    ]);
    Game { board, bounds, players, history }
}

pub fn game_of_mode_and_history(mode: GameMode, history: GameHistory) -> Game {
    let mut board = mode.initial_board;
    let bounds = mode.bounds;
    let history_iter = history.iter();
    for movement in history_iter {
        if let Some(piece) = board.remove(&movement.from) {
            board.insert(movement.to.clone(), piece);
        }
    }
    let turn = evaluate_turn(&history);
    let players = HashMap::from([
        (
            Color::Black,
            GamePlayer {
                color: Color::Black,
                captures: Vec::new(),
                moves: if turn == Color::Black {
                    allowed_moves_of_player(&board, &bounds, &history, &Color::Black)
                } else {
                    HashMap::new()
                },
            },
        ),
        (
            Color::White,
            GamePlayer {
                color: Color::White,
                captures: Vec::new(),
                moves: if turn == Color::White {
                    allowed_moves_of_player(&board, &bounds, &history, &Color::White)
                } else {
                    HashMap::new()
                },
            },
        ),
    ]);
    Game { bounds: mode.bounds, board, players, history }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        color::Color,
        game::{
            Game,
            board::{board_of_str, board_to_string},
            game::GameBounds,
            mode::standard_chess,
            mov::{CaptureMov, DefaultMov, GameMov, MenaceMov},
            player::GamePlayer,
        },
        mov::Mov,
        pos::Pos,
    };

    use super::{game_of_mode, game_of_mode_and_history};

    #[test]
    fn game_of_mode_standard_chess() {
        let mode = standard_chess();
        let game = game_of_mode(standard_chess());
        assert_eq!(
            board_to_string(&mode.bounds, &game.board),
            "".to_owned()
                + "♜♞♝♛♚♝♞♜\n"
                + "♟♟♟♟♟♟♟♟\n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "♙♙♙♙♙♙♙♙\n"
                + "♖♘♗♕♔♗♘♖\n"
        );
        assert_eq!(game.bounds, GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 });
        assert_eq!(game.history, Vec::new());
        assert_eq!(
            game.players,
            HashMap::from([
                (
                    Color::Black,
                    GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
                ),
                (
                    Color::White,
                    GamePlayer {
                        color: Color::White,
                        captures: Vec::new(),
                        moves: HashMap::from([
                            (
                                Pos::of_str("A2"),
                                vec![
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "A2", "A3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "A2", "A4"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "A2", "B3"))),
                                ]
                            ),
                            (
                                Pos::of_str("B2"),
                                vec![
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "B2", "B3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "B2", "B4"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "B2", "A3"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "B2", "D3"))),
                                ]
                            ),
                            (
                                Pos::of_str("C2"),
                                vec![
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "C2", "C3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "C2", "C4"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "C2", "B3"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "C2", "E3"))),
                                ]
                            ),
                            (
                                Pos::of_str("D2"),
                                vec![
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "D2", "D3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "D2", "D4"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "D2", "C3"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "D2", "E3"))),
                                ]
                            ),
                            (
                                Pos::of_str("E2"),
                                vec![
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "E2", "E3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "E2", "E4"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "E2", "D3"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "E2", "F3"))),
                                ]
                            ),
                            (
                                Pos::of_str("F2"),
                                vec![
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "F2", "F3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "F2", "F4"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "F2", "E3"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "F2", "G3"))),
                                ]
                            ),
                            (
                                Pos::of_str("G2"),
                                vec![
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "G2", "G3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "G2", "G4"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "G2", "F3"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "G2", "H3"))),
                                ]
                            ),
                            (
                                Pos::of_str("H2"),
                                vec![
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "H2", "H3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♙', "H2", "H4"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♙', "H2", "G3"))),
                                ]
                            ),
                            (
                                Pos::of_str("A1"),
                                vec![
                                    GameMov::from(MenaceMov::from(Mov::of('♖', "A1", "B1"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♖', "A1", "A2"))),
                                ]
                            ),
                            (
                                Pos::of_str("B1"),
                                vec![
                                    GameMov::from(DefaultMov::from(Mov::of('♘', "B1", "C3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♘', "B1", "A3"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♘', "B1", "D2"))),
                                ]
                            ),
                            (
                                Pos::of_str("C1"),
                                vec![
                                    GameMov::from(MenaceMov::from(Mov::of('♗', "C1", "D2"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♗', "C1", "B2"))),
                                ]
                            ),
                            (
                                Pos::of_str("D1"),
                                vec![
                                    GameMov::from(MenaceMov::from(Mov::of('♕', "D1", "E2"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♕', "D1", "E1"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♕', "D1", "C1"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♕', "D1", "C2"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♕', "D1", "D2"))),
                                ]
                            ),
                            (
                                Pos::of_str("E1"),
                                vec![
                                    GameMov::from(MenaceMov::from(Mov::of('♔', "E1", "F2"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♔', "E1", "F1"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♔', "E1", "D1"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♔', "E1", "D2"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♔', "E1", "E2"))),
                                ]
                            ),
                            (
                                Pos::of_str("F1"),
                                vec![
                                    GameMov::from(MenaceMov::from(Mov::of('♗', "F1", "G2"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♗', "F1", "E2"))),
                                ]
                            ),
                            (
                                Pos::of_str("G1"),
                                vec![
                                    GameMov::from(DefaultMov::from(Mov::of('♘', "G1", "H3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♘', "G1", "F3"))),
                                    GameMov::from(DefaultMov::from(Mov::of('♘', "G1", "E2"))),
                                ]
                            ),
                            (
                                Pos::of_str("H1"),
                                vec![
                                    GameMov::from(MenaceMov::from(Mov::of('♖', "H1", "G1"))),
                                    GameMov::from(MenaceMov::from(Mov::of('♖', "H1", "H2"))),
                                ]
                            ),
                        ]),
                    },
                ),
            ])
        );
    }

    #[test]
    fn game_of_mode_and_history_standard_chess() {
        let mode = standard_chess();
        let history = vec![
            Mov::of('♙', "A2", "A4"),
            Mov::of('♟', "A7", "A5"),
            Mov::of('♙', "B2", "B4"),
            Mov::of('♟', "B7", "B5"),
            Mov::of('♙', "C2", "C4"),
            Mov::of('♟', "C7", "C5"),
            Mov::of('♙', "D2", "D4"),
            Mov::of('♟', "D7", "D5"),
            Mov::of('♙', "E2", "E4"),
            Mov::of('♟', "E7", "E5"),
            Mov::of('♙', "F2", "F4"),
            Mov::of('♟', "F7", "F5"),
            Mov::of('♙', "G2", "G4"),
            Mov::of('♟', "G7", "G5"),
            Mov::of('♙', "H2", "H4"),
            Mov::of('♟', "H7", "H5"),
            //
            Mov::of('♙', "B4", "A5"),
            Mov::of('♟', "B5", "A4"),
            Mov::of('♙', "D4", "C5"),
            Mov::of('♟', "D5", "C4"),
            Mov::of('♙', "E4", "F5"),
            Mov::of('♟', "E5", "F4"),
            Mov::of('♙', "G4", "H5"),
            Mov::of('♟', "G5", "H4"),
            //
            Mov::of('♖', "A1", "A4"),
            Mov::of('♜', "A8", "A5"),
            Mov::of('♖', "H1", "H4"),
            Mov::of('♜', "H8", "H5"),
            //
            Mov::of('♗', "C1", "F4"),
            Mov::of('♝', "F8", "C5"),
            Mov::of('♗', "F1", "C4"),
            Mov::of('♝', "C8", "F5"),
            Mov::of('♗', "F4", "B8"),
            Mov::of('♝', "C5", "G1"),
            Mov::of('♗', "C4", "G8"),
            Mov::of('♝', "F5", "B1"),
            //
            Mov::of('♖', "A4", "A1"),
            Mov::of('♜', "A5", "A8"),
            Mov::of('♖', "H4", "H1"),
            Mov::of('♜', "H5", "H8"),
            Mov::of('♖', "A1", "B1"),
            Mov::of('♜', "A8", "B8"),
            Mov::of('♖', "H1", "G1"),
            Mov::of('♜', "H8", "G8"),
            //
            Mov::of('♔', "E1", "E2"),
            Mov::of('♚', "E8", "E7"),
            //
            Mov::of('♖', "G1", "G8"),
            Mov::of('♜', "B8", "B1"),
            Mov::of('♖', "G8", "D8"),
            Mov::of('♜', "B1", "D1"),
        ];
        assert_eq!(
            game_of_mode_and_history(standard_chess(), history),
            Game {
                board: board_of_str(
                    &mode.bounds,
                    [
                        "   ♖    ",
                        "    ♚   ",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "    ♔   ",
                        "   ♜    ",
                    ]
                ),
                bounds: GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 },
                players: HashMap::from([
                    (
                        Color::Black,
                        GamePlayer {
                            color: Color::Black,
                            captures: Vec::new(),
                            moves: HashMap::new(),
                        },
                    ),
                    (
                        Color::White,
                        GamePlayer {
                            color: Color::White,
                            captures: Vec::new(),
                            moves: HashMap::from([
                                (
                                    Pos::of_str("E2"),
                                    vec![
                                        GameMov::from(DefaultMov::from(Mov::of('♔', "E2", "F3"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♔', "E2", "F2"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♔', "E2", "F1"))), //////
                                        GameMov::from(DefaultMov::from(Mov::of('♔', "E2", "E1"))), //////
                                        GameMov::from(CaptureMov::from(Mov::of('♔', "E2", "D1"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♔', "E2", "D2"))), //////
                                        GameMov::from(DefaultMov::from(Mov::of('♔', "E2", "D3"))), //////
                                        GameMov::from(DefaultMov::from(Mov::of('♔', "E2", "E3"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("D8"),
                                    vec![
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "E8"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "F8"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "G8"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "H8"))),
                                        //
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "D7"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "D6"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "D5"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "D4"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "D3"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "D2"))),
                                        GameMov::from(CaptureMov::from(Mov::of('♖', "D8", "D1"))),
                                        //
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "C8"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "B8"))),
                                        GameMov::from(DefaultMov::from(Mov::of('♖', "D8", "A8"))),
                                    ]
                                ),
                            ]),
                        },
                    ),
                ]),
                history: vec![
                    Mov::of('♙', "A2", "A4"),
                    Mov::of('♟', "A7", "A5"),
                    Mov::of('♙', "B2", "B4"),
                    Mov::of('♟', "B7", "B5"),
                    Mov::of('♙', "C2", "C4"),
                    Mov::of('♟', "C7", "C5"),
                    Mov::of('♙', "D2", "D4"),
                    Mov::of('♟', "D7", "D5"),
                    Mov::of('♙', "E2", "E4"),
                    Mov::of('♟', "E7", "E5"),
                    Mov::of('♙', "F2", "F4"),
                    Mov::of('♟', "F7", "F5"),
                    Mov::of('♙', "G2", "G4"),
                    Mov::of('♟', "G7", "G5"),
                    Mov::of('♙', "H2", "H4"),
                    Mov::of('♟', "H7", "H5"),
                    //
                    Mov::of('♙', "B4", "A5"),
                    Mov::of('♟', "B5", "A4"),
                    Mov::of('♙', "D4", "C5"),
                    Mov::of('♟', "D5", "C4"),
                    Mov::of('♙', "E4", "F5"),
                    Mov::of('♟', "E5", "F4"),
                    Mov::of('♙', "G4", "H5"),
                    Mov::of('♟', "G5", "H4"),
                    //
                    Mov::of('♖', "A1", "A4"),
                    Mov::of('♜', "A8", "A5"),
                    Mov::of('♖', "H1", "H4"),
                    Mov::of('♜', "H8", "H5"),
                    //
                    Mov::of('♗', "C1", "F4"),
                    Mov::of('♝', "F8", "C5"),
                    Mov::of('♗', "F1", "C4"),
                    Mov::of('♝', "C8", "F5"),
                    //
                    Mov::of('♗', "F4", "B8"),
                    Mov::of('♝', "C5", "G1"),
                    Mov::of('♗', "C4", "G8"),
                    Mov::of('♝', "F5", "B1"),
                    //
                    Mov::of('♖', "A4", "A1"),
                    Mov::of('♜', "A5", "A8"),
                    Mov::of('♖', "H4", "H1"),
                    Mov::of('♜', "H5", "H8"),
                    //
                    Mov::of('♖', "A1", "B1"),
                    Mov::of('♜', "A8", "B8"),
                    Mov::of('♖', "H1", "G1"),
                    Mov::of('♜', "H8", "G8"),
                    //
                    Mov::of('♔', "E1", "E2"),
                    Mov::of('♚', "E8", "E7"),
                    //
                    Mov::of('♖', "G1", "G8"),
                    Mov::of('♜', "B8", "B1"),
                    Mov::of('♖', "G8", "D8"),
                    Mov::of('♜', "B1", "D1"),
                ],
            }
        );
    }
}
