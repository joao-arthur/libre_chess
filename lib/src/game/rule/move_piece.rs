use crate::{
    game::{
        GameBoard,
        capture::GameCapture,
        game::{GameBounds, GameHistory, GamePlayers},
        mov::{CastlingMovOld, DefaultMovOld, EnPassantMovOld, GameMovOld, PromotionMovOld},
        rule::{allowed_moves::allowed_moves_of_player, turn::evaluate_turn},
    },
    pos::Pos,
};

fn default_move(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    // selection
    mov: DefaultMovOld,
) {
    let mov = mov.mov;
    if let Some(piece) = board.remove(&mov.from) {
        if let Some(captured) = board.insert(mov.to.clone(), piece) {
            if let Some(player) = players.get_mut(&piece.color) {
                player.captures.push(GameCapture { piece: captured, at: history.len() as u16 });
            }
        }
    }
    history.push(mov);
}

fn en_passant_move(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    mov: EnPassantMovOld,
) {
    let mov = mov.mov;
    if let Some(piece) = board.remove(&mov.from) {
        board.insert(mov.to.clone(), piece);
        if let Some(captured) = board.remove(&Pos { col: mov.to.col, row: mov.from.row }) {
            if let Some(player) = players.get_mut(&piece.color) {
                player.captures.push(GameCapture { piece: captured, at: history.len() as u16 });
            }
        }
    }
    history.push(mov);
}

fn castling_move(board: &mut GameBoard, history: &mut GameHistory, mov: CastlingMovOld) {
    let mov = mov.mov;
    if let Some(piece) = board.remove(&mov.from) {
        board.insert(mov.to.clone(), piece);
        if mov.to.col > mov.from.col {
            if let Some(rook) = board.remove(&Pos { col: mov.to.col + 1, row: mov.to.row }) {
                board.insert(Pos { col: mov.to.col - 1, row: mov.to.row }, rook);
            }
        } else if let Some(rook) = board.remove(&Pos { col: mov.to.col - 1, row: mov.to.row }) {
            board.insert(Pos { col: mov.to.col + 1, row: mov.to.row }, rook);
        }
    }
    history.push(mov);
}

fn promotion_move(board: &mut GameBoard, history: &mut GameHistory, promotion: PromotionMovOld) {
    board.insert(promotion.pos.clone(), promotion.piece);
    // edit the pawn mov
}

fn move_piece(
    board: &mut GameBoard,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    mov: GameMovOld,
) {
    match mov {
        GameMovOld::Default(mov) => default_move(board, players, history, mov),
        GameMovOld::Capture(mov) => {
            default_move(board, players, history, DefaultMovOld::from(mov.mov))
        }
        GameMovOld::Menace(mov) => {}
        GameMovOld::EnPassant(mov) => en_passant_move(board, players, history, mov),
        GameMovOld::Castling(mov) => castling_move(board, history, mov),
        GameMovOld::Promotion(mov) => promotion_move(board, history, mov),
    }
}

pub fn app_move_piece(
    board: &mut GameBoard,
    bounds: &GameBounds,
    players: &mut GamePlayers,
    history: &mut GameHistory,
    mov: &GameMovOld,
) {
    let turn = evaluate_turn(history);
    move_piece(board, players, history, mov.clone());
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
