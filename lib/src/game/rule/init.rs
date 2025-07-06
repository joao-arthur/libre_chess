use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        Game,
        game::GameHistory,
        mode::GameMode,
        movement::menace::menace_of_player,
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
            GamePlayer {
                color: Color::Black,
                captures: Vec::new(),
                menace: menace_of_player(&board, &bounds, Color::Black),
                moves: HashMap::new(),
            },
        ),
        (
            Color::White,
            GamePlayer {
                color: Color::White,
                captures: Vec::new(),
                menace: menace_of_player(&board, &bounds, Color::White),
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
                menace: menace_of_player(&board, &bounds, Color::Black),
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
                menace: menace_of_player(&board, &bounds, Color::White),
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
        board::pos::{Pos, pos_of_str_slice},
        color::Color,
        game::{
            Game,
            board::board_of_str,
            game::GameBounds,
            mode::standard_chess,
            movement::movement::{CaptureMovement, DefaultMovement, GameMovement},
            player::GamePlayer,
        },
        movement::Movement,
    };

    use super::{game_of_mode, game_of_mode_and_history};

    #[test]
    fn game_of_mode_standard_chess() {
        let mode = standard_chess();
        assert_eq!(
            game_of_mode(standard_chess()),
            Game {
                board: board_of_str(
                    &mode.bounds,
                    [
                        "♜♞♝♛♚♝♞♜",
                        "♟♟♟♟♟♟♟♟",
                        "        ",
                        "        ",
                        "        ",
                        "        ",
                        "♙♙♙♙♙♙♙♙",
                        "♖♘♗♕♔♗♘♖",
                    ]
                ),
                bounds: GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 },
                players: HashMap::from([
                    (
                        Color::Black,
                        GamePlayer {
                            color: Color::Black,
                            captures: Vec::new(),
                            menace: pos_of_str_slice([
                                /**/ "B8", "C8", "D8", "E8", "F8", "G8", //
                                "A7", "B7", "C7", "D7", "E7", "F7", "G7", "H7", //
                                "A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6",
                            ])
                            .into_iter()
                            .collect(),
                            moves: HashMap::new(),
                        },
                    ),
                    (
                        Color::White,
                        GamePlayer {
                            color: Color::White,
                            captures: Vec::new(),
                            menace: pos_of_str_slice([
                                "A3", "B3", "C3", "D3", "E3", "F3", "G3", "H3", //
                                "A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2", //
                                /**/ "B1", "C1", "D1", "E1", "F1", "G1",
                            ])
                            .into_iter()
                            .collect(),
                            moves: HashMap::from([
                                (
                                    Pos::of_str("A2"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "A2", "A3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "A2", "A4"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("B2"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "B2", "B3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "B2", "B4"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("C2"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "C2", "C3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "C2", "C4"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("D2"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "D2", "D3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "D2", "D4"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("E2"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "E2", "E3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "E2", "E4"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("F2"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "F2", "F3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "F2", "F4"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("G2"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "G2", "G3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "G2", "G4"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("H2"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "H2", "H3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♙', "H2", "H4"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("B1"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♘', "B1", "C3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♘', "B1", "A3"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("G1"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♘', "G1", "H3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♘', "G1", "F3"))),
                                    ]
                                ),
                            ]),
                        },
                    ),
                ]),
                history: Vec::new(),
            }
        );
    }

    #[test]
    fn game_of_mode_and_history_standard_chess() {
        let mode = standard_chess();
        let history = vec![
            Movement::of_str('♙', "A2", "A4"),
            Movement::of_str('♟', "A7", "A5"),
            Movement::of_str('♙', "B2", "B4"),
            Movement::of_str('♟', "B7", "B5"),
            Movement::of_str('♙', "C2", "C4"),
            Movement::of_str('♟', "C7", "C5"),
            Movement::of_str('♙', "D2", "D4"),
            Movement::of_str('♟', "D7", "D5"),
            Movement::of_str('♙', "E2", "E4"),
            Movement::of_str('♟', "E7", "E5"),
            Movement::of_str('♙', "F2", "F4"),
            Movement::of_str('♟', "F7", "F5"),
            Movement::of_str('♙', "G2", "G4"),
            Movement::of_str('♟', "G7", "G5"),
            Movement::of_str('♙', "H2", "H4"),
            Movement::of_str('♟', "H7", "H5"),
            //
            Movement::of_str('♙', "B4", "A5"),
            Movement::of_str('♟', "B5", "A4"),
            Movement::of_str('♙', "D4", "C5"),
            Movement::of_str('♟', "D5", "C4"),
            Movement::of_str('♙', "E4", "F5"),
            Movement::of_str('♟', "E5", "F4"),
            Movement::of_str('♙', "G4", "H5"),
            Movement::of_str('♟', "G5", "H4"),
            //
            Movement::of_str('♖', "A1", "A4"),
            Movement::of_str('♜', "A8", "A5"),
            Movement::of_str('♖', "H1", "H4"),
            Movement::of_str('♜', "H8", "H5"),
            //
            Movement::of_str('♗', "C1", "F4"),
            Movement::of_str('♝', "F8", "C5"),
            Movement::of_str('♗', "F1", "C4"),
            Movement::of_str('♝', "C8", "F5"),
            Movement::of_str('♗', "F4", "B8"),
            Movement::of_str('♝', "C5", "G1"),
            Movement::of_str('♗', "C4", "G8"),
            Movement::of_str('♝', "F5", "B1"),
            //
            Movement::of_str('♖', "A4", "A1"),
            Movement::of_str('♜', "A5", "A8"),
            Movement::of_str('♖', "H4", "H1"),
            Movement::of_str('♜', "H5", "H8"),
            Movement::of_str('♖', "A1", "B1"),
            Movement::of_str('♜', "A8", "B8"),
            Movement::of_str('♖', "H1", "G1"),
            Movement::of_str('♜', "H8", "G8"),
            //
            Movement::of_str('♔', "E1", "E2"),
            Movement::of_str('♚', "E8", "E7"),
            //
            Movement::of_str('♖', "G1", "G8"),
            Movement::of_str('♜', "B8", "B1"),
            Movement::of_str('♖', "G8", "D8"),
            Movement::of_str('♜', "B1", "D1"),
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
                            menace: pos_of_str_slice([
                                "E1", "F1", "G1", "H1", //
                                "C1", "B1", "A1", //
                                "D2", "D3", "D4", "D5", "D6", "D7", "D8", //
                                "F8", "F7", "F6", "E6", "D6", "D7", "D8", "E8",
                            ])
                            .into_iter()
                            .collect(),
                            moves: HashMap::new(),
                        },
                    ),
                    (
                        Color::White,
                        GamePlayer {
                            color: Color::White,
                            captures: Vec::new(),
                            menace: pos_of_str_slice([
                                "E8", "F8", "G8", "H8", //
                                "D7", "D6", "D5", "D4", "D3", "D2", "D1", //
                                "C8", "B8", "A8", //
                                "F3", "F2", "F1", "E1", "D1", "D2", "D3", "E3",
                            ])
                            .into_iter()
                            .collect(),
                            moves: HashMap::from([
                                (
                                    Pos::of_str("E2"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "E2", "F3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "E2", "F2"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "E2", "F1"))), //////
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "E2", "E1"))), //////
                                        GameMovement::from(CaptureMovement::from(Movement::of_str('♔', "E2", "D1"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "E2", "D2"))), //////
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "E2", "D3"))), //////
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♔', "E2", "E3"))),
                                    ]
                                ),
                                (
                                    Pos::of_str("D8"),
                                    vec![
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "E8"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "F8"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "G8"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "H8"))),
                                        //
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "D7"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "D6"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "D5"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "D4"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "D3"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "D2"))),
                                        GameMovement::from(CaptureMovement::from(Movement::of_str('♖', "D8", "D1"))),
                                        //
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "C8"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "B8"))),
                                        GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D8", "A8"))),
                                    ]
                                ),
                            ]),
                        },
                    ),
                ]),
                history: vec![
                    Movement::of_str('♙', "A2", "A4"),
                    Movement::of_str('♟', "A7", "A5"),
                    Movement::of_str('♙', "B2", "B4"),
                    Movement::of_str('♟', "B7", "B5"),
                    Movement::of_str('♙', "C2", "C4"),
                    Movement::of_str('♟', "C7", "C5"),
                    Movement::of_str('♙', "D2", "D4"),
                    Movement::of_str('♟', "D7", "D5"),
                    Movement::of_str('♙', "E2", "E4"),
                    Movement::of_str('♟', "E7", "E5"),
                    Movement::of_str('♙', "F2", "F4"),
                    Movement::of_str('♟', "F7", "F5"),
                    Movement::of_str('♙', "G2", "G4"),
                    Movement::of_str('♟', "G7", "G5"),
                    Movement::of_str('♙', "H2", "H4"),
                    Movement::of_str('♟', "H7", "H5"),
                    //
                    Movement::of_str('♙', "B4", "A5"),
                    Movement::of_str('♟', "B5", "A4"),
                    Movement::of_str('♙', "D4", "C5"),
                    Movement::of_str('♟', "D5", "C4"),
                    Movement::of_str('♙', "E4", "F5"),
                    Movement::of_str('♟', "E5", "F4"),
                    Movement::of_str('♙', "G4", "H5"),
                    Movement::of_str('♟', "G5", "H4"),
                    //
                    Movement::of_str('♖', "A1", "A4"),
                    Movement::of_str('♜', "A8", "A5"),
                    Movement::of_str('♖', "H1", "H4"),
                    Movement::of_str('♜', "H8", "H5"),
                    //
                    Movement::of_str('♗', "C1", "F4"),
                    Movement::of_str('♝', "F8", "C5"),
                    Movement::of_str('♗', "F1", "C4"),
                    Movement::of_str('♝', "C8", "F5"),
                    //
                    Movement::of_str('♗', "F4", "B8"),
                    Movement::of_str('♝', "C5", "G1"),
                    Movement::of_str('♗', "C4", "G8"),
                    Movement::of_str('♝', "F5", "B1"),
                    //
                    Movement::of_str('♖', "A4", "A1"),
                    Movement::of_str('♜', "A5", "A8"),
                    Movement::of_str('♖', "H4", "H1"),
                    Movement::of_str('♜', "H5", "H8"),
                    //
                    Movement::of_str('♖', "A1", "B1"),
                    Movement::of_str('♜', "A8", "B8"),
                    Movement::of_str('♖', "H1", "G1"),
                    Movement::of_str('♜', "H8", "G8"),
                    //
                    Movement::of_str('♔', "E1", "E2"),
                    Movement::of_str('♚', "E8", "E7"),
                    //
                    Movement::of_str('♖', "G1", "G8"),
                    Movement::of_str('♜', "B8", "B1"),
                    Movement::of_str('♖', "G8", "D8"),
                    Movement::of_str('♜', "B1", "D1"),
                ],
            }
        );
    }
}
