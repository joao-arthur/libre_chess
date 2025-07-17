use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        board::GameBoard,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{
            GameMove,
            default::default_moves,
            special::{castling::castling_moves, en_passant::en_passant_moves},
        },
        player::PlayerMoves,
    },
    piece::PieceType,
};

pub fn pseudo_legal_moves_of_player(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    players: &GamePlayers,
    color: &Color,
) -> PlayerMoves {
    let mut result = HashMap::new();
    for (pos, piece) in board {
        if &piece.color == color {
            let mut moves = default_moves(board, bounds, pos);
            if piece.typ == PieceType::Pawn {
                moves.extend(en_passant_moves(board, history, pos));
            }
            if piece.typ == PieceType::King {
                moves.extend(castling_moves(board, history, players, pos));
            }
            result.insert(pos.clone(), moves);
        }
    }
    result
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

    use super::pseudo_legal_moves_of_player;

    #[test]
    fn pseudo_legal_moves_of_player_standard_moves() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "    ♚   ",
                "       ♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙       ",
                "    ♔   ",
            ],
        );
        let bounds = mode.bounds;
        let history = Vec::new();
        let players = HashMap::from([
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]);
        let color = Color::White;
        assert_eq!(
            pseudo_legal_moves_of_player(&board, &bounds, &history, &players, &color),
            HashMap::from([
                (
                    Pos::of("A2"),
                    HashMap::from([
                        (Pos::of("A3"), PieceMoveType::Default),
                        (Pos::of("A4"), PieceMoveType::Default),
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
        );
    }

    #[test]
    fn pseudo_legal_moves_of_player_special_moves() {
        let mode = standard_chess();
        let board = board_of_str(
            &mode.bounds,
            [
                "♜   ♚  ♜",
                "♟      ♟",
                "        ",
                "       ♝",
                "      ♟♙",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let bounds = mode.bounds;
        let history = vec![GameMove::default_of('♙', "H2", "H4")];
        let players = HashMap::from([
            (Color::Black, GamePlayer::from(Color::Black)),
            (Color::White, GamePlayer::from(Color::White)),
        ]);
        let color = Color::Black;
        assert_eq!(
            pseudo_legal_moves_of_player(&board, &bounds, &history, &players, &color),
            HashMap::from([
                (
                    Pos::of("A8"),
                    HashMap::from([
                        (Pos::of("B8"), PieceMoveType::Default),
                        (Pos::of("C8"), PieceMoveType::Default),
                        (Pos::of("D8"), PieceMoveType::Default),
                    ]),
                ),
                (
                    Pos::of("H8"),
                    HashMap::from([
                        (Pos::of("G8"), PieceMoveType::Default),
                        (Pos::of("F8"), PieceMoveType::Default),
                    ]),
                ),
                (
                    Pos::of("E8"),
                    HashMap::from([
                        (Pos::of("F8"), PieceMoveType::Default),
                        (Pos::of("F7"), PieceMoveType::Default),
                        (Pos::of("E7"), PieceMoveType::Default),
                        (Pos::of("D7"), PieceMoveType::Default),
                        (Pos::of("D8"), PieceMoveType::Default),
                        (Pos::of("H8"), PieceMoveType::ShortCastling),
                        (Pos::of("A8"), PieceMoveType::LongCastling),
                    ]),
                ),
                (
                    Pos::of("A7"),
                    HashMap::from([
                        (Pos::of("A6"), PieceMoveType::Default),
                        (Pos::of("A5"), PieceMoveType::Default),
                    ]),
                ),
                (Pos::of("H7"), HashMap::from([(Pos::of("H6"), PieceMoveType::Default),])),
                (
                    Pos::of("H5"),
                    HashMap::from([
                        (Pos::of("G6"), PieceMoveType::Default),
                        (Pos::of("F7"), PieceMoveType::Default),
                    ]),
                ),
                (
                    Pos::of("G4"),
                    HashMap::from([
                        (Pos::of("G3"), PieceMoveType::Default),
                        (Pos::of("H3"), PieceMoveType::EnPassant),
                    ]),
                ),
            ]),
        );
    }
}
