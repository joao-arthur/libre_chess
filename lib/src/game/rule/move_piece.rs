use crate::{
    game::{
        GameBoard,
        capture::GameCapture,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{
            CastlingMov, DefaultMov, EnPassantMov, GameMov, PromotionMov,
        },
        rule::{allowed_moves::allowed_moves_of_player, turn::evaluate_turn},
    },
    pos::Pos,
};

fn default_move(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    mov: DefaultMov,
) {
    if let Some(piece) = board.remove(&mov.mov.from) {
        if let Some(captured) = board.insert(mov.mov.to.clone(), piece) {
            if let Some(player) = players.get_mut(&piece.color) {
                player.captures.push(GameCapture { piece: captured, at: history.len() as u16 });
            }
        }
    }
    history.push(mov.mov);
}

fn en_passant_move(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    en_passant: EnPassantMov,
) {
    if let Some(piece) = board.remove(&en_passant.mov.from) {
        board.insert(en_passant.mov.to.clone(), piece);
        if let Some(captured) = board
            .remove(&Pos { col: en_passant.mov.to.col, row: en_passant.mov.from.row })
        {
            if let Some(player) = players.get_mut(&piece.color) {
                player.captures.push(GameCapture { piece: captured, at: history.len() as u16 });
            }
        }
    }
    history.push(en_passant.mov);
}

fn castling_move(board: &mut GameBoard, history: &mut GameHistory, castling: CastlingMov) {
    if let Some(piece) = board.remove(&castling.mov.from) {
        board.insert(castling.mov.to.clone(), piece);
        if castling.mov.to.col > castling.mov.from.col {
            if let Some(rook) = board
                .remove(&Pos { col: castling.mov.to.col + 1, row: castling.mov.to.row })
            {
                board.insert(
                    Pos { col: castling.mov.to.col - 1, row: castling.mov.to.row },
                    rook,
                );
            }
        } else if let Some(rook) =
            board.remove(&Pos { col: castling.mov.to.col - 1, row: castling.mov.to.row })
        {
            board.insert(
                Pos { col: castling.mov.to.col + 1, row: castling.mov.to.row },
                rook,
            );
        }
    }
    history.push(castling.mov);
}

fn promotion_move(board: &mut GameBoard, history: &mut GameHistory, promotion: PromotionMov) {
    board.insert(promotion.pos.clone(), promotion.piece);
    // edit the pawn mov
}

fn move_piece(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    movement: GameMov,
) {
    match movement {
        GameMov::Default(movement) => default_move(board, players, history, movement),
        GameMov::Capture(movement) => default_move(board, players, history, DefaultMov::from(movement.mov)),
        GameMov::Menace(movement) => {},
        GameMov::EnPassant(en_passant) => en_passant_move(board, players, history, en_passant),
        GameMov::Castling(castling) => castling_move(board, history, castling),
        GameMov::Promotion(promotion) => promotion_move(board, history, promotion),
    }
}

pub fn app_move_piece(
    board: &mut GameBoard,
    bounds: &GameBounds,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    movement: &GameMov,
) {
    let turn = evaluate_turn(history);
    move_piece(board, players, history, movement.clone());
    for (color, player) in players.iter_mut() {
        if &turn == color {
            player.moves.drain();
        } else {
            player.moves.extend(allowed_moves_of_player(board, bounds, history, color));
        }
    }
}

#[cfg(test)]
mod tests {}

// work on selection
