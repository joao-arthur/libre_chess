use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        Game, mode::GameMode, movement::menace, player::GamePlayer,
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::pos_of_str_slice,
        color::Color,
        game::{Game, mode::standard_chess, piece, player::GamePlayer},
        geometry::poligon::rect::RectU8,
    };

    use super::init_game;

    #[test]
    fn test_init_game_standard_chess() {
        assert_eq!(
            init_game(standard_chess()),
            Game {
                board: HashMap::from([
                    piece::of_str("A8", "♜"),
                    piece::of_str("B8", "♞"),
                    piece::of_str("C8", "♝"),
                    piece::of_str("D8", "♛"),
                    piece::of_str("E8", "♚"),
                    piece::of_str("F8", "♝"),
                    piece::of_str("G8", "♞"),
                    piece::of_str("H8", "♜"),
                    piece::of_str("A7", "♟"),
                    piece::of_str("B7", "♟"),
                    piece::of_str("C7", "♟"),
                    piece::of_str("D7", "♟"),
                    piece::of_str("E7", "♟"),
                    piece::of_str("F7", "♟"),
                    piece::of_str("G7", "♟"),
                    piece::of_str("H7", "♟"),
                    piece::of_str("A2", "♙"),
                    piece::of_str("B2", "♙"),
                    piece::of_str("C2", "♙"),
                    piece::of_str("D2", "♙"),
                    piece::of_str("E2", "♙"),
                    piece::of_str("F2", "♙"),
                    piece::of_str("G2", "♙"),
                    piece::of_str("H2", "♙"),
                    piece::of_str("A1", "♖"),
                    piece::of_str("B1", "♘"),
                    piece::of_str("C1", "♗"),
                    piece::of_str("D1", "♕"),
                    piece::of_str("E1", "♔"),
                    piece::of_str("F1", "♗"),
                    piece::of_str("G1", "♘"),
                    piece::of_str("H1", "♖"),
                ]),
                bounds: RectU8 { x1: 0, y1: 0, x2: 7, y2: 7 },
                players: HashMap::from([
                    (
                        Color::Black,
                        GamePlayer {
                            color: Color::Black,
                            captures: Vec::new(),
                            menace: pos_of_str_slice([
                                "B8", "C8", "D8", "E8", "F8", "G8", //
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
                                "B1", "C1", "D1", "E1", "F1", "G1",
                            ])
                            .into_iter()
                            .collect(),
                            moves: HashMap::new(),
                        },
                    ),
                ]),
                history: Vec::new(),
            }
        )
    }
}
