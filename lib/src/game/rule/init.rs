use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        Game,
        game::GameHistory,
        mode::GameMode,
        player::GamePlayer,
        rule::{allowed_movements::allowed_movements_of_player, turn::evaluate_turn},
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
                moves: allowed_movements_of_player(&board, &bounds, &history, &Color::White),
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
                    allowed_movements_of_player(&board, &bounds, &history, &Color::Black)
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
                    allowed_movements_of_player(&board, &bounds, &history, &Color::White)
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
            game::GameBounds, mode::standard_chess,
            movement::movement::{CaptureMove, DefaultMove, GameMove, MenaceMove},
            player::GamePlayer,
        },
        movement::Movement,
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
                                Pos::of_str("A2"),
                                vec![
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "A2", "A3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "A2", "A4"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "A2", "B3"))),
                                ]
                            ),
                            (
                                Pos::of_str("B2"),
                                vec![
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "B2", "B3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "B2", "B4"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "B2", "A3"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "B2", "D3"))),
                                ] 
                            ),
                            (
                                Pos::of_str("C2"),
                                vec![
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "C2", "C3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "C2", "C4"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "C2", "B3"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "C2", "E3"))),
                                ]
                            ),
                            (
                                Pos::of_str("D2"),
                                vec![
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "D2", "D3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "D2", "D4"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "D2", "C3"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "D2", "E3"))),
                                ]
                            ),
                            (
                                Pos::of_str("E2"),
                                vec![
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "E2", "E3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "E2", "E4"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "E2", "D3"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "E2", "F3"))),
                                ]
                            ),
                            (
                                Pos::of_str("F2"),
                                vec![
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "F2", "F3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "F2", "F4"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "F2", "E3"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "F2", "G3"))),
                                ]
                            ),
                            (
                                Pos::of_str("G2"),
                                vec![
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "G2", "G3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "G2", "G4"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "G2", "F3"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "G2", "H3"))),
                                ]
                            ),
                            (
                                Pos::of_str("H2"),
                                vec![
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "H2", "H3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♙', "H2", "H4"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♙', "H2", "G3"))),
                                ]
                            ),
                            (
                                Pos::of_str("A1"),
                                vec![
                                    GameMove::from(MenaceMove::from(Movement::of('♖', "A1", "B1"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♖', "A1", "A2"))),
                                ]
                            ),
                            (
                                Pos::of_str("B1"),
                                vec![
                                    GameMove::from(DefaultMove::from(Movement::of('♘', "B1", "C3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♘', "B1", "A3"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♘', "B1", "D2"))),
                                ]
                            ),
                            (
                                Pos::of_str("C1"),
                                vec![
                                    GameMove::from(MenaceMove::from(Movement::of('♗', "C1", "D2"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♗', "C1", "B2"))),
                                ]
                            ),
                            (
                                Pos::of_str("D1"),
                                vec![
                                    GameMove::from(MenaceMove::from(Movement::of('♕', "D1", "E2"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♕', "D1", "E1"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♕', "D1", "C1"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♕', "D1", "C2"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♕', "D1", "D2"))),
                                ]
                            ),
                            (
                                Pos::of_str("E1"),
                                vec![
                                    GameMove::from(MenaceMove::from(Movement::of('♔', "E1", "F2"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♔', "E1", "F1"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♔', "E1", "D1"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♔', "E1", "D2"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♔', "E1", "E2"))),
                                ]
                            ),
                            (
                                Pos::of_str("F1"),
                                vec![
                                    GameMove::from(MenaceMove::from(Movement::of('♗', "F1", "G2"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♗', "F1", "E2"))),
                                ]
                            ),
                            (
                                Pos::of_str("G1"),
                                vec![
                                    GameMove::from(DefaultMove::from(Movement::of('♘', "G1", "H3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♘', "G1", "F3"))),
                                    GameMove::from(DefaultMove::from(Movement::of('♘', "G1", "E2"))),
                                ]
                            ),
                            (
                                Pos::of_str("H1"),
                                vec![
                                    GameMove::from(MenaceMove::from(Movement::of('♖', "H1", "G1"))),
                                    GameMove::from(MenaceMove::from(Movement::of('♖', "H1", "H2"))),
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
            Movement::of('♙', "A2", "A4"),
            Movement::of('♟', "A7", "A5"),
            Movement::of('♙', "B2", "B4"),
            Movement::of('♟', "B7", "B5"),
            Movement::of('♙', "C2", "C4"),
            Movement::of('♟', "C7", "C5"),
            Movement::of('♙', "D2", "D4"),
            Movement::of('♟', "D7", "D5"),
            Movement::of('♙', "E2", "E4"),
            Movement::of('♟', "E7", "E5"),
            Movement::of('♙', "F2", "F4"),
            Movement::of('♟', "F7", "F5"),
            Movement::of('♙', "G2", "G4"),
            Movement::of('♟', "G7", "G5"),
            Movement::of('♙', "H2", "H4"),
            Movement::of('♟', "H7", "H5"),
            //
            Movement::of('♙', "B4", "A5"),
            Movement::of('♟', "B5", "A4"),
            Movement::of('♙', "D4", "C5"),
            Movement::of('♟', "D5", "C4"),
            Movement::of('♙', "E4", "F5"),
            Movement::of('♟', "E5", "F4"),
            Movement::of('♙', "G4", "H5"),
            Movement::of('♟', "G5", "H4"),
            //
            Movement::of('♖', "A1", "A4"),
            Movement::of('♜', "A8", "A5"),
            Movement::of('♖', "H1", "H4"),
            Movement::of('♜', "H8", "H5"),
            //
            Movement::of('♗', "C1", "F4"),
            Movement::of('♝', "F8", "C5"),
            Movement::of('♗', "F1", "C4"),
            Movement::of('♝', "C8", "F5"),
            Movement::of('♗', "F4", "B8"),
            Movement::of('♝', "C5", "G1"),
            Movement::of('♗', "C4", "G8"),
            Movement::of('♝', "F5", "B1"),
            //
            Movement::of('♖', "A4", "A1"),
            Movement::of('♜', "A5", "A8"),
            Movement::of('♖', "H4", "H1"),
            Movement::of('♜', "H5", "H8"),
            Movement::of('♖', "A1", "B1"),
            Movement::of('♜', "A8", "B8"),
            Movement::of('♖', "H1", "G1"),
            Movement::of('♜', "H8", "G8"),
            //
            Movement::of('♔', "E1", "E2"),
            Movement::of('♚', "E8", "E7"),
            //
            Movement::of('♖', "G1", "G8"),
            Movement::of('♜', "B8", "B1"),
            Movement::of('♖', "G8", "D8"),
            Movement::of('♜', "B1", "D1"),
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
                                        GameMove::from(DefaultMove::from(Movement::of('♔', "E2", "F3"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♔', "E2", "F2"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♔', "E2", "F1"))), //////
                                        GameMove::from(DefaultMove::from(Movement::of('♔', "E2", "E1"))), //////
                                        GameMove::from(CaptureMove::from(Movement::of('♔', "E2", "D1"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♔', "E2", "D2"))), //////
                                        GameMove::from(DefaultMove::from(Movement::of('♔', "E2", "D3"))), //////
                                        GameMove::from(DefaultMove::from(Movement::of('♔', "E2", "E3"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("D8"),
                                    vec![
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "E8"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "F8"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "G8"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "H8"))),
                                        //
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "D7"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "D6"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "D5"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "D4"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "D3"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "D2"))),
                                        GameMove::from(CaptureMove::from(Movement::of('♖', "D8", "D1"))),
                                        //
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "C8"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "B8"))),
                                        GameMove::from(DefaultMove::from(Movement::of('♖', "D8", "A8"))),
                                    ]
                                ),
                            ]),
                        },
                    ),
                ]),
                history: vec![
                    Movement::of('♙', "A2", "A4"),
                    Movement::of('♟', "A7", "A5"),
                    Movement::of('♙', "B2", "B4"),
                    Movement::of('♟', "B7", "B5"),
                    Movement::of('♙', "C2", "C4"),
                    Movement::of('♟', "C7", "C5"),
                    Movement::of('♙', "D2", "D4"),
                    Movement::of('♟', "D7", "D5"),
                    Movement::of('♙', "E2", "E4"),
                    Movement::of('♟', "E7", "E5"),
                    Movement::of('♙', "F2", "F4"),
                    Movement::of('♟', "F7", "F5"),
                    Movement::of('♙', "G2", "G4"),
                    Movement::of('♟', "G7", "G5"),
                    Movement::of('♙', "H2", "H4"),
                    Movement::of('♟', "H7", "H5"),
                    //
                    Movement::of('♙', "B4", "A5"),
                    Movement::of('♟', "B5", "A4"),
                    Movement::of('♙', "D4", "C5"),
                    Movement::of('♟', "D5", "C4"),
                    Movement::of('♙', "E4", "F5"),
                    Movement::of('♟', "E5", "F4"),
                    Movement::of('♙', "G4", "H5"),
                    Movement::of('♟', "G5", "H4"),
                    //
                    Movement::of('♖', "A1", "A4"),
                    Movement::of('♜', "A8", "A5"),
                    Movement::of('♖', "H1", "H4"),
                    Movement::of('♜', "H8", "H5"),
                    //
                    Movement::of('♗', "C1", "F4"),
                    Movement::of('♝', "F8", "C5"),
                    Movement::of('♗', "F1", "C4"),
                    Movement::of('♝', "C8", "F5"),
                    //
                    Movement::of('♗', "F4", "B8"),
                    Movement::of('♝', "C5", "G1"),
                    Movement::of('♗', "C4", "G8"),
                    Movement::of('♝', "F5", "B1"),
                    //
                    Movement::of('♖', "A4", "A1"),
                    Movement::of('♜', "A5", "A8"),
                    Movement::of('♖', "H4", "H1"),
                    Movement::of('♜', "H5", "H8"),
                    //
                    Movement::of('♖', "A1", "B1"),
                    Movement::of('♜', "A8", "B8"),
                    Movement::of('♖', "H1", "G1"),
                    Movement::of('♜', "H8", "G8"),
                    //
                    Movement::of('♔', "E1", "E2"),
                    Movement::of('♚', "E8", "E7"),
                    //
                    Movement::of('♖', "G1", "G8"),
                    Movement::of('♜', "B8", "B1"),
                    Movement::of('♖', "G8", "D8"),
                    Movement::of('♜', "B1", "D1"),
                ],
            }
        );
    }
}
