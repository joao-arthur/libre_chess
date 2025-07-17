use crate::{
    game::{
        board::GameBoard,
        capture::GameCapture,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{GameMove, GameMoveType},
        rule::{allowed_moves::legal_moves_of_player, turn::evaluate_turn},
        selection::Selection,
    },
    mov::Mov,
    pos::Pos,
};

fn castling_move(board: &mut GameBoard, history: &mut GameHistory, game_move: &GameMove) {}

fn promotion_move(board: &mut GameBoard, history: &mut GameHistory, game_move: &GameMove) {}

pub fn move_piece(
    board: &mut GameBoard,
    history: &mut GameHistory,
    players: &mut GamePlayers,
    bounds: &GameBounds,
    selection: &Selection,
    to: &Pos,
) {
    let _: Option<()> = (|| {
        let turn = evaluate_turn(history);
        let from = selection.selected_pos.clone()?;
        let selected_piece = board.get(&from)?;
        let selected_player = players.get_mut(&selected_piece.color)?;
        if selected_player.color == turn {
            let selected_piece_moves = selected_player.moves.get(&from).cloned()?;
            let game_move = selected_piece_moves.iter().find(|game_move| {
                (game_move.typ == GameMoveType::Default
                    || game_move.typ == GameMoveType::Capture
                    || game_move.typ == GameMoveType::EnPassant
                    || game_move.typ == GameMoveType::LongCastling
                    || game_move.typ == GameMoveType::ShortCastling
                    || game_move.typ == GameMoveType::PromotionToQueen
                    || game_move.typ == GameMoveType::PromotionToKnight
                    || game_move.typ == GameMoveType::PromotionToRook
                    || game_move.typ == GameMoveType::PromotionToBishop)
                    && &game_move.mov.to == to
            })?;
            match game_move.typ {
                GameMoveType::Default | GameMoveType::Capture => {
                    let piece = board.remove(&from)?;
                    let maybe_captured_piece = board.insert(to.clone(), piece);
                    if let Some(captured_piece) = maybe_captured_piece {
                        selected_player
                            .captures
                            .push(GameCapture { piece: captured_piece, at: history.len() as u16 });
                        history.push(GameMove {
                            mov: Mov { from: from.clone(), to: to.clone(), piece },
                            typ: GameMoveType::Capture,
                        });
                        if let Some(affected_player) = players.get_mut(&captured_piece.color) {
                            affected_player.moves.remove(to);
                        }
                    } else {
                        history.push(GameMove {
                            mov: Mov { from: from.clone(), to: to.clone(), piece },
                            typ: GameMoveType::Default,
                        });
                    }
                }
                GameMoveType::EnPassant => {
                    let pawn = board.remove(&from)?;
                    board.insert(to.clone(), pawn);
                    let capture_pos = Pos { col: to.col, row: from.row };
                    if let Some(captured_piece) = board.remove(&capture_pos) {
                        selected_player
                            .captures
                            .push(GameCapture { piece: captured_piece, at: history.len() as u16 });
                        history.push(GameMove {
                            mov: Mov { from: from.clone(), to: to.clone(), piece: pawn },
                            typ: GameMoveType::EnPassant,
                        });
                        if let Some(affected_player) = players.get_mut(&captured_piece.color) {
                            affected_player.moves.remove(&capture_pos);
                        }
                    }
                }
                GameMoveType::ShortCastling => {
                    let king = board.remove(&from)?;
                    let new_king_pos = Pos { row: from.row, col: 6 };
                    board.insert(new_king_pos, king);
                    let rook = board.remove(to)?;
                    let new_rook_pos = Pos { row: from.row, col: 5 };
                    board.insert(new_rook_pos, rook);
                    history.push(GameMove {
                        mov: Mov { from: from.clone(), to: to.clone(), piece: king },
                        typ: GameMoveType::ShortCastling,
                    });
                }
                GameMoveType::LongCastling => {
                    let king = board.remove(&from)?;
                    let new_king_pos = Pos { row: from.row, col: 2 };
                    board.insert(new_king_pos, king);
                    let rook = board.remove(to)?;
                    let new_rook_pos = Pos { row: from.row, col: 3 };
                    board.insert(new_rook_pos, rook);
                    history.push(GameMove {
                        mov: Mov { from: from.clone(), to: to.clone(), piece: king },
                        typ: GameMoveType::LongCastling,
                    });
                }
                GameMoveType::PromotionToQueen => {
                    // board.insert(game_move.mov.to.clone(), promotion.piece);
                    // edit the pawn mov
                }
                GameMoveType::PromotionToKnight => {
                    // board.insert(game_move.mov.to.clone(), promotion.piece);
                    // edit the pawn mov
                }
                GameMoveType::PromotionToRook => {
                    // board.insert(game_move.mov.to.clone(), promotion.piece);
                    // edit the pawn mov
                }
                GameMoveType::PromotionToBishop => {
                    // board.insert(game_move.mov.to.clone(), promotion.piece);
                    // edit the pawn mov
                }
                GameMoveType::Menace => {}
            }
            let new_moves =
                legal_moves_of_player(board, bounds, history, &players.clone(), &turn);
            players.get_mut(&turn).unwrap().moves = new_moves;
        }
        None
    })();
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        color::Color,
        game::{
            board::board_of_str, capture::GameCapture, mode::standard_chess, mov::GameMove,
            player::GamePlayer, selection::Selection,
        },
        piece::Piece,
        pos::Pos,
    };

    use super::move_piece;

    #[test]
    fn move_piece_default_move() {
        let mode = standard_chess();
        let selection =
            Selection { selected_pos: Some(Pos::of("A2")), selected_squares: HashSet::new() };

        let mut board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "♟       ",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙       ",
                "    ♔   ",
            ],
        );
        let mut history = Vec::new();
        let mut players = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("E8"),
                            vec![
                                GameMove::default_of('♚', "E8", "F8"),
                                GameMove::default_of('♚', "E8", "F7"),
                                GameMove::default_of('♚', "E8", "E7"),
                                GameMove::default_of('♚', "E8", "D7"),
                                GameMove::default_of('♚', "E8", "D8"),
                            ],
                        ),
                        (
                            Pos::of("A7"),
                            vec![
                                GameMove::default_of('♟', "A7", "A6"),
                                GameMove::default_of('♟', "A7", "A5"),
                                GameMove::menace_of('♟', "A7", "D6"),
                                GameMove::menace_of('♟', "A7", "F6"),
                            ],
                        ),
                    ]),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("A2"),
                            vec![
                                GameMove::default_of('♙', "A2", "A3"),
                                GameMove::default_of('♙', "A2", "A4"),
                                GameMove::menace_of('♙', "A2", "B3"),
                            ],
                        ),
                        (
                            Pos::of("E1"),
                            vec![
                                GameMove::default_of('♔', "E1", "F2"),
                                GameMove::default_of('♔', "E1", "F1"),
                                GameMove::default_of('♔', "E1", "D1"),
                                GameMove::default_of('♔', "E1", "D2"),
                                GameMove::default_of('♔', "E1", "E2"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);

        move_piece(
            &mut board,
            &mut history,
            &mut players,
            &mode.bounds,
            &selection,
            &Pos::of("A4"),
        );

        let board_after = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "♟       ",
                "        ",
                "        ",
                "♙       ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let history_after = vec![GameMove::default_of('♙', "A2", "A4")];
        let players_after = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("E8"),
                            vec![
                                GameMove::default_of('♚', "E8", "F8"),
                                GameMove::default_of('♚', "E8", "F7"),
                                GameMove::default_of('♚', "E8", "E7"),
                                GameMove::default_of('♚', "E8", "D7"),
                                GameMove::default_of('♚', "E8", "D8"),
                            ],
                        ),
                        (
                            Pos::of("A7"),
                            vec![
                                GameMove::default_of('♟', "A7", "A6"),
                                GameMove::default_of('♟', "A7", "A5"),
                                GameMove::menace_of('♟', "A7", "D6"),
                                GameMove::menace_of('♟', "A7", "F6"),
                            ],
                        ),
                    ]),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("A4"),
                            vec![
                                GameMove::default_of('♙', "A4", "A5"),
                                GameMove::menace_of('♙', "A4", "B5"),
                            ],
                        ),
                        (
                            Pos::of("E1"),
                            vec![
                                GameMove::default_of('♔', "E1", "F2"),
                                GameMove::default_of('♔', "E1", "F1"),
                                GameMove::default_of('♔', "E1", "D1"),
                                GameMove::default_of('♔', "E1", "D2"),
                                GameMove::default_of('♔', "E1", "E2"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);

        assert_eq!(board, board_after);
        assert_eq!(history, history_after);
        assert_eq!(players, players_after);
    }

    #[test]
    fn move_piece_capture_move() {
        let mode = standard_chess();
        let selection =
            Selection { selected_pos: Some(Pos::of("E5")), selected_squares: HashSet::new() };

        let mut board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "   ♟    ",
                "    ♙   ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let mut history = Vec::new();
        let mut players = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("E8"),
                            vec![
                                GameMove::default_of('♚', "E8", "F8"),
                                GameMove::default_of('♚', "E8", "F7"),
                                GameMove::default_of('♚', "E8", "E7"),
                                GameMove::default_of('♚', "E8", "D7"),
                                GameMove::default_of('♚', "E8", "D8"),
                            ],
                        ),
                        (
                            Pos::of("D6"),
                            vec![
                                GameMove::default_of('♟', "D6", "D5"),
                                GameMove::menace_of('♟', "D6", "C5"),
                                GameMove::capture_of('♟', "D6", "E5"),
                            ],
                        ),
                    ]),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("E5"),
                            vec![
                                GameMove::default_of('♙', "E5", "E6"),
                                GameMove::capture_of('♙', "E5", "D6"),
                                GameMove::menace_of('♙', "E5", "F6"),
                            ],
                        ),
                        (
                            Pos::of("E1"),
                            vec![
                                GameMove::default_of('♔', "E1", "F2"),
                                GameMove::default_of('♔', "E1", "F1"),
                                GameMove::default_of('♔', "E1", "D1"),
                                GameMove::default_of('♔', "E1", "D2"),
                                GameMove::default_of('♔', "E1", "E2"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);

        move_piece(
            &mut board,
            &mut history,
            &mut players,
            &mode.bounds,
            &selection,
            &Pos::of("D6"),
        );

        let board_after = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "   ♙    ",
                "        ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let history_after = vec![GameMove::capture_of('♙', "E5", "D6")];
        let players_after = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([(
                        Pos::of("E8"),
                        vec![
                            GameMove::default_of('♚', "E8", "F8"),
                            GameMove::default_of('♚', "E8", "F7"),
                            GameMove::default_of('♚', "E8", "E7"),
                            GameMove::default_of('♚', "E8", "D7"),
                            GameMove::default_of('♚', "E8", "D8"),
                        ],
                    )]),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: vec![GameCapture { at: 0, piece: Piece::of('♟') }],
                    moves: HashMap::from([
                        (
                            Pos::of("D6"),
                            vec![
                                GameMove::default_of('♙', "D6", "D7"),
                                GameMove::menace_of('♙', "D6", "C7"),
                                GameMove::menace_of('♙', "D6", "E7"),
                            ],
                        ),
                        (
                            Pos::of("E1"),
                            vec![
                                GameMove::default_of('♔', "E1", "F2"),
                                GameMove::default_of('♔', "E1", "F1"),
                                GameMove::default_of('♔', "E1", "D1"),
                                GameMove::default_of('♔', "E1", "D2"),
                                GameMove::default_of('♔', "E1", "E2"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);

        assert_eq!(board, board_after);
        assert_eq!(history, history_after);
        assert_eq!(players, players_after);
    }

    #[test]
    fn move_piece_capture_en_passant_move() {
        let mode = standard_chess();
        let selection =
            Selection { selected_pos: Some(Pos::of("E5")), selected_squares: HashSet::new() };

        let mut board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "        ",
                "   ♟♙   ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let mut history = Vec::new();
        let mut players = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("E8"),
                            vec![
                                GameMove::default_of('♚', "E8", "F8"),
                                GameMove::default_of('♚', "E8", "F7"),
                                GameMove::default_of('♚', "E8", "E7"),
                                GameMove::default_of('♚', "E8", "D7"),
                                GameMove::default_of('♚', "E8", "D8"),
                            ],
                        ),
                        (
                            Pos::of("D5"),
                            vec![
                                GameMove::default_of('♟', "D5", "D5"),
                                GameMove::menace_of('♟', "D5", "C5"),
                                GameMove::menace_of('♟', "D5", "E5"),
                            ],
                        ),
                    ]),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("E5"),
                            vec![
                                GameMove::default_of('♙', "E5", "E6"),
                                GameMove::menace_of('♙', "E5", "D6"),
                                GameMove::menace_of('♙', "E5", "F6"),
                                GameMove::en_passant_of('♙', "E5", "D6"),
                            ],
                        ),
                        (
                            Pos::of("E1"),
                            vec![
                                GameMove::default_of('♔', "E1", "F2"),
                                GameMove::default_of('♔', "E1", "F1"),
                                GameMove::default_of('♔', "E1", "D1"),
                                GameMove::default_of('♔', "E1", "D2"),
                                GameMove::default_of('♔', "E1", "E2"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);

        move_piece(
            &mut board,
            &mut history,
            &mut players,
            &mode.bounds,
            &selection,
            &Pos::of("D6"),
        );

        let board_after = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "   ♙    ",
                "        ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let history_after = vec![GameMove::en_passant_of('♙', "E5", "D6")];
        let players_after = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([(
                        Pos::of("E8"),
                        vec![
                            GameMove::default_of('♚', "E8", "F8"),
                            GameMove::default_of('♚', "E8", "F7"),
                            GameMove::default_of('♚', "E8", "E7"),
                            GameMove::default_of('♚', "E8", "D7"),
                            GameMove::default_of('♚', "E8", "D8"),
                        ],
                    )]),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: vec![GameCapture { at: 0, piece: Piece::of('♟') }],
                    moves: HashMap::from([
                        (
                            Pos::of("D6"),
                            vec![
                                GameMove::default_of('♙', "D6", "D7"),
                                GameMove::menace_of('♙', "D6", "C7"),
                                GameMove::menace_of('♙', "D6", "E7"),
                            ],
                        ),
                        (
                            Pos::of("E1"),
                            vec![
                                GameMove::default_of('♔', "E1", "F2"),
                                GameMove::default_of('♔', "E1", "F1"),
                                GameMove::default_of('♔', "E1", "D1"),
                                GameMove::default_of('♔', "E1", "D2"),
                                GameMove::default_of('♔', "E1", "E2"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);

        assert_eq!(board, board_after);
        assert_eq!(history, history_after);
        assert_eq!(players, players_after);
    }

    #[test]
    fn move_piece_capture_short_castling_move() {
        let mode = standard_chess();
        let selection =
            Selection { selected_pos: Some(Pos::of("E1")), selected_squares: HashSet::new() };

        let mut board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "     ♙ ♙",
                "    ♔  ♖",
            ],
        );
        let mut history = Vec::new();
        let mut players = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([(
                        Pos::of("E8"),
                        vec![
                            GameMove::default_of('♚', "E8", "F8"),
                            GameMove::default_of('♚', "E8", "F7"),
                            GameMove::default_of('♚', "E8", "E7"),
                            GameMove::default_of('♚', "E8", "D7"),
                            GameMove::default_of('♚', "E8", "D8"),
                        ],
                    )]),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("F2"),
                            vec![
                                GameMove::default_of('♙', "F2", "F3"),
                                GameMove::default_of('♙', "F2", "F4"),
                                GameMove::menace_of('♙', "F2", "E3"),
                                GameMove::menace_of('♙', "F2", "G3"),
                            ],
                        ),
                        (
                            Pos::of("H2"),
                            vec![
                                GameMove::default_of('♙', "H2", "H3"),
                                GameMove::default_of('♙', "H2", "H4"),
                                GameMove::menace_of('♙', "H2", "G3"),
                            ],
                        ),
                        (
                            Pos::of("E1"),
                            vec![
                                GameMove::menace_of('♔', "E1", "F2"),
                                GameMove::default_of('♔', "E1", "F1"),
                                GameMove::default_of('♔', "E1", "D1"),
                                GameMove::default_of('♔', "E1", "D2"),
                                GameMove::default_of('♔', "E1", "E2"),
                                GameMove::short_castling_of('♔', "E1", "H1"),
                            ],
                        ),
                        (
                            Pos::of("H1"),
                            vec![
                                GameMove::default_of('♖', "H1", "G1"),
                                GameMove::default_of('♖', "H1", "F1"),
                                GameMove::menace_of('♖', "H1", "E1"),
                                GameMove::menace_of('♖', "H1", "E2"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);

        move_piece(
            &mut board,
            &mut history,
            &mut players,
            &mode.bounds,
            &selection,
            &Pos::of("H1"),
        );

        let board_after = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "     ♙ ♙",
                "     ♖♔ ",
            ],
        );
        let history_after = vec![GameMove::short_castling_of('♔', "E1", "H1")];
        let players_after = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([(
                        Pos::of("E8"),
                        vec![
                            GameMove::default_of('♚', "E8", "F8"),
                            GameMove::default_of('♚', "E8", "F7"),
                            GameMove::default_of('♚', "E8", "E7"),
                            GameMove::default_of('♚', "E8", "D7"),
                            GameMove::default_of('♚', "E8", "D8"),
                        ],
                    )]),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("F2"),
                            vec![
                                GameMove::default_of('♙', "F2", "F3"),
                                GameMove::default_of('♙', "F2", "F4"),
                                GameMove::menace_of('♙', "F2", "E3"),
                                GameMove::menace_of('♙', "F2", "G3"),
                            ],
                        ),
                        (
                            Pos::of("H2"),
                            vec![
                                GameMove::default_of('♙', "H2", "H3"),
                                GameMove::default_of('♙', "H2", "H4"),
                                GameMove::menace_of('♙', "H2", "G3"),
                            ],
                        ),
                        (
                            Pos::of("G1"),
                            vec![
                                GameMove::menace_of('♔', "G1", "H2"),
                                GameMove::default_of('♔', "G1", "H1"),
                                GameMove::menace_of('♔', "G1", "F1"),
                                GameMove::menace_of('♔', "G1", "F2"),
                                GameMove::default_of('♔', "G1", "G2"),
                            ],
                        ),
                        (
                            Pos::of("F1"),
                            vec![
                                GameMove::menace_of('♖', "F1", "G1"),
                                GameMove::default_of('♖', "F1", "E1"),
                                GameMove::default_of('♖', "F1", "D1"),
                                GameMove::default_of('♖', "F1", "C1"),
                                GameMove::default_of('♖', "F1", "B1"),
                                GameMove::default_of('♖', "F1", "A1"),
                                GameMove::menace_of('♖', "F1", "F2"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);

        assert_eq!(board, board_after);
        assert_eq!(history, history_after);
        assert_eq!(players, players_after);
    }

    #[test]
    fn move_piece_capture_long_castling_move() {
        let mode = standard_chess();
        let selection =
            Selection { selected_pos: Some(Pos::of("E1")), selected_squares: HashSet::new() };

        let mut board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙  ♙    ",
                "♖   ♔   ",
            ],
        );
        let mut history = Vec::new();
        let mut players = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([(
                        Pos::of("E8"),
                        vec![
                            GameMove::default_of('♚', "E8", "F8"),
                            GameMove::default_of('♚', "E8", "F7"),
                            GameMove::default_of('♚', "E8", "E7"),
                            GameMove::default_of('♚', "E8", "D7"),
                            GameMove::default_of('♚', "E8", "D8"),
                        ],
                    )]),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("A2"),
                            vec![
                                GameMove::default_of('♙', "A2", "A3"),
                                GameMove::default_of('♙', "A2", "A4"),
                                GameMove::menace_of('♙', "A2", "B3"),
                            ],
                        ),
                        (
                            Pos::of("D2"),
                            vec![
                                GameMove::default_of('♙', "D2", "D3"),
                                GameMove::default_of('♙', "D2", "D4"),
                                GameMove::menace_of('♙', "D2", "C3"),
                                GameMove::menace_of('♙', "D2", "E3"),
                            ],
                        ),
                        (
                            Pos::of("A1"),
                            vec![
                                GameMove::default_of('♖', "A1", "B1"),
                                GameMove::default_of('♖', "A1", "C1"),
                                GameMove::default_of('♖', "A1", "D1"),
                                GameMove::menace_of('♖', "A1", "E1"),
                                GameMove::menace_of('♖', "A1", "A2"),
                            ],
                        ),
                        (
                            Pos::of("E1"),
                            vec![
                                GameMove::default_of('♔', "E1", "F2"),
                                GameMove::default_of('♔', "E1", "F1"),
                                GameMove::default_of('♔', "E1", "D1"),
                                GameMove::menace_of('♔', "E1", "D2"),
                                GameMove::default_of('♔', "E1", "E2"),
                                GameMove::long_castling_of('♔', "E1", "A1"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);

        move_piece(
            &mut board,
            &mut history,
            &mut players,
            &mode.bounds,
            &selection,
            &Pos::of("A1"),
        );

        let board_after = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙  ♙    ",
                "  ♔♖    ",
            ],
        );
        let history_after = vec![GameMove::long_castling_of('♔', "E1", "A1")];
        let players_after = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: HashMap::from([(
                        Pos::of("E8"),
                        vec![
                            GameMove::default_of('♚', "E8", "F8"),
                            GameMove::default_of('♚', "E8", "F7"),
                            GameMove::default_of('♚', "E8", "E7"),
                            GameMove::default_of('♚', "E8", "D7"),
                            GameMove::default_of('♚', "E8", "D8"),
                        ],
                    )]),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: HashMap::from([
                        (
                            Pos::of("A2"),
                            vec![
                                GameMove::default_of('♙', "A2", "A3"),
                                GameMove::default_of('♙', "A2", "A4"),
                                GameMove::menace_of('♙', "A2", "B3"),
                            ],
                        ),
                        (
                            Pos::of("D2"),
                            vec![
                                GameMove::default_of('♙', "D2", "D3"),
                                GameMove::default_of('♙', "D2", "D4"),
                                GameMove::menace_of('♙', "D2", "C3"),
                                GameMove::menace_of('♙', "D2", "E3"),
                            ],
                        ),
                        (
                            Pos::of("C1"),
                            vec![
                                GameMove::menace_of('♔', "C1", "D2"),
                                GameMove::menace_of('♔', "C1", "D1"),
                                GameMove::default_of('♔', "C1", "B1"),
                                GameMove::default_of('♔', "C1", "B2"),
                                GameMove::default_of('♔', "C1", "C2"),
                            ],
                        ),
                        (
                            Pos::of("D1"),
                            vec![
                                GameMove::default_of('♖', "D1", "E1"),
                                GameMove::default_of('♖', "D1", "F1"),
                                GameMove::default_of('♖', "D1", "G1"),
                                GameMove::default_of('♖', "D1", "H1"),
                                GameMove::menace_of('♖', "D1", "C1"),
                                GameMove::menace_of('♖', "D1", "D2"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);

        assert_eq!(board, board_after);
        assert_eq!(history, history_after);
        assert_eq!(players, players_after);
    }
}
