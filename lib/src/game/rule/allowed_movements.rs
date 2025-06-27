use std::collections::HashMap;

use crate::{
    board::pos::Pos,
    color::Color,
    game::{
        Game, GameBoard,
        game::{GameBounds, GameHistory},
        movement::{
            default,
            movement::GameMovement,
            special::{castling, en_passant},
        },
        player::GamePlayer,
    },
    piece::Type,
};

fn allowed_movements_of_piece(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    pos: &Pos,
) -> Vec<GameMovement> {
    if let Some(piece) = board.get(pos) {
        match piece.t {
            Type::Pawn => [
                default::movements(&board, &bounds, pos)
                    .into_iter()
                    .map(GameMovement::from)
                    .collect::<Vec<GameMovement>>(),
                en_passant::movements(&board, &history, pos)
                    .into_iter()
                    .map(GameMovement::from)
                    .collect::<Vec<GameMovement>>(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            Type::King => [
                default::movements(&board, &bounds, pos)
                    .into_iter()
                    .map(GameMovement::from)
                    .collect::<Vec<GameMovement>>(),
                castling::movements(&board, &bounds, &history, pos)
                    .into_iter()
                    .map(GameMovement::from)
                    .collect::<Vec<GameMovement>>(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            _ => default::movements(&board, &bounds, pos)
                .into_iter()
                .map(GameMovement::from)
                .collect::<Vec<GameMovement>>(),
        }
    } else {
        Vec::new()
    }
}

pub fn allowed_movements_of_player(
    board: &GameBoard,
    bounds: &GameBounds,
    history: &GameHistory,
    color: &Color,
) -> HashMap<Pos, Vec<GameMovement>> {
    let mut result = HashMap::new();
    for (pos, piece) in board {
        if &piece.color == color {
            let mut movements = allowed_movements_of_piece(board, bounds, history, pos);
            // if piece.t == Type::King {
            //     for (curr_color, curr_player) in game.players {
            //         if curr_color != player.color {
            //             movements.retain(|mov|  !curr_player.menace.contains(mov));
            //         }
            //     }
            // }
            result.insert(pos.clone(), movements);
        }
    }
    result
}

#[cfg(test)]
mod tests {}
