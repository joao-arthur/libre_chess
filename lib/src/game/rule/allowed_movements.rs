use std::collections::HashMap;

use crate::{
    board::pos::Pos,
    color::Color,
    game::{
        movement::{
            naive,
            special::{
                castling::{
                    black_king_can_long_castling, black_king_can_short_castling,
                    white_king_can_long_castling, white_king_can_short_castling,
                },
                en_passant::en_passant,
            }, Movement,
        }, rule::turn::evaluate_turn, Game
    },
    piece::Type,
};

pub fn allowed_movements_of_piece(game: &Game, pos: &Pos) -> Vec<Pos> {
    if let Some(piece) = game.board.get(pos) {
        let curr_turn = evaluate_turn(game);
        if piece.color != curr_turn {
            return Vec::new();
        }
        let mut naive_movements = naive::movements_of_piece(&game.board, &game.bounds, pos);
        if piece.t == Type::King {
            for player in game.players.values() {
                if player.color != curr_turn {
                    naive_movements.retain(|mov| !player.menace.contains(mov));
                }
            }
            match piece.color {
                Color::White => {
                    if white_king_can_short_castling(&game.board, &game.history) {
                        naive_movements.push(Pos::of_str("G1"));
                    }
                    if white_king_can_long_castling(&game.board, &game.history) {
                        naive_movements.push(Pos::of_str("B1"));
                    }
                }
                Color::Black => {
                    if black_king_can_short_castling(&game.board, &game.history) {
                        naive_movements.push(Pos::of_str("G8"));
                    }
                    if black_king_can_long_castling(&game.board, &game.history) {
                        naive_movements.push(Pos::of_str("B8"));
                    }
                }
            }
        }
        if piece.t == Type::Pawn {
            naive_movements.extend(en_passant(&game.board, &game.history, &pos));
        }
        naive_movements
    } else {
        Vec::new()
    }
}

struct GamePieceMovement {
    movement: Movement,
    capture: Option<Pos>,
    aditional_movement: Option<Movement>
}

pub fn allowed_movements_of_player() -> HashMap<Pos, Vec<GamePieceMovement>> {
    // if (is_in_check()) {
    // keep only the movements that avoid check
    // }


    HashMap::new()
}

#[cfg(test)]
mod tests {}
