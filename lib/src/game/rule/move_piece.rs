use crate::game::{movement::{naive::naive_movements_board, Movement}, rule::turn::get_turn, Game};

pub fn move_piece(play: &mut Game, movement: Movement) {
    let curr_turn = get_turn(play);
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
        player.possible_movements = naive_movements_board(&play.board, &player.color);
    }
    play.history.push(movement);
    // if 50 moves with no capture return MoveResult::Stalemate
}

#[cfg(test)]
mod tests {


}
