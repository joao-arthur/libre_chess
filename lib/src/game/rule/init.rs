use std::collections::HashMap;

use crate::{
    color::PieceColor,
    game::{
        Game,
        game::GameHistory,
        mode::GameMode,
        player::GamePlayer,
        rule::{allowed_moves::allowed_moves_of_player, turn::evaluate_turn},
    },
};

pub fn game_of_mode(mode: GameMode) -> Game {
    let board = mode.initial_board;
    let bounds = mode.bounds;
    let history = Vec::new();
    let players = HashMap::from([
        (
            PieceColor::Black,
            GamePlayer {
                color: PieceColor::Black,
                captures: Vec::new(),
                moves: allowed_moves_of_player(&board, &bounds, &history, &PieceColor::Black),
            },
        ),
        (
            PieceColor::White,
            GamePlayer {
                color: PieceColor::White,
                captures: Vec::new(),
                moves: allowed_moves_of_player(&board, &bounds, &history, &PieceColor::White),
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
        if let Some(piece) = board.remove(&mov.from) {
            board.insert(mov.to.clone(), piece);
        }
    }
    let turn = evaluate_turn(&history);
    let players = HashMap::from([
        (
            PieceColor::Black,
            GamePlayer {
                color: PieceColor::Black,
                captures: Vec::new(),
                moves: if turn == PieceColor::Black {
                    allowed_moves_of_player(&board, &bounds, &history, &PieceColor::Black)
                } else {
                    HashMap::new()
                },
            },
        ),
        (
            PieceColor::White,
            GamePlayer {
                color: PieceColor::White,
                captures: Vec::new(),
                moves: if turn == PieceColor::White {
                    allowed_moves_of_player(&board, &bounds, &history, &PieceColor::White)
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
        color::PieceColor,
        game::{
            Game,
            board::{board_of_str, board_to_string},
            game::GameBounds,
            mode::standard_chess,
            mov::{CaptureMovOld, DefaultMovOld, GameMovOld, game_move_vec_to_string},
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
        let black = game.players.get(&PieceColor::Black).unwrap();
        assert_eq!(black.color, PieceColor::Black);
        assert_eq!(black.captures, Vec::new());
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("A7")).unwrap()),
            "".to_owned()
                + "        \n"
                + "♟       \n"
                + "●○      \n"
                + "●       \n"
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
                + "○●○     \n"
                + " ●      \n"
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
                + " ○●○    \n"
                + "  ●     \n"
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
                + "  ○●○   \n"
                + "   ●    \n"
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
                + "   ○●○  \n"
                + "    ●   \n"
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
                + "    ○●○ \n"
                + "     ●  \n"
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
                + "     ○●○\n"
                + "      ● \n"
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
                + "      ○●\n"
                + "       ●\n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("A8")).unwrap()),
            "".to_owned()
                + "♜○      \n"
                + "○       \n"
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
                + "   ○    \n"
                + "● ●     \n"
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
                + " ○ ○    \n"
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
                + "  ○♛○   \n"
                + "  ○○○   \n"
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
                + "   ○♚○  \n"
                + "   ○○○  \n"
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
                + "    ○ ○ \n"
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
                + "    ○   \n"
                + "     ● ●\n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, black.moves.get(&Pos::of_str("H8")).unwrap()),
            "".to_owned()
                + "      ○♜\n"
                + "       ○\n"
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
        let white = game.players.get(&PieceColor::White).unwrap();
        assert_eq!(white.color, PieceColor::White);
        assert_eq!(white.captures, Vec::new());
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("A2")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "●       \n"
                + "●○      \n"
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
                + " ●      \n"
                + "○●○     \n"
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
                + "  ●     \n"
                + " ○●○    \n"
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
                + "   ●    \n"
                + "  ○●○   \n"
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
                + "    ●   \n"
                + "   ○●○  \n"
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
                + "     ●  \n"
                + "    ○●○ \n"
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
                + "      ● \n"
                + "     ○●○\n"
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
                + "       ●\n"
                + "      ○●\n"
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
                + "○       \n"
                + "♖○      \n"
        );
        assert_eq!(
            game_move_vec_to_string(&mode.bounds, white.moves.get(&Pos::of_str("B1")).unwrap()),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "● ●     \n"
                + "   ○    \n"
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
                + " ○ ○    \n"
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
                + "  ○○○   \n"
                + "  ○♕○   \n"
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
                + "   ○○○  \n"
                + "   ○♔○  \n"
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
                + "    ○ ○ \n"
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
                + "     ● ●\n"
                + "    ○   \n"
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
                + "       ○\n"
                + "      ○♖\n"
        );
    }

    #[test]
    fn game_of_mode_and_history_standard_chess() {
        let mode = standard_chess();
        let history = vec![
            Mov::of('♙', "A2", "A4"),
            Mov::of('♟', "A7", "A5"),
            Mov::of('♙', "B2", "B4"),
            Mov::of('♟', "B7", "B5"),
            Mov::of('♙', "C2", "C4"),
            Mov::of('♟', "C7", "C5"),
            Mov::of('♙', "D2", "D4"),
            Mov::of('♟', "D7", "D5"),
            Mov::of('♙', "E2", "E4"),
            Mov::of('♟', "E7", "E5"),
            Mov::of('♙', "F2", "F4"),
            Mov::of('♟', "F7", "F5"),
            Mov::of('♙', "G2", "G4"),
            Mov::of('♟', "G7", "G5"),
            Mov::of('♙', "H2", "H4"),
            Mov::of('♟', "H7", "H5"),
            //
            Mov::of('♙', "B4", "A5"),
            Mov::of('♟', "B5", "A4"),
            Mov::of('♙', "D4", "C5"),
            Mov::of('♟', "D5", "C4"),
            Mov::of('♙', "E4", "F5"),
            Mov::of('♟', "E5", "F4"),
            Mov::of('♙', "G4", "H5"),
            Mov::of('♟', "G5", "H4"),
            //
            Mov::of('♖', "A1", "A4"),
            Mov::of('♜', "A8", "A5"),
            Mov::of('♖', "H1", "H4"),
            Mov::of('♜', "H8", "H5"),
            //
            Mov::of('♗', "C1", "F4"),
            Mov::of('♝', "F8", "C5"),
            Mov::of('♗', "F1", "C4"),
            Mov::of('♝', "C8", "F5"),
            Mov::of('♗', "F4", "B8"),
            Mov::of('♝', "C5", "G1"),
            Mov::of('♗', "C4", "G8"),
            Mov::of('♝', "F5", "B1"),
            //
            Mov::of('♖', "A4", "A1"),
            Mov::of('♜', "A5", "A8"),
            Mov::of('♖', "H4", "H1"),
            Mov::of('♜', "H5", "H8"),
            Mov::of('♖', "A1", "B1"),
            Mov::of('♜', "A8", "B8"),
            Mov::of('♖', "H1", "G1"),
            Mov::of('♜', "H8", "G8"),
            //
            Mov::of('♔', "E1", "E2"),
            Mov::of('♚', "E8", "E7"),
            //
            Mov::of('♖', "G1", "G8"),
            Mov::of('♜', "B8", "B1"),
            Mov::of('♖', "G8", "D8"),
            Mov::of('♜', "B1", "D1"),
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
                        PieceColor::Black,
                        GamePlayer {
                            color: PieceColor::Black,
                            captures: Vec::new(),
                            moves: HashMap::new(),
                        },
                    ),
                    (
                        PieceColor::White,
                        GamePlayer {
                            color: PieceColor::White,
                            captures: Vec::new(),
                            moves: HashMap::from([
                                (
                                    Pos::of_str("E2"),
                                    vec![
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♔', "E2", "F3"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♔', "E2", "F2"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♔', "E2", "F1"
                                        ))), //////
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♔', "E2", "E1"
                                        ))), //////
                                        GameMovOld::from(CaptureMovOld::from(Mov::of(
                                            '♔', "E2", "D1"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♔', "E2", "D2"
                                        ))), //////
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♔', "E2", "D3"
                                        ))), //////
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♔', "E2", "E3"
                                        ))),
                                    ]
                                ),
                                (
                                    Pos::of_str("D8"),
                                    vec![
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "E8"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "F8"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "G8"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "H8"
                                        ))),
                                        //
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "D7"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "D6"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "D5"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "D4"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "D3"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "D2"
                                        ))),
                                        GameMovOld::from(CaptureMovOld::from(Mov::of(
                                            '♖', "D8", "D1"
                                        ))),
                                        //
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "C8"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "B8"
                                        ))),
                                        GameMovOld::from(DefaultMovOld::from(Mov::of(
                                            '♖', "D8", "A8"
                                        ))),
                                    ]
                                ),
                            ]),
                        },
                    ),
                ]),
                history: vec![
                    Mov::of('♙', "A2", "A4"),
                    Mov::of('♟', "A7", "A5"),
                    Mov::of('♙', "B2", "B4"),
                    Mov::of('♟', "B7", "B5"),
                    Mov::of('♙', "C2", "C4"),
                    Mov::of('♟', "C7", "C5"),
                    Mov::of('♙', "D2", "D4"),
                    Mov::of('♟', "D7", "D5"),
                    Mov::of('♙', "E2", "E4"),
                    Mov::of('♟', "E7", "E5"),
                    Mov::of('♙', "F2", "F4"),
                    Mov::of('♟', "F7", "F5"),
                    Mov::of('♙', "G2", "G4"),
                    Mov::of('♟', "G7", "G5"),
                    Mov::of('♙', "H2", "H4"),
                    Mov::of('♟', "H7", "H5"),
                    //
                    Mov::of('♙', "B4", "A5"),
                    Mov::of('♟', "B5", "A4"),
                    Mov::of('♙', "D4", "C5"),
                    Mov::of('♟', "D5", "C4"),
                    Mov::of('♙', "E4", "F5"),
                    Mov::of('♟', "E5", "F4"),
                    Mov::of('♙', "G4", "H5"),
                    Mov::of('♟', "G5", "H4"),
                    //
                    Mov::of('♖', "A1", "A4"),
                    Mov::of('♜', "A8", "A5"),
                    Mov::of('♖', "H1", "H4"),
                    Mov::of('♜', "H8", "H5"),
                    //
                    Mov::of('♗', "C1", "F4"),
                    Mov::of('♝', "F8", "C5"),
                    Mov::of('♗', "F1", "C4"),
                    Mov::of('♝', "C8", "F5"),
                    //
                    Mov::of('♗', "F4", "B8"),
                    Mov::of('♝', "C5", "G1"),
                    Mov::of('♗', "C4", "G8"),
                    Mov::of('♝', "F5", "B1"),
                    //
                    Mov::of('♖', "A4", "A1"),
                    Mov::of('♜', "A5", "A8"),
                    Mov::of('♖', "H4", "H1"),
                    Mov::of('♜', "H5", "H8"),
                    //
                    Mov::of('♖', "A1", "B1"),
                    Mov::of('♜', "A8", "B8"),
                    Mov::of('♖', "H1", "G1"),
                    Mov::of('♜', "H8", "G8"),
                    //
                    Mov::of('♔', "E1", "E2"),
                    Mov::of('♚', "E8", "E7"),
                    //
                    Mov::of('♖', "G1", "G8"),
                    Mov::of('♜', "B8", "B1"),
                    Mov::of('♖', "G8", "D8"),
                    Mov::of('♜', "B1", "D1"),
                ],
            }
        );
    }
}
