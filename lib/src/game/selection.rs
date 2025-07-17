use std::collections::HashSet;

use crate::{
    game::{
        board::GameBoard,
        game::{GameHistory, GamePlayers},
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
                    if selected_piece_moves.iter().any(|(to, _)| to == &pos) {
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
                    // if moves.is_empty() {} -> blocked piece

                    if 1 == 1 {
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
    use std::collections::HashSet;

    use crate::{
        color::Color,
        game::{
            board::board_of_str,
            game::{GamePlayers, empty_players},
            mode::standard_chess,
            mov::PieceMoveType,
            player::GamePlayer,
        },
        pos::Pos,
    };

    use super::{Selection, toggle_selection};

    #[test]
    fn player_selected_empty_square() {
        let mode = standard_chess();
        let mut selection = Selection { selected_squares: HashSet::new(), selected_pos: None };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        let pos = Pos::of("D4");
        toggle_selection(&mut selection, &board, &players, &history, pos.clone());
        assert_eq!(
            selection,
            Selection { selected_squares: HashSet::from([pos.clone()]), selected_pos: None }
        );
    }

    #[test]
    fn player_toggle_select_empty_square() {
        let mode = standard_chess();
        let pos = Pos::of("D4");
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
            Selection { selected_squares: HashSet::from([Pos::of("D4")]), selected_pos: None };
        let board = mode.initial_board;
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [(
                        Pos::of("B2"),
                        [
                            (Pos::of("B3"), PieceMoveType::Default),
                            (Pos::of("B4"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
        ]
        .into();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of("B2"));
        assert_eq!(
            selection,
            Selection { selected_squares: HashSet::new(), selected_pos: Some(Pos::of("B2")) }
        );
    }

    #[test]
    fn player_toggle_selected_piece() {
        let mode = standard_chess();
        let pos = Pos::of("B2");
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of("D4")]),
            selected_pos: Some(pos.clone()),
        };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, pos.clone());
        assert_eq!(
            selection,
            Selection { selected_squares: HashSet::from([Pos::of("D4")]), selected_pos: None }
        );
    }

    #[test]
    fn player_select_another_player_piece() {
        let mode = standard_chess();
        let mut selection =
            Selection { selected_squares: HashSet::from([Pos::of("D4")]), selected_pos: None };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        let pos = Pos::of("G7");
        toggle_selection(&mut selection, &board, &players, &history, pos);
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_own_piece_then_another_own_piece() {
        let mode = standard_chess();
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of("D4")]),
            selected_pos: Some(Pos::of("B2")),
        };
        let board = mode.initial_board;
        let pos = Pos::of("C2");
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [(
                        pos.clone(),
                        [
                            (Pos::of("C3"), PieceMoveType::Default),
                            (Pos::of("C4"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
        ]
        .into();
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
            selected_squares: HashSet::from([Pos::of("D4")]),
            selected_pos: Some(Pos::of("B2")),
        };
        let board = mode.initial_board;
        let players = empty_players();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of("B7"));
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_mov() {
        let mut selection =
            Selection { selected_squares: HashSet::new(), selected_pos: Some(Pos::of("D5")) };
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [(
                        Pos::of("D5"),
                        [
                            (Pos::of("E5"), PieceMoveType::Default),
                            (Pos::of("D4"), PieceMoveType::Default),
                            (Pos::of("C5"), PieceMoveType::Default),
                            (Pos::of("D6"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
        ]
        .into();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of("E5"));
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_en_passant_mov() {
        let mut selection =
            Selection { selected_squares: HashSet::new(), selected_pos: Some(Pos::of("A5")) };
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [(
                        Pos::of("A5"),
                        [
                            (Pos::of("A6"), PieceMoveType::Default),
                            (Pos::of("B6"), PieceMoveType::EnPassant),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
        ]
        .into();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of("B6"));
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_castling_mov() {
        let mut selection =
            Selection { selected_squares: HashSet::new(), selected_pos: Some(Pos::of("E1")) };
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [(
                        Pos::of("E1"),
                        [
                            (Pos::of("A1"), PieceMoveType::ShortCastling),
                            (Pos::of("H1"), PieceMoveType::LongCastling),
                            (Pos::of("F2"), PieceMoveType::Default),
                            (Pos::of("F1"), PieceMoveType::Default),
                            (Pos::of("D1"), PieceMoveType::Default),
                            (Pos::of("D2"), PieceMoveType::Default),
                            (Pos::of("E2"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
        ]
        .into();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of("A1"));
        assert_eq!(selection, Selection { selected_squares: HashSet::new(), selected_pos: None });
    }

    #[test]
    fn player_select_menace_mov() {
        let mut selection =
            Selection { selected_squares: HashSet::new(), selected_pos: Some(Pos::of("E2")) };
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
        let players = [
            (Color::Black, GamePlayer::from(Color::Black)),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [(
                        Pos::of("E2"),
                        [
                            (Pos::of("E3"), PieceMoveType::Default),
                            (Pos::of("E4"), PieceMoveType::Default),
                        ]
                        .into(),
                    )]
                    .into(),
                },
            ),
        ]
        .into();
        let history = Vec::new();
        toggle_selection(&mut selection, &board, &players, &history, Pos::of("F3"));
        assert_eq!(
            selection,
            Selection { selected_squares: HashSet::from([Pos::of("F3")]), selected_pos: None }
        );
    }
}
