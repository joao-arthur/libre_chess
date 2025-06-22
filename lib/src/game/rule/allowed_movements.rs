use std::collections::HashMap;

use crate::{
    board::pos::Pos,
    color::Color,
    game::{
        Game,
        movement::{
            naive,
            special::{
                castling::{
                    black_king_can_long_castling, black_king_can_short_castling,
                    white_king_can_long_castling, white_king_can_short_castling,
                },
                en_passant::en_passant,
            },
        },
        player::GamePlayer,
        rule::turn::evaluate_turn,
    },
    movement::Movement,
    piece::{Piece, Type},
};

//pub fn allowed_movements_of_piece(game: &Game, pos: &Pos) -> Vec<Pos> {
//    if let Some(piece) = game.board.get(pos) {
//        let curr_turn = evaluate_turn(game);
//        if piece.color != curr_turn {
//            return Vec::new();
//        }
//        let mut naive_movements = naive::movements_of_piece(&game.board, &game.bounds, pos);
//        if piece.t == Type::King {
//            for player in game.players.values() {
//                if player.color != curr_turn {
//                    naive_movements.retain(|mov| !player.menace.contains(mov));
//                }
//            }
//            match piece.color {
//                Color::White => {
//                    if white_king_can_short_castling(&game.board, &game.history) {
//                        naive_movements.push(Pos::of_str("G1"));
//                    }
//                    if white_king_can_long_castling(&game.board, &game.history) {
//                        naive_movements.push(Pos::of_str("B1"));
//                    }
//                }
//                Color::Black => {
//                    if black_king_can_short_castling(&game.board, &game.history) {
//                        naive_movements.push(Pos::of_str("G8"));
//                    }
//                    if black_king_can_long_castling(&game.board, &game.history) {
//                        naive_movements.push(Pos::of_str("B8"));
//                    }
//                }
//            }
//        }
//        if piece.t == Type::Pawn {
//            naive_movements.extend(en_passant(&game.board, &game.history, &pos));
//        }
//        naive_movements
//    } else {
//        Vec::new()
//    }
//}

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
mod tests {
//    use std::collections::HashMap;

//    use crate::{
//        board::pos::Pos,
//        color::Color,
//        game::{mode::standard_chess, movement::Movement, rule::init::init_game},
//        piece::Piece,
//    };

//    use super::{GameMovementOld, allowed_movements_of_player};

//    #[test]
//    fn test_allowed_movements_of_player() {
//        let game = init_game(standard_chess());
//        assert_eq!(
//            allowed_movements_of_player(&game, game.players.get(&Color::White).unwrap()),
//            HashMap::from([
//                (
//                    Pos::of_str("A2"),
//                    vec![
//                        GameMovementOld::from(Movement::of_str("♙", "A2", "A3")),
//                        GameMovementOld::from(Movement::of_str("♙", "A2", "A4")),
//                    ]
//                ),
//                (
//                    Pos::of_str("B2"),
//                    vec![
//                        GameMovementOld::from(Movement::of_str("♙", "B2", "B3")),
//                        GameMovementOld::from(Movement::of_str("♙", "B2", "B4")),
//                    ]
//                ),
//                (
//                    Pos::of_str("C2"),
//                    vec![
//                        GameMovementOld::from(Movement::of_str("♙", "C2", "C3")),
//                        GameMovementOld::from(Movement::of_str("♙", "C2", "C4")),
//                    ]
//                ),
//                (
//                    Pos::of_str("D2"),
//                    vec![
//                        GameMovementOld::from(Movement::of_str("♙", "D2", "D3")),
//                        GameMovementOld::from(Movement::of_str("♙", "D2", "D4")),
//                    ]
//                ),
//                (
//                    Pos::of_str("E2"),
//                    vec![
//                        GameMovementOld::from(Movement::of_str("♙", "E2", "E3")),
//                        GameMovementOld::from(Movement::of_str("♙", "E2", "E4")),
//                    ]
//                ),
//                (
//                    Pos::of_str("F2"),
//                    vec![
//                        GameMovementOld::from(Movement::of_str("♙", "F2", "F3")),
//                        GameMovementOld::from(Movement::of_str("♙", "F2", "F4")),
//                    ]
//                ),
//                (
//                    Pos::of_str("G2"),
//                    vec![
//                        GameMovementOld::from(Movement::of_str("♙", "G2", "G3")),
//                        GameMovementOld::from(Movement::of_str("♙", "G2", "G4")),
//                    ]
//                ),
//                (
//                    Pos::of_str("H2"),
//                    vec![
//                        GameMovementOld::from(Movement::of_str("♙", "H2", "H3")),
//                        GameMovementOld::from(Movement::of_str("♙", "H2", "H4")),
//                    ]
//                ),
//                (
//                    Pos::of_str("B1"),
//                    vec![
//                        GameMovementOld::from(Movement::of_str("♘", "B1", "A3")),
//                        GameMovementOld::from(Movement::of_str("♘", "B1", "C3")),
//                    ]
//                ),
//                (
//                    Pos::of_str("G1"),
//                    vec![
//                        GameMovementOld::from(Movement::of_str("♘", "G1", "F3")),
//                        GameMovementOld::from(Movement::of_str("♘", "G1", "H3")),
//                    ]
//                ),
//            ])
//        );
//    }
}
