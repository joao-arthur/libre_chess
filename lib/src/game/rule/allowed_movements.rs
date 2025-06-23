
use crate::{
    board::pos::Pos,
    game::{
        Game,
        movement::{
            default,
            movement::GameMovement,
            special::{castling, en_passant},
        },
    },
    piece::Type,
};

pub fn allowed_movements_of_piece(game: &Game, pos: &Pos) -> Vec<GameMovement> {
    if let Some(piece) = game.board.get(pos) {
        match piece.t {
            Type::Pawn => [
                default::movements(&game.board, &game.bounds, pos)
                    .into_iter()
                    .map(GameMovement::from)
                    .collect::<Vec<GameMovement>>(),
                en_passant::movements(&game.board, &game.history, pos)
                    .into_iter()
                    .map(GameMovement::from)
                    .collect::<Vec<GameMovement>>(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            Type::King => [
                default::movements(&game.board, &game.bounds, pos)
                    .into_iter()
                    .map(GameMovement::from)
                    .collect::<Vec<GameMovement>>(),
                castling::movements(&game.board, &game.bounds, &game.history, pos)
                    .into_iter()
                    .map(GameMovement::from)
                    .collect::<Vec<GameMovement>>(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            _ => default::movements(&game.board, &game.bounds, pos)
                .into_iter()
                .map(GameMovement::from)
                .collect::<Vec<GameMovement>>(),
        }
    } else {
        Vec::new()
    }
}

//pub fn allowed_movements_of_player(
//    game: &Game,
//    player: &GamePlayer,
//) -> HashMap<Pos, Vec<GameMovementOld>> {
//    let mut result = HashMap::new();
//
//    let board = game.board.iter();
//    for (pos, piece) in board {
//        if piece.color == player.color {
//            let mut naive_movements = naive::movements_of_piece(&game.board, &game.bounds, pos);
//            if piece.t == Type::King {
//                for (curr_color, curr_player) in game.players {
//                    if curr_color != player.color {
//                        naive_movements.retain(|mov| !curr_player.menace.contains(mov));
//                    }
//                }
//                match piece.color {
//                    Color::White => {
//                        if white_king_can_short_castling(&game.board, &game.history) {
//                            naive_movements.push(GameMovementOld {
//                                movement: Movement {
//                                    piece: piece.clone(),
//                                    from: pos,
//                                    to: Pos::of_str("G1"),
//                                },
//                                capture: None,
//                                secondary_movement: Movement {
//                                    piece: Piece::of_str("♖"),
//                                    from: pos,
//                                    to: Pos::of_str("G1"),
//                                },
//                            });
//                        }
//                        if white_king_can_long_castling(&game.board, &game.history) {
//                            naive_movements.push(Pos::of_str("B1"));
//                        }
//                    }
//                    Color::Black => {
//                        if black_king_can_short_castling(&game.board, &game.history) {
//                            naive_movements.push(Pos::of_str("G8"));
//                        }
//                        if black_king_can_long_castling(&game.board, &game.history) {
//                            naive_movements.push(Pos::of_str("B8"));
//                        }
//                    }
//                }
//            }
//            if piece.t == Type::Pawn {
//                naive_movements.extend(en_passant(&game.board, &game.history, &pos));
//            }
//            naive_movements
//        }
//    }
//
//    result
//}

#[cfg(test)]
mod tests {}
