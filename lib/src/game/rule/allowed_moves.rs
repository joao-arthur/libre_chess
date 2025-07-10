use std::collections::HashMap;

use crate::{
    color::Color,
    game::{
        GameBoard,
        game::{GameBounds, GameHistory},
        mov::{
            GameMovOld,
            default::default_moves,
            special::{castling::castling_moves, en_passant::en_passant_moves},
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
) -> Vec<GameMovOld> {
    if let Some(piece) = board.get(pos) {
        match piece.t {
            Type::Pawn => [
                default_moves(board, bounds, pos).into_iter().collect::<Vec<GameMovOld>>(),
                en_passant_moves(board, history, pos)
                    .into_iter()
                    .map(GameMovOld::from)
                    .collect::<Vec<GameMovOld>>(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            Type::King => {
                //     for (curr_color, curr_player) in game.players {
                //         if curr_color != player.color {
                //             moves.retain(|mov|  !curr_player.menace.contains(mov));
                //         }
                //     }
                [
                    default_moves(board, bounds, pos).into_iter().collect::<Vec<GameMovOld>>(),
                    castling_moves(board, bounds, history, pos)
                        .into_iter()
                        .map(GameMovOld::from)
                        .collect::<Vec<GameMovOld>>(),
                ]
                .into_iter()
                .flatten()
                .collect()
            }
            _ => default_moves(board, bounds, pos).into_iter().collect::<Vec<GameMovOld>>(),
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
) -> HashMap<Pos, Vec<GameMovOld>> {
    // is in check?

    let mut result = HashMap::new();
    for (pos, piece) in board {
        if &piece.color == color {
            let moves = allowed_moves_of_piece(board, bounds, history, pos);
            result.insert(pos.clone(), moves);
        }
    }
    result
}

#[cfg(test)]
mod tests {}
