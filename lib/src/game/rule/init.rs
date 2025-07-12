use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        game::{Game, GameHistory},
        mode::GameMode,
        player::GamePlayer,
        rule::{allowed_moves::allowed_moves_of_player, turn::evaluate_turn},
    },
};

pub fn game_of_mode(mode: GameMode) -> Game {
    let board = mode.initial_board;
    let bounds = mode.bounds;
    let history = Vec::new();
    let mut players = HashMap::from([
        (
            Color::Black,
            GamePlayer {
                color: Color::Black,
                captures: Vec::new(),
                moves: allowed_moves_of_player(
                    &board,
                    &bounds,
                    &history,
                    &HashMap::from([
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
                                moves: HashMap::new(),
                            },
                        ),
                    ]),
                    &Color::Black,
                ),
            },
        ),
        (
            Color::White,
            GamePlayer {
                color: Color::White,
                captures: Vec::new(),
                moves: allowed_moves_of_player(
                    &board,
                    &bounds,
                    &history,
                    &HashMap::from([
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
                                moves: HashMap::new(),
                            },
                        ),
                    ]),
                    &Color::White,
                ),
            },
        ),
    ]);

    Game { board, bounds, players, history }
}

pub fn game_of_mode_and_history(mode: GameMode, history: GameHistory) -> Game {
    let mut board = mode.initial_board;
    let bounds = mode.bounds;
    let history_it = history.iter();
    for mov in history_it {
        if let Some(piece) = board.remove(&mov.mov.from) {
            board.insert(mov.mov.to.clone(), piece);
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
                    allowed_moves_of_player(
                        &board,
                        &bounds,
                        &history,
                        &HashMap::from([
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
                                    moves: HashMap::new(),
                                },
                            ),
                        ]),
                        &Color::Black,
                    )
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
                    allowed_moves_of_player(
                        &board,
                        &bounds,
                        &history,
                        &HashMap::from([
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
                                    moves: HashMap::new(),
                                },
                            ),
                        ]),
                        &Color::White,
                    )
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
            board::{board_of_str, board_to_string},
            game::Game,
            game::GameBounds,
            mode::standard_chess,
            mov::{GameMove, game_move_vec_to_string},
            player::GamePlayer,
        },
        mov::Mov,
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
    }

    #[test]
    fn game_of_mode_standard_chess_black_player() {
        let mode = standard_chess();
        let game = game_of_mode(standard_chess());
        let black = game.players.get(&Color::Black).unwrap();
        assert_eq!(black.color, Color::Black);
        assert_eq!(black.captures, Vec::new());
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("A7")).unwrap()),
            "".to_owned()
                + "        \n"
                + "♟       \n"
                + "○◌      \n"
                + "○       \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("B7")).unwrap()),
            "".to_owned()
                + "        \n"
                + " ♟      \n"
                + "◌○◌     \n"
                + " ○      \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("C7")).unwrap()),
            "".to_owned()
                + "        \n"
                + "  ♟     \n"
                + " ◌○◌    \n"
                + "  ○     \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("D7")).unwrap()),
            "".to_owned()
                + "        \n"
                + "   ♟    \n"
                + "  ◌○◌   \n"
                + "   ○    \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("E7")).unwrap()),
            "".to_owned()
                + "        \n"
                + "    ♟   \n"
                + "   ◌○◌  \n"
                + "    ○   \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("F7")).unwrap()),
            "".to_owned()
                + "        \n"
                + "     ♟  \n"
                + "    ◌○◌ \n"
                + "     ○  \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("G7")).unwrap()),
            "".to_owned()
                + "        \n"
                + "      ♟ \n"
                + "     ◌○◌\n"
                + "      ○ \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("H7")).unwrap()),
            "".to_owned()
                + "        \n"
                + "       ♟\n"
                + "      ◌○\n"
                + "       ○\n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("A8")).unwrap()),
            "".to_owned()
                + "♜◌      \n"
                + "◌       \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("B8")).unwrap()),
            "".to_owned()
                + " ♞      \n"
                + "   ◌    \n"
                + "○ ○     \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("C8")).unwrap()),
            "".to_owned()
                + "  ♝     \n"
                + " ◌ ◌    \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("D8")).unwrap()),
            "".to_owned()
                + "  ◌♛◌   \n"
                + "  ◌◌◌   \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("E8")).unwrap()),
            "".to_owned()
                + "   ◌♚◌  \n"
                + "   ◌◌◌  \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("F8")).unwrap()),
            "".to_owned()
                + "     ♝  \n"
                + "    ◌ ◌ \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("G8")).unwrap()),
            "".to_owned()
                + "      ♞ \n"
                + "    ◌   \n"
                + "     ○ ○\n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("H8")).unwrap()),
            "".to_owned()
                + "      ◌♜\n"
                + "       ◌\n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
    }

    #[test]
    fn game_of_mode_standard_chess_white_player() {
        let mode = standard_chess();
        let game = game_of_mode(standard_chess());
        let white = game.players.get(&Color::White).unwrap();
        assert_eq!(white.color, Color::White);
        assert_eq!(white.captures, Vec::new());
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("A2")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "○       \n"
                + "○◌      \n"
                + "♙       \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("B2")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + " ○      \n"
                + "◌○◌     \n"
                + " ♙      \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("C2")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "  ○     \n"
                + " ◌○◌    \n"
                + "  ♙     \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("D2")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "   ○    \n"
                + "  ◌○◌   \n"
                + "   ♙    \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("E2")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "    ○   \n"
                + "   ◌○◌  \n"
                + "    ♙   \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("F2")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "     ○  \n"
                + "    ◌○◌ \n"
                + "     ♙  \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("G2")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "      ○ \n"
                + "     ◌○◌\n"
                + "      ♙ \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("H2")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "       ○\n"
                + "      ◌○\n"
                + "       ♙\n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("A1")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "◌       \n"
                + "♖◌      \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("B1")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "○ ○     \n"
                + "   ◌    \n"
                + " ♘      \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("C1")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + " ◌ ◌    \n"
                + "  ♗     \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("D1")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "  ◌◌◌   \n"
                + "  ◌♕◌   \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("E1")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "   ◌◌◌  \n"
                + "   ◌♔◌  \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("F1")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "    ◌ ◌ \n"
                + "     ♗  \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("G1")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "     ○ ○\n"
                + "    ◌   \n"
                + "      ♘ \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("H1")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "       ◌\n"
                + "      ◌♖\n"
        );
    }

    #[test]
    fn game_of_mode_and_history_standard_chess() {
        let mode = standard_chess();
        let history = vec![
            GameMove::default_of('♙', "A2", "A4"),
            GameMove::default_of('♟', "A7", "A5"),
            GameMove::default_of('♙', "B2", "B4"),
            GameMove::default_of('♟', "B7", "B5"),
            GameMove::default_of('♙', "C2", "C4"),
            GameMove::default_of('♟', "C7", "C5"),
            GameMove::default_of('♙', "D2", "D4"),
            GameMove::default_of('♟', "D7", "D5"),
            GameMove::default_of('♙', "E2", "E4"),
            GameMove::default_of('♟', "E7", "E5"),
            GameMove::default_of('♙', "F2", "F4"),
            GameMove::default_of('♟', "F7", "F5"),
            GameMove::default_of('♙', "G2", "G4"),
            GameMove::default_of('♟', "G7", "G5"),
            GameMove::default_of('♙', "H2", "H4"),
            GameMove::default_of('♟', "H7", "H5"),
            GameMove::capture_of('♙', "B4", "A5"),
            GameMove::capture_of('♟', "B5", "A4"),
            GameMove::capture_of('♙', "D4", "C5"),
            GameMove::capture_of('♟', "D5", "C4"),
            GameMove::capture_of('♙', "E4", "F5"),
            GameMove::capture_of('♟', "E5", "F4"),
            GameMove::capture_of('♙', "G4", "H5"),
            GameMove::capture_of('♟', "G5", "H4"),
            GameMove::capture_of('♖', "A1", "A4"),
            GameMove::capture_of('♜', "A8", "A5"),
            GameMove::capture_of('♖', "H1", "H4"),
            GameMove::capture_of('♜', "H8", "H5"),
            GameMove::capture_of('♗', "C1", "F4"),
            GameMove::capture_of('♝', "F8", "C5"),
            GameMove::capture_of('♗', "F1", "C4"),
            GameMove::capture_of('♝', "C8", "F5"),
            GameMove::capture_of('♗', "F4", "B8"),
            GameMove::capture_of('♝', "C5", "G1"),
            GameMove::capture_of('♗', "C4", "G8"),
            GameMove::capture_of('♝', "F5", "B1"),
            GameMove::default_of('♖', "A4", "A1"),
            GameMove::default_of('♜', "A5", "A8"),
            GameMove::default_of('♖', "H4", "H1"),
            GameMove::default_of('♜', "H5", "H8"),
            GameMove::capture_of('♖', "A1", "B1"),
            GameMove::capture_of('♜', "A8", "B8"),
            GameMove::capture_of('♖', "H1", "G1"),
            GameMove::capture_of('♜', "H8", "G8"),
            GameMove::default_of('♔', "E1", "E2"),
            GameMove::default_of('♚', "E8", "E7"),
            GameMove::capture_of('♖', "G1", "G8"),
            GameMove::capture_of('♜', "B8", "B1"),
            GameMove::capture_of('♖', "G8", "D8"),
            GameMove::capture_of('♜', "B1", "D1"),
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
                                        GameMove::default_of('♔', "E2", "F3"),
                                        GameMove::default_of('♔', "E2", "F2"),
                                        GameMove::default_of('♔', "E2", "F1"), //////
                                        GameMove::default_of('♔', "E2", "E1"), //////
                                        GameMove::capture_of('♔', "E2", "D1"),
                                        GameMove::default_of('♔', "E2", "D2"), //////
                                        GameMove::default_of('♔', "E2", "D3"), //////
                                        GameMove::default_of('♔', "E2", "E3"),
                                    ]
                                ),
                                (
                                    Pos::of_str("D8"),
                                    vec![
                                        GameMove::default_of('♖', "D8", "E8"),
                                        GameMove::default_of('♖', "D8", "F8"),
                                        GameMove::default_of('♖', "D8", "G8"),
                                        GameMove::default_of('♖', "D8", "H8"),
                                        //
                                        GameMove::default_of('♖', "D8", "D7"),
                                        GameMove::default_of('♖', "D8", "D6"),
                                        GameMove::default_of('♖', "D8", "D5"),
                                        GameMove::default_of('♖', "D8", "D4"),
                                        GameMove::default_of('♖', "D8", "D3"),
                                        GameMove::default_of('♖', "D8", "D2"),
                                        GameMove::capture_of('♖', "D8", "D1"),
                                        //
                                        GameMove::default_of('♖', "D8", "C8"),
                                        GameMove::default_of('♖', "D8", "B8"),
                                        GameMove::default_of('♖', "D8", "A8"),
                                    ]
                                ),
                            ]),
                        },
                    ),
                ]),
                history: vec![
                    GameMove::default_of('♙', "A2", "A4"),
                    GameMove::default_of('♟', "A7", "A5"),
                    GameMove::default_of('♙', "B2", "B4"),
                    GameMove::default_of('♟', "B7", "B5"),
                    GameMove::default_of('♙', "C2", "C4"),
                    GameMove::default_of('♟', "C7", "C5"),
                    GameMove::default_of('♙', "D2", "D4"),
                    GameMove::default_of('♟', "D7", "D5"),
                    GameMove::default_of('♙', "E2", "E4"),
                    GameMove::default_of('♟', "E7", "E5"),
                    GameMove::default_of('♙', "F2", "F4"),
                    GameMove::default_of('♟', "F7", "F5"),
                    GameMove::default_of('♙', "G2", "G4"),
                    GameMove::default_of('♟', "G7", "G5"),
                    GameMove::default_of('♙', "H2", "H4"),
                    GameMove::default_of('♟', "H7", "H5"),
                    GameMove::capture_of('♙', "B4", "A5"),
                    GameMove::capture_of('♟', "B5", "A4"),
                    GameMove::capture_of('♙', "D4", "C5"),
                    GameMove::capture_of('♟', "D5", "C4"),
                    GameMove::capture_of('♙', "E4", "F5"),
                    GameMove::capture_of('♟', "E5", "F4"),
                    GameMove::capture_of('♙', "G4", "H5"),
                    GameMove::capture_of('♟', "G5", "H4"),
                    GameMove::capture_of('♖', "A1", "A4"),
                    GameMove::capture_of('♜', "A8", "A5"),
                    GameMove::capture_of('♖', "H1", "H4"),
                    GameMove::capture_of('♜', "H8", "H5"),
                    GameMove::capture_of('♗', "C1", "F4"),
                    GameMove::capture_of('♝', "F8", "C5"),
                    GameMove::capture_of('♗', "F1", "C4"),
                    GameMove::capture_of('♝', "C8", "F5"),
                    GameMove::capture_of('♗', "F4", "B8"),
                    GameMove::capture_of('♝', "C5", "G1"),
                    GameMove::capture_of('♗', "C4", "G8"),
                    GameMove::capture_of('♝', "F5", "B1"),
                    GameMove::default_of('♖', "A4", "A1"),
                    GameMove::default_of('♜', "A5", "A8"),
                    GameMove::default_of('♖', "H4", "H1"),
                    GameMove::default_of('♜', "H5", "H8"),
                    GameMove::capture_of('♖', "A1", "B1"),
                    GameMove::capture_of('♜', "A8", "B8"),
                    GameMove::capture_of('♖', "H1", "G1"),
                    GameMove::capture_of('♜', "H8", "G8"),
                    GameMove::default_of('♔', "E1", "E2"),
                    GameMove::default_of('♚', "E8", "E7"),
                    GameMove::capture_of('♖', "G1", "G8"),
                    GameMove::capture_of('♜', "B8", "B1"),
                    GameMove::capture_of('♖', "G8", "D8"),
                    GameMove::capture_of('♜', "B1", "D1"),
                ],
            }
        );
    }
}
