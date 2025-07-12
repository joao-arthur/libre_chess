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

fn allowed_moves_of_piece(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    players: &GamePlayers,
    pos: &Pos,
) -> Vec<GameMove> {
    if let Some(piece) = board.get(pos) {
        match piece.typ {
            PieceType::Pawn => [
                default_moves(board, bounds, pos).into_iter().collect::<Vec<GameMove>>(),
                en_passant_moves(board, history, pos)
                    .into_iter()
                    .map(GameMove::from)
                    .collect::<Vec<GameMove>>(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            PieceType::King => {
                //     for (curr_color, curr_player) in game.players {
                //         if curr_color != player.color {
                //             moves.retain(|mov|  !curr_player.menace.contains(mov));
                //         }
                //     }
                [
                    default_moves(board, bounds, pos).into_iter().collect::<Vec<GameMove>>(),
                    castling_moves(board, history, players, pos)
                        .into_iter()
                        .map(GameMove::from)
                        .collect::<Vec<GameMove>>(),
                ]
                .into_iter()
                .flatten()
                .collect()
            }
            _ => default_moves(board, bounds, pos).into_iter().collect::<Vec<GameMove>>(),
        }
    } else {
        Vec::new()
    }
}

pub fn allowed_moves_of_player(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    players: &GamePlayers,
    color: &Color,
) -> HashMap<Pos, Vec<GameMove>> {
    // is in check?
    let mut result = HashMap::new();
    for (pos, piece) in board {
        if &piece.color == color {
            let moves = allowed_moves_of_piece(board, bounds, history, players, pos);
            result.insert(pos.clone(), moves);
        }
    }
    result
}

#[cfg(test)]
mod tests {}
