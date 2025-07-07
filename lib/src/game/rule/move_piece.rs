use crate::{
    game::{
        GameBoard,
        capture::GameCapture,
        game::{GameBounds, GameHistory, GamePlayers},
        movement::movement::{
            CastlingMovement, DefaultMovement, EnPassantMovement, GameMovement, PromotionMovement,
        },
        rule::{allowed_movements::allowed_movements_of_player, turn::evaluate_turn},
    },
    pos::Pos,
};

fn default_move(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    movement: DefaultMovement,
) {
    if let Some(piece) = board.remove(&movement.movement.from) {
        if let Some(captured) = board.insert(movement.movement.to.clone(), piece) {
            if let Some(player) = players.get_mut(&piece.color) {
                player.captures.push(GameCapture { piece: captured, at: history.len() as u16 });
            }
        }
    }
    history.push(movement.movement);
}

fn en_passant_move(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    en_passant: EnPassantMovement,
) {
    if let Some(piece) = board.remove(&en_passant.movement.from) {
        board.insert(en_passant.movement.to.clone(), piece);
        if let Some(captured) = board
            .remove(&Pos { col: en_passant.movement.to.col, row: en_passant.movement.from.row })
        {
            if let Some(player) = players.get_mut(&piece.color) {
                player.captures.push(GameCapture { piece: captured, at: history.len() as u16 });
            }
        }
    }
    history.push(en_passant.movement);
}

fn castling_move(board: &mut GameBoard, history: &mut GameHistory, castling: CastlingMovement) {
    if let Some(piece) = board.remove(&castling.movement.from) {
        board.insert(castling.movement.to.clone(), piece);
        if castling.movement.to.col > castling.movement.from.col {
            if let Some(rook) = board
                .remove(&Pos { col: castling.movement.to.col + 1, row: castling.movement.to.row })
            {
                board.insert(
                    Pos { col: castling.movement.to.col - 1, row: castling.movement.to.row },
                    rook,
                );
            }
        } else if let Some(rook) =
            board.remove(&Pos { col: castling.movement.to.col - 1, row: castling.movement.to.row })
        {
            board.insert(
                Pos { col: castling.movement.to.col + 1, row: castling.movement.to.row },
                rook,
            );
        }
    }
    history.push(castling.movement);
}

fn promotion_move(board: &mut GameBoard, history: &mut GameHistory, promotion: PromotionMovement) {
    board.insert(promotion.pos.clone(), promotion.piece);
    // edit the pawn move
}

fn move_piece(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    movement: GameMovement,
) {
    match movement {
        GameMovement::Default(movement) => default_move(board, players, history, movement),
        GameMovement::Capture(movement) => default_move(board, players, history, DefaultMovement::from(movement.movement)),
        GameMovement::Menace(movement) => {},
        GameMovement::EnPassant(en_passant) => en_passant_move(board, players, history, en_passant),
        GameMovement::Castling(castling) => castling_move(board, history, castling),
        GameMovement::Promotion(promotion) => promotion_move(board, history, promotion),
    }
}

pub fn app_move_piece(
    board: &mut GameBoard,
    bounds: &GameBounds,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    movement: &GameMovement,
) {
    let turn = evaluate_turn(history);
    move_piece(board, players, history, movement.clone());
    for (color, player) in players.iter_mut() {
        if &turn == color {
            player.moves.drain();
        } else {
            player.moves.extend(allowed_movements_of_player(board, bounds, history, color));
        }
    }
}

#[cfg(test)]
mod tests {}

// work on selection
