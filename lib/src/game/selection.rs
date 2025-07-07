use std::collections::HashSet;

use crate::{
    game::{
        GameBoard,
        game::{GameHistory, GamePlayers},
        mov::GameMov,
        rule::turn::evaluate_turn,
    },
    pos::Pos,
};

#[derive(Debug, PartialEq)]
pub struct Selection {
    pub selected_squares: HashSet<Pos>,
    pub selected_piece: Option<Pos>,
    // If it gets hard to handle: HashMap<Pos, Vec<GameMov>>
    pub selected_piece_movements: Vec<GameMov>,
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
            GameMov::Default(mov) => mov.mov.to == pos,
            GameMov::EnPassant(mov) => mov.mov.to == pos,
            GameMov::Castling(mov) => mov.mov.to == pos,
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
        color::Color,
        game::{
            board::board_of_str,
            game::GamePlayers,
            mode::standard_chess,
            mov::{DefaultMov, GameMov},
            player::GamePlayer,
        },
        mov::Mov,
        pos::Pos,
    };

    use super::{Selection, toggle_selection};

    fn empty_players() -> GamePlayers {
        HashMap::from([
            (
                Color::Black,
                GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ])
    }

    #[test]
    fn player_selected_empty_square() {
        let mode = standard_chess();
        let mut selection = Selection {
            selected_squares: HashSet::new(),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        let pos = Pos::of_str("D4");
        toggle_selection(&mut selection, &board, &players, &history, pos.clone());
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::from([pos.clone()]),
                selected_piece: None,
                selected_piece_movements: Vec::new(),
            }
        );
    }

    #[test]
    fn player_toggle_select_empty_square() {
        let mode = standard_chess();
        let pos = Pos::of_str("D4");
        let mut selection = Selection {
            selected_squares: HashSet::from([pos.clone()]),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, pos.clone());
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
        let mode = standard_chess();
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = mode.initial_board;
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
                    moves: HashMap::from([(
                        Pos::of_str("B2"),
                        vec![
                            GameMov::from(DefaultMov::from(Mov::of('♟', "B2", "B3"))),
                            GameMov::from(DefaultMov::from(Mov::of('♟', "B2", "B4"))),
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
                    GameMov::from(DefaultMov::from(Mov::of('♟', "B2", "B3"))),
                    GameMov::from(DefaultMov::from(Mov::of('♟', "B2", "B4"))),
                ],
            }
        );
    }

    #[test]
    fn player_toggle_selected_piece() {
        let mode = standard_chess();
        let pos = Pos::of_str("B2");
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: Some(pos.clone()),
            selected_piece_movements: Vec::new(),
        };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, pos.clone());
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
        let mode = standard_chess();
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        let pos = Pos::of_str("G7");
        toggle_selection(&mut selection, &board, &players, &history, pos);
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
        let mode = standard_chess();
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: Some(Pos::of_str("B2")),
            selected_piece_movements: Vec::new(),
        };
        let board = mode.initial_board;
        let pos = Pos::of_str("C2");
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
                    moves: HashMap::from([(
                        pos.clone(),
                        vec![
                            GameMov::from(DefaultMov::from(Mov::of('♟', "C2", "C3"))),
                            GameMov::from(DefaultMov::from(Mov::of('♟', "C2", "C4"))),
                        ],
                    )]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, pos.clone());
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::new(),
                selected_piece: Some(pos.clone()),
                selected_piece_movements: vec![
                    GameMov::from(DefaultMov::from(Mov::of('♟', "C2", "C3"))),
                    GameMov::from(DefaultMov::from(Mov::of('♟', "C2", "C4"))),
                ],
            }
        );
    }

    #[test]
    fn player_select_own_piece_then_another_player_piece() {
        let mode = standard_chess();
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: Some(Pos::of_str("B2")),
            selected_piece_movements: Vec::new(),
        };
        let board = mode.initial_board;
        let players = empty_players();
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
                GameMov::from(DefaultMov::from(Mov::of('♖', "D5", "E5"))),
                GameMov::from(DefaultMov::from(Mov::of('♖', "D5", "D4"))),
                GameMov::from(DefaultMov::from(Mov::of('♖', "D5", "C5"))),
                GameMov::from(DefaultMov::from(Mov::of('♖', "D5", "D6"))),
            ],
        };
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "   ♟    ",
                "  ♟♖♟   ",
                "   ♟    ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
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
                    moves: HashMap::from([(
                        Pos::of_str("D5"),
                        vec![
                            GameMov::from(DefaultMov::from(Mov::of('♖', "D5", "E5"))),
                            GameMov::from(DefaultMov::from(Mov::of('♖', "D5", "D4"))),
                            GameMov::from(DefaultMov::from(Mov::of('♖', "D5", "C5"))),
                            GameMov::from(DefaultMov::from(Mov::of('♖', "D5", "D6"))),
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
