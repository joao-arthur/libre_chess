use std::collections::HashSet;

use crate::{
    board::pos::Pos,
    game::{
        game::{GameHistory, GamePlayers}, movement::movement::GameMovement, rule::turn::evaluate_turn, GameBoard
    },
};

#[derive(Debug, PartialEq)]
pub struct Selection {
    pub selected_squares: HashSet<Pos>,
    pub selected_piece: Option<Pos>,
    pub selected_piece_movements: Vec<GameMovement>,
}

pub fn toggle(
    selection: &mut Selection,
    board: &GameBoard,
    players: &GamePlayers,
    history: &GameHistory,
    pos: Pos,
) {
    if let Some(piece) = board.get(&pos) {
        if let Some(selected_piece) = &selection.selected_piece {
            if &pos == selected_piece {
                selection.selected_piece = None;
                selection.selected_piece_movements.clear();
                return;
            }
        }
        let turn = evaluate_turn(history);
        selection.selected_squares.clear();
        if turn == piece.color {
            selection.selected_piece = Some(pos.clone());
            selection.selected_piece_movements.clear();
            if let Some(player) = players.get(&turn) {
                if let Some(moves) = player.moves.get(&pos) {
                    selection.selected_piece_movements = moves.clone();
                }
            }
        } else {
            selection.selected_piece = None;
            selection.selected_piece_movements.clear();
        }
        return;
    }
    if selection.selected_squares.contains(&pos) {
        selection.selected_squares.remove(&pos);
    } else {
        selection.selected_squares.insert(pos);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        board::pos::Pos,
        color::Color,
        game::{mode::standard_chess, movement::movement::{DefaultMovement, GameMovement}, player::GamePlayer}, movement::Movement,
    };

    use super::{Selection, toggle};

    #[test]
    fn select_empty_square() {
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
        toggle(&mut selection, &board, &players, &history, Pos::of_str("D4"));
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
    fn toggle_unselect_square() {
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
        toggle(&mut selection, &board, &players, &history, Pos::of_str("D4"));
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
    fn select_user_piece() {
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
                    moves: HashMap::from([
                        (Pos::of_str("B2"), vec![
                            GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "B2", "B3"))),
                            GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "B2", "B4"))),
                        ])
                    ]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle(&mut selection, &board, &players, &history, Pos::of_str("B2"));
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
    fn select_other_user_piece() {
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
        toggle(&mut selection, &board, &players, &history, Pos::of_str("G7"));
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
    fn selected_piece_then_same_piece() {
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
        toggle(&mut selection, &board, &players, &history, Pos::of_str("B2"));
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
    fn selected_piece_then_another_piece() {
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
                    moves: HashMap::from([
                        (Pos::of_str("C2"), vec![
                            GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "C2", "C3"))),
                            GameMovement::from(DefaultMovement::from(Movement::of_str('♟', "C2", "C4"))),
                        ])
                    ]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle(&mut selection, &board, &players, &history, Pos::of_str("C2"));
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
    fn selected_piece_then_another_user_piece() {
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
        toggle(&mut selection, &board, &players, &history, Pos::of_str("B7"));
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
