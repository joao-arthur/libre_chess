use std::collections::{HashMap, HashSet};

use crate::{
    color::Color,
    game::{
        Game, game::GameHistory, mode::GameMode, movement::menace, player::GamePlayer,
        rule::allowed_movements::allowed_movements_of_player,
    },
};

pub fn init_game(mode: GameMode) -> Game {
    let players = HashMap::from([
        (
            Color::Black,
            GamePlayer {
                color: Color::Black,
                captures: Vec::new(),
                menace: menace::menace_of_player(&mode.initial_board, &mode.bounds, &Color::Black),
                moves: HashMap::new(),
            },
        ),
        (
            Color::White,
            GamePlayer {
                color: Color::White,
                captures: Vec::new(),
                menace: menace::menace_of_player(&mode.initial_board, &mode.bounds, &Color::White),
                moves: HashMap::new(),
            },
        ),
    ]);
    let mut game =
        Game { board: mode.initial_board, bounds: mode.bounds, players, history: Vec::new() };
    let mut player = game.players.get_mut(&Color::White).unwrap();
    let moves =
        allowed_movements_of_player(&game.board, &game.bounds, &game.history, &Color::White);
    player.moves = moves;
    game
}

fn game_of_history(mode: GameMode, history: GameHistory) -> Game {
    let mut board = mode.initial_board;
    let history_iter = history.iter();
    for movement in history_iter {
        if let Some(piece) = board.remove(&movement.from) {
            board.insert(movement.to.clone(), piece);
        }
    }
    Game {
        bounds: mode.bounds,
        board,
        players: HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    menace: HashSet::new(),
                    moves: HashMap::new(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    menace: HashSet::new(),
                    moves: HashMap::new(),
                },
            ),
        ]),
        history,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::{Pos, pos_of_str_slice},
        color::Color,
        game::{
            Game, board,
            mode::standard_chess,
            movement::movement::{DefaultMovement, GameMovement},
            piece,
            player::GamePlayer,
        },
        geometry::poligon::rect::RectU8,
        movement::Movement,
    };

    use super::init_game;

    #[test]
    fn test_init_game_standard_chess() {
        assert_eq!(
            init_game(standard_chess()),
            Game {
                board: board::of_str([
                    "♜♞♝♛♚♝♞♜",
                    "♟♟♟♟♟♟♟♟",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "♙♙♙♙♙♙♙♙",
                    "♖♘♗♕♔♗♘♖",
                ]),
                bounds: RectU8 { x1: 0, y1: 0, x2: 7, y2: 7 },
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
                                (Pos::of_str("A1"), vec![]),
                                (Pos::of_str("C1"), vec![]),
                                (Pos::of_str("D1"), vec![]),
                                (Pos::of_str("E1"), vec![]),
                                (Pos::of_str("F1"), vec![]),
                                (Pos::of_str("H1"), vec![]),
                            ]),
                        },
                    ),
                ]),
                history: Vec::new(),
            }
        )
    }

    fn test_game_of() {}
}
