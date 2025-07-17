use crate::{
    game::{
        board::GameBoard,
        game::{GameHistory, GamePlayers},
        rule::turn::evaluate_turn,
    },
    piece::PieceType,
};

pub fn is_in_check(board: &GameBoard, players: &GamePlayers, history: &GameHistory) -> bool {
    let turn = evaluate_turn(history);
    let maybe_king =
        board.iter().find(|(_, piece)| piece.typ == PieceType::King && piece.color == turn);
    if let Some((king_pos, _)) = maybe_king {
        for player in players.values() {
            if player.color != turn {
                let moves_it = player.moves.iter();
                for (_, moves) in moves_it {
                    if moves.contains_key(&king_pos) {
                        return true;
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
        game::{
            board::board_of_str,
            mode::standard_chess,
            mov::{GameMove, PieceMoveType},
            player::GamePlayer,
        },
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
                            HashMap::from([
                                (Pos::of("F8"), PieceMoveType::Default),
                                (Pos::of("F7"), PieceMoveType::Default),
                                (Pos::of("D7"), PieceMoveType::Default),
                                (Pos::of("D8"), PieceMoveType::Default),
                            ]),
                        ),
                        (
                            Pos::of("E6"),
                            HashMap::from([
                                (Pos::of("E6"), PieceMoveType::Default),
                                (Pos::of("E5"), PieceMoveType::Default),
                            ]),
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
                            HashMap::from([
                                (Pos::of("F2"), PieceMoveType::Default),
                                (Pos::of("F1"), PieceMoveType::Default),
                                (Pos::of("D1"), PieceMoveType::Default),
                                (Pos::of("D2"), PieceMoveType::Default),
                            ]),
                        ),
                        (
                            Pos::of("E2"),
                            HashMap::from([
                                (Pos::of("E3"), PieceMoveType::Default),
                                (Pos::of("E4"), PieceMoveType::Default),
                            ]),
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
                            HashMap::from([
                                (Pos::of("F8"), PieceMoveType::Default),
                                (Pos::of("F7"), PieceMoveType::Default),
                                (Pos::of("D7"), PieceMoveType::Default),
                                (Pos::of("D8"), PieceMoveType::Default),
                            ]),
                        ),
                        (
                            Pos::of("E6"),
                            HashMap::from([
                                (Pos::of("E6"), PieceMoveType::Default),
                                (Pos::of("E5"), PieceMoveType::Default),
                            ]),
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
                            HashMap::from([
                                (Pos::of("D8"), PieceMoveType::Default),
                                (Pos::of("E8"), PieceMoveType::Default),
                            ]),
                        ),
                        (
                            Pos::of("E1"),
                            HashMap::from([
                                (Pos::of("F2"), PieceMoveType::Default),
                                (Pos::of("F1"), PieceMoveType::Default),
                                (Pos::of("D1"), PieceMoveType::Default),
                                (Pos::of("D2"), PieceMoveType::Default),
                                (Pos::of("E2"), PieceMoveType::Default),
                            ]),
                        ),
                    ]),
                },
            ),
        ]);
        assert!(is_in_check(&board, &players, &history));
    }
}
