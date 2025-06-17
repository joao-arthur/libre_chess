use crate::game::{
    Game,
    movement::{Movement, naive},
    rule::turn::evaluate_turn,
};

pub fn move_piece(play: &mut Game, movement: Movement) {
    let curr_turn = evaluate_turn(play);
    if movement.piece.color != curr_turn {
        return;
    }
    // if (is_in_check() && is_check_after_move()) {
    //     return;
    // }
    play.board.remove(&movement.from);
    if let Some(player) = play.players.get_mut(&movement.piece.color) {
        if let Some(captured) = play.board.insert(movement.to.clone(), movement.piece) {
            player.captured_pieces.push(captured);
        }
        player.possible_movements =
            naive::movements_of_player(&play.board, &play.bounds, &player.color);
    }
    play.history.push(movement);
    // if 50 moves with no capture return MoveResult::Stalemate
}

#[cfg(test)]
mod tests {}
