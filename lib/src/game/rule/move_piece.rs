use crate::game::{
    Game,
    movement::{Movement, naive},
    rule::turn::evaluate_turn,
};

pub fn move_piece(game: &mut Game, movement: Movement) {
    let curr_turn = evaluate_turn(game);
    if movement.piece.color != curr_turn {
        return;
    }
    // if (is_in_check() && is_check_after_move()) {
    //     return;
    // }
    game.board.remove(&movement.from);
    if let Some(player) = game.players.get_mut(&movement.piece.color) {
        if let Some(captured) = game.board.insert(movement.to.clone(), movement.piece) {
            player.captured_pieces.push(captured);
        }
        player.possible_movements =
            naive::movements_of_player(&game.board, &game.bounds, &player.color);
    }
    game.history.push(movement);


   // if game.board.get() {
   //     piece.king {
   //         // if castling
   //          move king and move rook
   //     }
   // }



    // if 50 moves with no capture return MoveResult::Stalemate
}

#[cfg(test)]
mod tests {}
