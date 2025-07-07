use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        GameBoard,
        game::{GameBounds, GameHistory},
        mov::{
            GameMov,
            default,
            special::{castling, en_passant},
        },
    },
    piece::Type,
    pos::Pos,
};

fn allowed_moves_of_piece(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    pos: &Pos,
) -> Vec<GameMov> {
    if let Some(piece) = board.get(pos) {
        match piece.t {
            Type::Pawn => [
                default::moves(board, bounds, pos)
                    .into_iter()
                    .map(GameMov::from)
                    .collect::<Vec<GameMov>>(),
                en_passant::moves(board, history, pos)
                    .into_iter()
                    .map(GameMov::from)
                    .collect::<Vec<GameMov>>(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            Type::King => [
                default::moves(board, bounds, pos)
                    .into_iter()
                    .map(GameMov::from)
                    .collect::<Vec<GameMov>>(),
                castling::moves(board, bounds, history, pos)
                    .into_iter()
                    .map(GameMov::from)
                    .collect::<Vec<GameMov>>(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            _ => default::moves(board, bounds, pos)
                .into_iter()
                .map(GameMov::from)
                .collect::<Vec<GameMov>>(),
        }
    } else {
        Vec::new()
    }
}

pub fn allowed_moves_of_player(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    color: &Color,
) -> HashMap<Pos, Vec<GameMov>> {
    let mut result = HashMap::new();
    for (pos, piece) in board {
        if &piece.color == color {
            let moves = allowed_moves_of_piece(board, bounds, history, pos);
            // if piece.t == Type::King {
            //     for (curr_color, curr_player) in game.players {
            //         if curr_color != player.color {
            //             moves.retain(|mov|  !curr_player.menace.contains(mov));
            //         }
            //     }
            // }
            result.insert(pos.clone(), moves);
        }
    }
    result
}

#[cfg(test)]
mod tests {}
