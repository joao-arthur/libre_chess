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
    },
    piece::PieceType,
    pos::Pos,
};

pub fn pseudo_legal_moves_of_player(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    players: &GamePlayers,
    color: &Color,
) -> HashMap<Pos, Vec<GameMove>> {
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
        game::{board::board_of_str, mode::standard_chess, mov::GameMove, player::GamePlayer},
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
            (
                Color::Black,
                GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
            ),
            (
                Color::White,
                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
            ),
        ]);
        let color = Color::White;
        assert_eq!(
            pseudo_legal_moves_of_player(&board, &bounds, &history, &players, &color),
            HashMap::from([
                (
                    Pos::of("A2"),
                    vec![
                        GameMove::default_of('♙', "A2", "A3"),
                        GameMove::default_of('♙', "A2", "A4"),
                        GameMove::menace_of('♙', "A2", "B3"),
                    ]
                ),
                (
                    Pos::of("E1"),
                    vec![
                        GameMove::default_of('♔', "E1", "F2"),
                        GameMove::default_of('♔', "E1", "F1"),
                        GameMove::default_of('♔', "E1", "D1"),
                        GameMove::default_of('♔', "E1", "D2"),
                        GameMove::default_of('♔', "E1", "E2"),
                    ]
                ),
            ])
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
                    vec![
                        GameMove::default_of('♜', "A8", "B8"),
                        GameMove::default_of('♜', "A8", "C8"),
                        GameMove::default_of('♜', "A8", "D8"),
                        GameMove::menace_of('♜', "A8", "E8"),
                        GameMove::menace_of('♜', "A8", "A7"),
                    ]
                ),
                (
                    Pos::of("H8"),
                    vec![
                        GameMove::menace_of('♜', "H8", "H7"),
                        GameMove::default_of('♜', "H8", "G8"),
                        GameMove::default_of('♜', "H8", "F8"),
                        GameMove::menace_of('♜', "H8", "E8"),
                    ]
                ),
                (
                    Pos::of("E8"),
                    vec![
                        GameMove::default_of('♚', "E8", "F8"),
                        GameMove::default_of('♚', "E8", "F7"),
                        GameMove::default_of('♚', "E8", "E7"),
                        GameMove::default_of('♚', "E8", "D7"),
                        GameMove::default_of('♚', "E8", "D8"),
                        GameMove::short_castling_of('♚', "E8", "H8"),
                        GameMove::long_castling_of('♚', "E8", "A8"),
                    ]
                ),
                (
                    Pos::of("A7"),
                    vec![
                        GameMove::default_of('♟', "A7", "A6"),
                        GameMove::default_of('♟', "A7", "A5"),
                        GameMove::menace_of('♟', "A7", "B6"),
                    ]
                ),
                (
                    Pos::of("H7"),
                    vec![
                        GameMove::default_of('♟', "H7", "H6"),
                        GameMove::menace_of('♟', "H7", "G6"),
                    ]
                ),
                (
                    Pos::of("H5"),
                    vec![
                        GameMove::menace_of('♝', "H5", "G4"),
                        GameMove::default_of('♝', "H5", "G6"),
                        GameMove::default_of('♝', "H5", "F7"),
                        GameMove::menace_of('♝', "H5", "E8"),
                    ]
                ),
                (
                    Pos::of("G4"),
                    vec![
                        GameMove::default_of('♟', "G4", "G3"),
                        GameMove::menace_of('♟', "G4", "F3"),
                        GameMove::menace_of('♟', "G4", "H3"),
                        GameMove::en_passant_of('♟', "G4", "H3"),
                    ]
                ),
            ])
        );
    }
}
