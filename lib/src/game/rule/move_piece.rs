use crate::{
    board::pos::Pos,
    game::{Game, capture::GameCapture, movement::naive, rule::turn::evaluate_turn},
    movement::Movement,
    piece::Piece,
};

fn default_move(game: &mut Game, movement: Movement) {
    game.board.remove(&movement.from);
    if let Some(player) = game.players.get_mut(&movement.piece.color) {
        if let Some(captured) = game.board.insert(movement.to.clone(), movement.piece) {
            player.captures.push(GameCapture { piece: captured, at: game.history.len() as u16 });
        }
    }
    game.history.push(movement);
}

fn en_passant_move(game: &mut Game, movement: Movement) {
    game.board.remove(&movement.from);
    if let Some(player) = game.players.get_mut(&movement.piece.color) {
        game.board.insert(movement.to.clone(), movement.piece);
        if let Some(captured) = game.board.remove(&Pos { col: movement.to.col, row: movement.from.row }) {
            player.captures.push(GameCapture { piece: captured, at: game.history.len() as u16 });
        }
    }
    game.history.push(movement);
}

fn castling_move(game: &mut Game, movement: Movement) {
    game.board.remove(&movement.from);
    if let Some(player) = game.players.get_mut(&movement.piece.color) {
        game.board.insert(movement.to.clone(), movement.piece);
        if let Some(captured) = game.board.remove(&Pos { col: movement.to.col, row: movement.from.row }) {
            player.captures.push(GameCapture { piece: captured, at: game.history.len() as u16 });
        }
    }
    game.history.push(movement);
}


pub fn move_piece(game: &mut Game, movement: Movement) {
    let curr_turn = evaluate_turn(game);
    if movement.piece.color != curr_turn {
        return;
    }
    game.board.remove(&movement.from);
    if let Some(player) = game.players.get_mut(&movement.piece.color) {
        if let Some(captured) = game.board.insert(movement.to.clone(), movement.piece) {
            player.captures.push(GameCapture { piece: captured, at: game.history.len() as u16 });
        }
   //     player.menace = naive::movements_of_player(&game.board, &game.bounds, &player.color);
    }
    if movement.piece == Piece::of_str("♔") {
        if movement.to == Pos::of_str("G1") {
            if let Some(rook) = game.board.remove(&Pos::of_str("H1")) {
                game.board.insert(Pos::of_str("F1"), rook);
            }
        }
        if movement.to == Pos::of_str("B1") {
            if let Some(rook) = game.board.remove(&Pos::of_str("A1")) {
                game.board.insert(Pos::of_str("C1"), rook);
            }
        }
    }
    if movement.piece == Piece::of_str("♚") {
        if movement.to == Pos::of_str("G8") {
            if let Some(rook) = game.board.remove(&Pos::of_str("H8")) {
                game.board.insert(Pos::of_str("F1"), rook);
            }
        }
        if movement.to == Pos::of_str("B8") {
            if let Some(rook) = game.board.remove(&Pos::of_str("A8")) {
                game.board.insert(Pos::of_str("C8"), rook);
            }
        }
    }
    if movement.piece == Piece::of_str("♟") {}
    if movement.piece == Piece::of_str("♙") {}

    // en passand captire

    game.history.push(movement);

    // if 50 moves with no capture return MoveResult::Stalemate
}

#[cfg(test)]
mod tests {}
