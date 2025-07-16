use crate::{
    game::{
        board::GameBoard,
        capture::GameCapture,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{GameMove, GameMoveType},
        player::GamePlayer,
        rule::{allowed_moves::allowed_moves_of_player, turn::evaluate_turn},
        selection::Selection,
    }, mov::Mov, pos::Pos
};

fn default_move(
    board: &mut GameBoard,
    player: &mut GamePlayer,
    history: &mut GameHistory,
    game_move: &GameMove,
) {

}

fn en_passant_move(
    board: &mut GameBoard,
    player: &mut GamePlayer,
    history: &mut GameHistory,
    game_move: &GameMove,
) {
    if let Some(piece) = board.remove(&game_move.mov.from) {
        board.insert(game_move.mov.to.clone(), piece);
        if let Some(captured) =
            board.remove(&Pos { col: game_move.mov.to.col, row: game_move.mov.from.row })
        {
            player.captures.push(GameCapture { piece: captured, at: history.len() as u16 });
        }
    }
    history.push(game_move.clone());
}

fn castling_move(board: &mut GameBoard, history: &mut GameHistory, game_move: &GameMove) {
    if let Some(piece) = board.remove(&game_move.mov.from) {
        board.insert(game_move.mov.to.clone(), piece);
        if game_move.mov.to.col > game_move.mov.from.col {
            if let Some(rook) =
                board.remove(&Pos { col: game_move.mov.to.col + 1, row: game_move.mov.to.row })
            {
                board
                    .insert(Pos { col: game_move.mov.to.col - 1, row: game_move.mov.to.row }, rook);
            }
        } else if let Some(rook) =
            board.remove(&Pos { col: game_move.mov.to.col - 1, row: game_move.mov.to.row })
        {
            board.insert(Pos { col: game_move.mov.to.col + 1, row: game_move.mov.to.row }, rook);
        }
    }
    history.push(game_move.clone());
}

fn promotion_move(board: &mut GameBoard, history: &mut GameHistory, game_move: &GameMove) {
    // match type
    // board.insert(game_move.mov.to.clone(), promotion.piece);
    // edit the pawn mov
}

pub fn move_piece(
    board: &mut GameBoard,
    history: &mut GameHistory,
    players: &mut GamePlayers,
    bounds: &GameBounds,
    selection: &Selection,
    pos: &Pos,
) {
    let mut changed = false;
    let turn = evaluate_turn(history);
    if let Some(selected_pos) = &selection.selected_pos {
        if let Some(selected_piece) = board.get(selected_pos) {
            if let Some(selected_player) = players.get_mut(&selected_piece.color) {
                if selected_player.color == turn {
                    if let Some(selected_piece_moves) =
                        selected_player.moves.get(&selected_pos).cloned()
                    {
                        let maybe_move = selected_piece_moves.iter().find(|game_move| {
                            (game_move.typ == GameMoveType::Default
                                || game_move.typ == GameMoveType::Capture
                                || game_move.typ == GameMoveType::EnPassant
                                || game_move.typ == GameMoveType::LongCastling
                                || game_move.typ == GameMoveType::ShortCastling
                                || game_move.typ == GameMoveType::PromotionToQueen
                                || game_move.typ == GameMoveType::PromotionToKnight
                                || game_move.typ == GameMoveType::PromotionToRook
                                || game_move.typ == GameMoveType::PromotionToBishop)
                                && &game_move.mov.to == pos
                        });
                        if let Some(game_move) = maybe_move {
                            match game_move.typ {
                                GameMoveType::Default | GameMoveType::Capture => {
                                    changed = true;
                                    if let Some(piece) = board.remove(&game_move.mov.from) {
                                        let maybe_captured = board.insert(game_move.mov.to.clone(), piece);
                                        if let Some(captured) = maybe_captured {
                                            selected_player.captures.push(GameCapture { piece: captured, at: history.len() as u16 });
                                            history.push(
                                                GameMove {
                                                    mov: Mov {
                                                        from: selected_pos.clone(),
                                                        to: pos.clone(),
                                                        piece
                                                    },
                                                    typ: GameMoveType::Capture
                                                }
                                            );
                                        } else {
                                            history.push(
                                                GameMove {
                                                    mov: Mov {
                                                        from: selected_pos.clone(),
                                                        to: pos.clone(),
                                                        piece
                                                    }, 
                                                    typ: GameMoveType::Default
                                                });
                                        }
                                    }
                                }
                                GameMoveType::EnPassant => {
                                    changed = true;
                                    en_passant_move(board, selected_player, history, game_move)
                                }
                                GameMoveType::LongCastling | GameMoveType::ShortCastling => {
                                    changed = true;
                                    castling_move(board, history, game_move)
                                }
                                GameMoveType::PromotionToQueen
                                | GameMoveType::PromotionToKnight
                                | GameMoveType::PromotionToRook
                                | GameMoveType::PromotionToBishop => {
                                    changed = true;
                                    promotion_move(board, history, game_move)
                                }
                                GameMoveType::Menace => {}
                            }
                        }
                    }
                }
            }
        }
    }
    if changed {
        let new_moves = allowed_moves_of_player(
            board,
            bounds,
            history,
            &players.clone(),
            &turn,
        );
        players.get_mut(&turn).unwrap().moves = new_moves;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        color::Color,
        game::{
            board::board_of_str, mode::standard_chess, mov::GameMove, player::GamePlayer,
            selection::Selection,
        },
        pos::Pos,
    };

    use super::move_piece;

    #[test]
    fn test_move_piece_default_move() {
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
}
