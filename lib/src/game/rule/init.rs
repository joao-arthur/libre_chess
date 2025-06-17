use std::collections::HashMap;

use crate::{
    color::Color,
    game::{Game, mode::Mode, movement::naive, player::Player},
};

pub fn init_game(mode: Mode) -> Game {
    let players = HashMap::from([
        (
            Color::White,
            Player {
                color: Color::White,
                captured_pieces: Vec::new(),
                possible_movements: naive::movements_of_player(
                    &mode.initial_board,
                    &mode.bounds,
                    &Color::White,
                ),
            },
        ),
        (
            Color::Black,
            Player {
                color: Color::Black,
                captured_pieces: Vec::new(),
                possible_movements: naive::movements_of_player(
                    &mode.initial_board,
                    &mode.bounds,
                    &Color::Black,
                ),
            },
        ),
    ]);
    Game { board: mode.initial_board, bounds: mode.bounds, players, history: Vec::new() }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        board::pos::pos_of_str_slice,
        color::Color,
        game::{Game, mode::standard_chess, piece, player::Player},
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
                        Color::White,
                        Player {
                            color: Color::White,
                            captured_pieces: Vec::new(),
                            possible_movements: pos_of_str_slice([
                                "A3", "B3", "C3", "D3", "E3", "F3", "G3", "H3", "A4", "B4", "C4",
                                "D4", "E4", "F4", "G4", "H4",
                            ])
                            .into_iter()
                            .collect(),
                        },
                    ),
                    (
                        Color::Black,
                        Player {
                            color: Color::Black,
                            captured_pieces: Vec::new(),
                            possible_movements: pos_of_str_slice([
                                "A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6", "A5", "B5", "C5",
                                "D5", "E5", "F5", "G5", "H5",
                            ])
                            .into_iter()
                            .collect(),
                        },
                    ),
                ]),
                history: Vec::new(),
            }
        )
    }
}
