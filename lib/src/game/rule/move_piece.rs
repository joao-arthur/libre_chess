use crate::{
    board::pos::Pos,
    game::{Game, capture::GameCapture, movement::naive, rule::turn::evaluate_turn},
    movement::Movement,
    piece::Piece,
};

fn default_move(game: &mut Game, movement: Movement) {
    if let Some(piece) = game.board.remove(&movement.from) {
        if let Some(captured) = game.board.insert(movement.to.clone(), piece) {
            if let Some(player) = game.players.get_mut(&piece.color) {
                player.captures.push(GameCapture { piece: captured, at: game.history.len() as u16 });
            }
        }
    }
    game.history.push(movement);
}

fn en_passant_move(game: &mut Game, movement: Movement) {
    if let Some(piece) = game.board.remove(&movement.from) {
        game.board.insert(movement.to.clone(), piece);
        if let Some(captured) = game.board.remove(&Pos { col: movement.to.col, row: movement.from.row }) {
            if let Some(player) = game.players.get_mut(&piece.color) {
                player.captures.push(GameCapture { piece: captured, at: game.history.len() as u16 });
            }
        }
    }
    game.history.push(movement);
}

fn castling_move(game: &mut Game, movement: Movement) {
    if let Some(piece) = game.board.remove(&movement.from) {
        game.board.insert(movement.to.clone(), piece);
        if movement.to.col > movement.from.col {
            if let Some(rook) = game.board.remove(&Pos { col: movement.to.col + 1, row: movement.to.row }) {
                game.board.insert(Pos { col: movement.to.col - 1, row: movement.to.row }, rook);
            }
        } else {
            if let Some(rook) = game.board.remove(&Pos { col: movement.to.col - 1, row: movement.to.row }) {
                game.board.insert(Pos { col: movement.to.col + 1, row: movement.to.row }, rook);
            }
        }
    }
    game.history.push(movement);
}


pub fn move_piece(game: &mut Game, movement: Movement) {
    let curr_turn = evaluate_turn(game);
    game.history.push(movement);
}

#[cfg(test)]
mod tests {}
