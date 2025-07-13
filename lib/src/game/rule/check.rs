use crate::{
    game::{
        board::GameBoard,
        game::{Game, GameHistory, GamePlayers},
        mov::{GameMove, GameMoveType},
        rule::turn::evaluate_turn,
    },
    piece::PieceType,
};

pub fn is_in_check(board: &GameBoard, players: &GamePlayers, history: &GameHistory) -> bool {
    let curr_turn = evaluate_turn(&history);
    let maybe_king =
        board.iter().find(|(_, piece)| piece.typ == PieceType::King && piece.color == curr_turn);
    if let Some((king_pos, _)) = maybe_king {
        for player in players.values() {
            if player.color != curr_turn {
                let moves_it = player.moves.iter();
                for (_, moves) in moves_it {
                    for game_move in moves {
                        match game_move.typ {
                            GameMoveType::Default(_) | GameMoveType::Capture(_) => {
                                if &game_move.mov.to == king_pos {
                                    return true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        color::Color,
        game::{board::board_of_str, mode::standard_chess, mov::GameMove, player::GamePlayer},
        pos::Pos,
    };

    use super::is_in_check;

    #[test]
    fn is_in_check_false() {
        let mode = standard_chess();
        let bounds = mode.bounds;
        let board = board_of_str(
            &bounds,
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
        let history = Vec::new();
        let players = HashMap::from([
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
                                GameMove::default_of('♚', "E8", "D7"),
                                GameMove::default_of('♚', "E8", "D8"),
                                GameMove::menace_of('♚', "E8", "E7"),
                            ],
                        ),
                        (
                            Pos::of("E6"),
                            vec![
                                GameMove::default_of('♟', "E7", "E6"),
                                GameMove::default_of('♟', "E7", "E5"),
                                GameMove::menace_of('♟', "E7", "D6"),
                                GameMove::menace_of('♟', "E7", "F6"),
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
                            Pos::of("E1"),
                            vec![
                                GameMove::default_of('♔', "E1", "F2"),
                                GameMove::default_of('♔', "E1", "F1"),
                                GameMove::default_of('♔', "E1", "D1"),
                                GameMove::default_of('♔', "E1", "D2"),
                                GameMove::menace_of('♔', "E1", "E2"),
                            ],
                        ),
                        (
                            Pos::of("E2"),
                            vec![
                                GameMove::default_of('♙', "E2", "E3"),
                                GameMove::default_of('♙', "E2", "E4"),
                                GameMove::menace_of('♙', "E2", "D3"),
                                GameMove::menace_of('♙', "E2", "F3"),
                            ],
                        ),
                    ]),
                },
            ),
        ]);
        assert!(!is_in_check(&board, &players, &history));
    }

    #[test]
    fn is_in_check_true() {
        let mode = standard_chess();
        let bounds = mode.bounds;
        let board = board_of_str(
            &bounds,
            [
                "    ♚   ",
                "   ♙♟   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let history = vec![GameMove::default_of('♙', "D6", "D7")];
        let players = HashMap::from([
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
                                GameMove::capture_of('♚', "E8", "D7"),
                                GameMove::default_of('♚', "E8", "D8"),
                                GameMove::menace_of('♚', "E8", "E7"),
                            ],
                        ),
                        (
                            Pos::of("E6"),
                            vec![
                                GameMove::default_of('♟', "E7", "E6"),
                                GameMove::default_of('♟', "E7", "E5"),
                                GameMove::menace_of('♟', "E7", "D6"),
                                GameMove::menace_of('♟', "E7", "F6"),
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
                            Pos::of("D7"),
                            vec![
                                GameMove::default_of('♙', "D7", "D8"),
                                GameMove::capture_of('♙', "D7", "E8"),
                                GameMove::menace_of('♙', "D7", "C8"),
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
        assert!(is_in_check(&board, &players, &history));
    }
}
