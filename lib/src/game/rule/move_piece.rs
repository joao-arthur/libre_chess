use crate::{
    board::pos::Pos,
    game::{
        capture::GameCapture, movement::movement::{
            CastlingMovement, DefaultMovement, EnPassantMovement, GameMovement, PromotionMovement,
        }, rule::{allowed_movements::allowed_movements_of_player, turn::evaluate_turn}, Game
    },
};

fn default_move(game: &mut Game, movement: DefaultMovement) {
    if let Some(piece) = game.board.remove(&movement.movement.from) {
        if let Some(captured) = game.board.insert(movement.movement.to.clone(), piece) {
            if let Some(player) = game.players.get_mut(&piece.color) {
                player
                    .captures
                    .push(GameCapture { piece: captured, at: game.history.len() as u16 });
            }
        }
    }
    game.history.push(movement.movement);
}

fn en_passant_move(game: &mut Game, en_passant: EnPassantMovement) {
    if let Some(piece) = game.board.remove(&en_passant.movement.from) {
        game.board.insert(en_passant.movement.to.clone(), piece);
        if let Some(captured) = game
            .board
            .remove(&Pos { col: en_passant.movement.to.col, row: en_passant.movement.from.row })
        {
            if let Some(player) = game.players.get_mut(&piece.color) {
                player
                    .captures
                    .push(GameCapture { piece: captured, at: game.history.len() as u16 });
            }
        }
    }
    game.history.push(en_passant.movement);
}

fn castling_move(game: &mut Game, castling: CastlingMovement) {
    if let Some(piece) = game.board.remove(&castling.movement.from) {
        game.board.insert(castling.movement.to.clone(), piece);
        if castling.movement.to.col > castling.movement.from.col {
            if let Some(rook) = game
                .board
                .remove(&Pos { col: castling.movement.to.col + 1, row: castling.movement.to.row })
            {
                game.board.insert(
                    Pos { col: castling.movement.to.col - 1, row: castling.movement.to.row },
                    rook,
                );
            }
        } else if let Some(rook) = game
            .board
            .remove(&Pos { col: castling.movement.to.col - 1, row: castling.movement.to.row })
        {
            game.board.insert(
                Pos { col: castling.movement.to.col + 1, row: castling.movement.to.row },
                rook,
            );
        }
    }
    game.history.push(castling.movement);
}

fn promotion_move(game: &mut Game, promotion: PromotionMovement) {
    game.board.insert(promotion.pos.clone(), promotion.piece);
    // edit the pawn move
}

fn move_piece(game: &mut Game, movement: GameMovement) {
    match movement {
        GameMovement::Default(movement) => default_move(game, movement),
        GameMovement::EnPassant(en_passant) => en_passant_move(game, en_passant),
        GameMovement::Castling(castling) => castling_move(game, castling),
        GameMovement::Promotion(promotion) => promotion_move(game, promotion),
    }
}

pub fn app_move_piece(game: &mut Game, movement: GameMovement) {
    let turn = evaluate_turn(&game.history);
    move_piece(game, movement);
    for (color, player) in game.players.iter_mut() {
        if &turn == color {
            player.moves.drain();
        } else {
           player.moves.extend(allowed_movements_of_player(&game.board, &game.bounds, &game.history, &color));
        }
    }
}

#[cfg(test)]
mod tests {}


// work on selection