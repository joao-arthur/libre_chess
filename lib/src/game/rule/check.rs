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
                for (_, moves) in player.moves.iter() {
                    if moves.contains_key(king_pos) {
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

    use crate::{
        color::Color,
        game::{
            board::board_of_str,
            mode::standard_chess,
            mov::{GameMove, PieceMoveType},
            player::GamePlayer,
        },
        pos::pos_of,
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
        let players = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("E8"),
                            [
                                (pos_of("F8"), PieceMoveType::Default),
                                (pos_of("F7"), PieceMoveType::Default),
                                (pos_of("D7"), PieceMoveType::Default),
                                (pos_of("D8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("E6"),
                            [
                                (pos_of("E6"), PieceMoveType::Default),
                                (pos_of("E5"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("E1"),
                            [
                                (pos_of("F2"), PieceMoveType::Default),
                                (pos_of("F1"), PieceMoveType::Default),
                                (pos_of("D1"), PieceMoveType::Default),
                                (pos_of("D2"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("E2"),
                            [
                                (pos_of("E3"), PieceMoveType::Default),
                                (pos_of("E4"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
        ]
        .into();
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
        let players = [
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("E8"),
                            [
                                (pos_of("F8"), PieceMoveType::Default),
                                (pos_of("F7"), PieceMoveType::Default),
                                (pos_of("D7"), PieceMoveType::Default),
                                (pos_of("D8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("E6"),
                            [
                                (pos_of("E6"), PieceMoveType::Default),
                                (pos_of("E5"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: [
                        (
                            pos_of("D7"),
                            [
                                (pos_of("D8"), PieceMoveType::Default),
                                (pos_of("E8"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                        (
                            pos_of("E1"),
                            [
                                (pos_of("F2"), PieceMoveType::Default),
                                (pos_of("F1"), PieceMoveType::Default),
                                (pos_of("D1"), PieceMoveType::Default),
                                (pos_of("D2"), PieceMoveType::Default),
                                (pos_of("E2"), PieceMoveType::Default),
                            ]
                            .into(),
                        ),
                    ]
                    .into(),
                },
            ),
        ]
        .into();
        assert!(is_in_check(&board, &players, &history));
    }
}
