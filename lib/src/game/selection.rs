use std::collections::HashSet;

use crate::{
    board::pos::Pos,
    game::{
        GameBoard,
        game::{GameHistory, GamePlayers},
        movement::movement::GameMovement,
        rule::turn::evaluate_turn,
    },
};

#[derive(Debug, PartialEq)]
pub struct Selection {
    pub selected_squares: HashSet<Pos>,
    pub selected_piece: Option<Pos>,
    // If it gets hard to handle: HashMap<Pos, Vec<GameMovement>>
    pub selected_piece_movements: Vec<GameMovement>,
}

pub fn toggle_selection(
    selection: &mut Selection,
    board: &GameBoard,
    players: &GamePlayers,
    history: &GameHistory,
    pos: Pos,
) {
    // Player selected a movement
    if selection
        .selected_piece_movements
        .iter()
        .find(|mov| match mov {
            GameMovement::Default(mov) => mov.movement.to == pos,
            GameMovement::EnPassant(mov) => mov.movement.to == pos,
            GameMovement::Castling(mov) => mov.movement.to == pos,
            _ => false,
        })
        .is_some()
    {
        selection.selected_squares.clear();
        selection.selected_piece = None;
        selection.selected_piece_movements.clear();
        return;
    }
    if let Some(piece) = board.get(&pos) {
        // Player selected the already selected piece
        if let Some(selected_piece) = &selection.selected_piece {
            if &pos == selected_piece {
                selection.selected_piece = None;
                selection.selected_piece_movements.clear();
                return;
            }
        }
        let turn = evaluate_turn(history);
        // Player selected another piece of himself
        if turn == piece.color {
            if let Some(player) = players.get(&turn) {
                if let Some(moves) = player.moves.get(&pos) {
                    selection.selected_squares.clear();
                    selection.selected_piece = Some(pos.clone());
                    selection.selected_piece_movements = moves.clone();
                    return;
                }
            }
            // Player same player piece that is locked
            selection.selected_squares.clear();
            selection.selected_piece = None;
            selection.selected_piece_movements.clear();
            return;
        }
        // Player selected another player piece
        selection.selected_squares.clear();
        selection.selected_piece = None;
        selection.selected_piece_movements.clear();
        return;
    }
    // Player selected empty square already selected
    if selection.selected_squares.contains(&pos) {
        selection.selected_squares.remove(&pos);
        return;
    }
    // empty square
    selection.selected_squares.insert(pos);
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        board::pos::Pos,
        color::Color,
        game::{
            board::board_of_str,
            mode::standard_chess,
            movement::movement::{DefaultMovement, GameMovement},
            player::GamePlayer,
        },
        movement::Movement,
    };

    use super::{Selection, toggle_selection};

    #[test]
    fn player_selected_empty_square() {
        let mut selection = Selection {
            selected_squares: HashSet::new(),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        let players = HashMap::from([
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
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("D4"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::from([Pos::of_str("D4")]),
                selected_piece: None,
                selected_piece_movements: Vec::new(),
            }
        );
    }

    #[test]
    fn player_toggle_select_empty_square() {
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        let players = HashMap::from([
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
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("D4"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::new(),
                selected_piece: None,
                selected_piece_movements: Vec::new(),
            }
        );
    }

    #[test]
    fn player_select_own_piece() {
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        let players = HashMap::from([
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
                    moves: HashMap::from([(
                        Pos::of_str("B2"),
                        vec![
                            GameMovement::from(DefaultMovement::from(Movement::of_str(
                                '♟', "B2", "B3",
                            ))),
                            GameMovement::from(DefaultMovement::from(Movement::of_str(
                                '♟', "B2", "B4",
                            ))),
                        ],
                    )]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("B2"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::new(),
                selected_piece: Some(Pos::of_str("B2")),
                selected_piece_movements: vec![
                    GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "B2", "B3"))),
                    GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "B2", "B4"))),
                ],
            }
        );
    }

    #[test]
    fn player_toggle_selected_piece() {
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: Some(Pos::of_str("B2")),
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        let players = HashMap::from([
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
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("B2"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::from([Pos::of_str("D4")]),
                selected_piece: None,
                selected_piece_movements: Vec::new(),
            }
        );
    }

    #[test]
    fn player_select_another_player_piece() {
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        let players = HashMap::from([
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
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("G7"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::new(),
                selected_piece: None,
                selected_piece_movements: Vec::new(),
            }
        );
    }

    #[test]
    fn player_select_own_piece_then_another_own_piece() {
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: Some(Pos::of_str("B2")),
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        let players = HashMap::from([
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
                    moves: HashMap::from([(
                        Pos::of_str("C2"),
                        vec![
                            GameMovement::from(DefaultMovement::from(Movement::of_str(
                                '♟', "C2", "C3",
                            ))),
                            GameMovement::from(DefaultMovement::from(Movement::of_str(
                                '♟', "C2", "C4",
                            ))),
                        ],
                    )]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("C2"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::new(),
                selected_piece: Some(Pos::of_str("C2")),
                selected_piece_movements: vec![
                    GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "C2", "C3"))),
                    GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "C2", "C4"))),
                ],
            }
        );
    }

    #[test]
    fn player_select_own_piece_then_another_player_piece() {
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: Some(Pos::of_str("B2")),
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        let players = HashMap::from([
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
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("B7"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::new(),
                selected_piece: None,
                selected_piece_movements: Vec::new(),
            }
        );
    }

    #[test]
    fn player_select_movement() {
        let mut selection = Selection {
            selected_squares: HashSet::new(),
            selected_piece: Some(Pos::of_str("D5")),
            selected_piece_movements: vec![
                GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D5", "E5"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D5", "D4"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D5", "C5"))),
                GameMovement::from(DefaultMovement::from(Movement::of_str('♖', "D5", "D6"))),
            ],
        };
        let mode = standard_chess();
        let board = board_of_str(&mode, [
            "    ♚   ",
            "        ",
            "   ♟    ",
            "  ♟♖♟   ",
            "   ♟    ",
            "        ",
            "        ",
            "    ♔   ",
        ]);
        let players = HashMap::from([
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
                    moves: HashMap::from([(
                        Pos::of_str("D5"),
                        vec![
                            GameMovement::from(DefaultMovement::from(Movement::of_str(
                                '♖', "D5", "E5",
                            ))),
                            GameMovement::from(DefaultMovement::from(Movement::of_str(
                                '♖', "D5", "D4",
                            ))),
                            GameMovement::from(DefaultMovement::from(Movement::of_str(
                                '♖', "D5", "C5",
                            ))),
                            GameMovement::from(DefaultMovement::from(Movement::of_str(
                                '♖', "D5", "D6",
                            ))),
                        ],
                    )]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("E5"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::new(),
                selected_piece: None,
                selected_piece_movements: Vec::new(),
            }
        );
    }
}
