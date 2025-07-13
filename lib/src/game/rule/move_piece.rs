use crate::{
    game::{
        board::GameBoard,
        capture::GameCapture,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{GameMove, GameMoveType},
        rule::{allowed_moves::allowed_moves_of_player, turn::evaluate_turn},
    },
    pos::Pos,
};

fn default_move(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    game_move: &GameMove,
) {
    if let Some(piece) = board.remove(&game_move.mov.from) {
        if let Some(captured) = board.insert(game_move.mov.to.clone(), piece) {
            if let Some(player) = players.get_mut(&piece.color) {
                player.captures.push(GameCapture { piece: captured, at: history.len() as u16 });
            }
        }
    }
    history.push(game_move.clone());
}

fn en_passant_move(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    game_move: &GameMove,
) {
    if let Some(piece) = board.remove(&game_move.mov.from) {
        board.insert(game_move.mov.to.clone(), piece);
        if let Some(captured) =
            board.remove(&Pos { col: game_move.mov.to.col, row: game_move.mov.from.row })
        {
            if let Some(player) = players.get_mut(&piece.color) {
                player.captures.push(GameCapture { piece: captured, at: history.len() as u16 });
            }
        }
    }
    history.push(game_move.clone());
}

fn castling_move(board: &mut GameBoard, history: &mut GameHistory, game_move: &GameMove) {
    if let Some(piece) = board.remove(&game_move.mov.from) {
        board.insert(game_move.mov.to.clone(), piece);
        if game_move.mov.to.col > game_move.mov.from.col {
            if let Some(rook) =
                board.remove(&Pos { col: game_move.mov.to.col + 1, row: game_move.mov.to.row })
            {
                board
                    .insert(Pos { col: game_move.mov.to.col - 1, row: game_move.mov.to.row }, rook);
            }
        } else if let Some(rook) =
            board.remove(&Pos { col: game_move.mov.to.col - 1, row: game_move.mov.to.row })
        {
            board.insert(Pos { col: game_move.mov.to.col + 1, row: game_move.mov.to.row }, rook);
        }
    }
    history.push(game_move.clone());
}

fn promotion_move(board: &mut GameBoard, history: &mut GameHistory, game_move: &GameMove) {
    // match type
    // board.insert(game_move.mov.to.clone(), promotion.piece);
    // edit the pawn mov
}

fn move_piece(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    game_move: &GameMove,
) {
    match game_move.typ {
        GameMoveType::Default(_) => default_move(board, players, history, game_move),
        GameMoveType::Capture(_) => default_move(board, players, history, game_move),
        GameMoveType::Menace(_) => {}
        GameMoveType::EnPassant(_) => en_passant_move(board, players, history, game_move),
        GameMoveType::LongCastling(_) | GameMoveType::ShortCastling(_) => {
            castling_move(board, history, game_move)
        }
        GameMoveType::PromotionToQueen(_)
        | GameMoveType::PromotionToKnight(_)
        | GameMoveType::PromotionToRook(_)
        | GameMoveType::PromotionToBishop(_) => promotion_move(board, history, game_move),
    }
}

pub fn app_move_piece(
    board: &mut GameBoard,
    bounds: &GameBounds,
    history: &mut GameHistory,
    players: &mut GamePlayers,
    game_move: &GameMove,
) {
    let turn = evaluate_turn(history);
    move_piece(board, players, history, game_move);
    for (color, player) in players.iter_mut() {
        if &turn == color {
            player.moves.drain();
        } else {
         //   player.moves.extend(allowed_moves_of_player(board, bounds, history, players, color));
        }
    }
}

#[cfg(test)]
mod tests {
    ////
}

// work on selection
