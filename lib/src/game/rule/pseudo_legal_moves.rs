use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        board::GameBoard,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{
            default::default_moves, special::{castling::castling_moves, en_passant::en_passant_moves}, GameMove,
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
        game::{
            board::board_of_str, mode::standard_chess, mov::GameMove,
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
        )
    }

}
