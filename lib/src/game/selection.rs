use std::collections::HashSet;

use crate::{
    game::{
        board::GameBoard,
        game::{GameHistory, GamePlayers},
        mov::GameMoveType,
        rule::turn::evaluate_turn,
    },
    pos::Pos,
};

#[derive(Debug, PartialEq)]
pub struct Selection {
    pub selected_squares: HashSet<Pos>,
    pub selected_pos: Option<Pos>,
}

pub fn toggle_selection(
    selection: &mut Selection,
    board: &GameBoard,
    players: &GamePlayers,
    history: &GameHistory,
    pos: Pos,
) {
    // Player selected a possible move out of the selected piece
    if let Some(selected_pos) = &selection.selected_pos {
        if let Some(selected_piece) = board.get(selected_pos) {
            if let Some(player) = players.get(&selected_piece.color) {
                if let Some(selected_piece_moves) = player.moves.get(selected_pos) {
                    if selected_piece_moves
                        .iter()
                        .find(|game_move| match game_move.typ {
                            GameMoveType::Default(_)
                            | GameMoveType::EnPassant(_)
                            | GameMoveType::Capture(_)
                            | GameMoveType::ShortCastling(_)
                            | GameMoveType::LongCastling(_) => game_move.mov.to == pos,
                            _ => false,
                        })
                        .is_some()
                    {
                        selection.selected_squares.clear();
                        selection.selected_pos = None;
                        return;
                    }
                }
            }
        }
    }
    if let Some(piece) = board.get(&pos) {
        // Player selected the already selected piece
        if let Some(selected_pos) = &selection.selected_pos {
            if &pos == selected_pos {
                selection.selected_pos = None;
                return;
            }
        }
        let turn = evaluate_turn(history);
        if turn == piece.color {
            if let Some(player) = players.get(&turn) {
                if let Some(moves) = player.moves.get(&pos) {
                    if moves.iter().any(|game_move| match game_move.typ {
                        GameMoveType::Default(_) => true,
                        GameMoveType::Capture(_) => true,
                        GameMoveType::Menace(_) => false,
                        GameMoveType::EnPassant(_) => true,
                        GameMoveType::LongCastling(_) => true,
                        GameMoveType::ShortCastling(_) => true,
                        GameMoveType::PromotionToQueen(_) => true,
                        GameMoveType::PromotionToRook(_) => true,
                        GameMoveType::PromotionToBishop(_) => true,
                        GameMoveType::PromotionToKnight(_) => true,
                    }) {
                        // Player selected another piece of himself
                        selection.selected_squares.clear();
                        selection.selected_pos = Some(pos.clone());
                    } else {
                        // Player selected another piece of himself that is locked
                        selection.selected_squares.clear();
                        selection.selected_pos = None;
                    }
                    return;
                }
            }
            return;
        }
        // Player selected another player piece
        selection.selected_squares.clear();
        selection.selected_pos = None;
        return;
    }
    // Player selected empty square already selected
    if selection.selected_squares.contains(&pos) {
        selection.selected_squares.remove(&pos);
        return;
    }
    // empty square
    selection.selected_squares.insert(pos);
    selection.selected_pos = None;
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        color::Color,
        game::{
            board::board_of_str, game::GamePlayers, mode::standard_chess, mov::GameMove,
            player::GamePlayer,
        },
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
        let mut selection = Selection { selected_squares: HashSet::new(), selected_pos: None };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        let pos = Pos::of_str("D4");
        toggle_selection(&mut selection, &board, &players, &history, pos.clone());
        assert_eq!(
            selection,
            Selection { selected_squares: HashSet::from([pos.clone()]), selected_pos: None }
        );
    }

    #[test]
    fn player_toggle_select_empty_square() {
        let mode = standard_chess();
        let pos = Pos::of_str("D4");
        let mut selection =
            Selection { selected_squares: HashSet::from([pos.clone()]), selected_pos: None };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, pos.clone());
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_own_piece() {
        let mode = standard_chess();
        let mut selection =
            Selection { selected_squares: HashSet::from([Pos::of_str("D4")]), selected_pos: None };
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
                            GameMove::default_of('♙', "B2", "B3"),
                            GameMove::default_of('♙', "B2", "B4"),
                            GameMove::menace_of('♙', "B2", "A3"),
                            GameMove::menace_of('♙', "B2", "C3"),
                        ],
                    )]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("B2"));
        assert_eq!(
            selection,
            Selection { selected_squares: HashSet::new(), selected_pos: Some(Pos::of_str("B2")) }
        );
    }

    #[test]
    fn player_toggle_selected_piece() {
        let mode = standard_chess();
        let pos = Pos::of_str("B2");
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_pos: Some(pos.clone()),
        };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, pos.clone());
        assert_eq!(
            selection,
            Selection { selected_squares: HashSet::from([Pos::of_str("D4")]), selected_pos: None }
        );
    }

    #[test]
    fn player_select_another_player_piece() {
        let mode = standard_chess();
        let mut selection =
            Selection { selected_squares: HashSet::from([Pos::of_str("D4")]), selected_pos: None };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        let pos = Pos::of_str("G7");
        toggle_selection(&mut selection, &board, &players, &history, pos);
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_own_piece_then_another_own_piece() {
        let mode = standard_chess();
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_pos: Some(Pos::of_str("B2")),
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
                            GameMove::default_of('♙', "C2", "C3"),
                            GameMove::default_of('♙', "C2", "C4"),
                            GameMove::menace_of('♙', "C2", "B3"),
                            GameMove::menace_of('♙', "C2", "D3"),
                        ],
                    )]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, pos.clone());
        assert_eq!(
            selection,
            Selection { selected_squares: HashSet::new(), selected_pos: Some(pos.clone()) }
        );
    }

    #[test]
    fn player_select_own_piece_then_another_player_piece() {
        let mode = standard_chess();
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_pos: Some(Pos::of_str("B2")),
        };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("B7"));
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_mov() {
        let mut selection =
            Selection { selected_squares: HashSet::new(), selected_pos: Some(Pos::of_str("D5")) };
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
                            GameMove::default_of('♖', "D5", "E5"),
                            GameMove::default_of('♖', "D5", "D4"),
                            GameMove::default_of('♖', "D5", "C5"),
                            GameMove::default_of('♖', "D5", "D6"),
                        ],
                    )]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("E5"));
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_en_passant_mov() {
        let mut selection =
            Selection { selected_squares: HashSet::new(), selected_pos: Some(Pos::of_str("A5")) };
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "       ♚",
                "        ",
                "        ",
                "♙♟      ",
                "        ",
                "        ",
                "        ",
                "       ♔",
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
                        Pos::of_str("A5"),
                        vec![
                            GameMove::default_of('♙', "A5", "A6"),
                            GameMove::en_passant_of('♙', "A5", "B6"),
                            GameMove::menace_of('♙', "A5", "B6"),
                        ],
                    )]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("B6"));
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_castling_mov() {
        let mut selection =
            Selection { selected_squares: HashSet::new(), selected_pos: Some(Pos::of_str("E1")) };
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "♜   ♚  ♜",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "♖   ♔  ♖",
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
                        Pos::of_str("E1"),
                        vec![
                            GameMove::short_castling_of('♔', "E1", "A1"),
                            GameMove::long_castling_of('♔', "E1", "H1"),
                            GameMove::default_of('♔', "E1", "F2"),
                            GameMove::default_of('♔', "E1", "F1"),
                            GameMove::default_of('♔', "E1", "D1"),
                            GameMove::default_of('♔', "E1", "D2"),
                            GameMove::default_of('♔', "E1", "E2"),
                        ],
                    )]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("A1"));
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_menace_mov() {
        let mut selection =
            Selection { selected_squares: HashSet::new(), selected_pos: Some(Pos::of_str("E2")) };
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "    ♟   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "    ♙   ",
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
                        Pos::of_str("E2"),
                        vec![
                            GameMove::default_of('♙', "E2", "E3"),
                            GameMove::default_of('♙', "E2", "E4"),
                            GameMove::menace_of('♙', "E2", "D3"),
                            GameMove::menace_of('♙', "E2", "F3"),
                        ],
                    )]),
                },
            ),
        ]);
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of_str("F3"));
        assert_eq!(
            selection,
            Selection { selected_squares: HashSet::from([Pos::of_str("F3")]), selected_pos: None }
        );
    }
}
