use std::collections::HashSet;

use crate::{
    board::pos::Pos,
    game::{Game, movement::naive, rule::turn::evaluate_turn},
    piece::Type,
};

// in the next iteration, this will be pre calculated for each piece
// but ill need to make sure it doesnot break for king
pub fn allowed_movements(play: &Game, pos: &Pos) -> Vec<Pos> {
    if let Some(piece) = play.board.get(pos) {
        let curr_turn = evaluate_turn(play);
        if piece.color != curr_turn {
            return Vec::new();
        }
        // if check, keep only the movements that avoid check
        let mut naive_movements = naive::movements_of_piece(&play.board, &play.bounds, pos);
        if piece.t == Type::King {
            let mut other_pos: HashSet<Pos> = HashSet::new();
            for player in play.players.values() {
                if player.color != curr_turn {
                    for mov in player.possible_movements.iter() {
                        other_pos.insert(mov.clone());
                    }
                }
            }
            naive_movements.retain(|mov| !other_pos.contains(mov));
            // if short_castling add
            // if long_castling add
        }
        if piece.t == Type::Pawn {
            // if can't capture left, remove
            // if can't capture right, remove
            // if en_passantm, keep
        }
        naive_movements
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {}
