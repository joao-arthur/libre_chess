use std::collections::HashSet;

use crate::{board::pos::Pos, game::{movement::naive::naive_movements_piece, rule::turn::get_turn, Game}, piece::Type};

pub fn allowed_movements(play: &Game, pos: &Pos) -> Vec<Pos> {
    if let Some(piece) = play.board.get(&pos) {
        let curr_turn = get_turn(play);
        if piece.color != curr_turn {
            return Vec::new();
        }
        let mut naive_movements = naive_movements_piece(&play.board, pos);
        if piece.t == Type::King {
            let mut other_pos: HashSet<Pos> = HashSet::new();
            for player in play.players.values() {
                if player.color != curr_turn {
                    for mov in player.possible_movements.iter() {
                        other_pos.insert(mov.clone());
                    }
                }
            }
            naive_movements =
                naive_movements.into_iter().filter(|mov| !other_pos.contains(&mov)).collect();
        }
        // add special movements (check, en passant, castling) here
        return naive_movements;
    } else {
        return Vec::new();
    }
}

#[cfg(test)]
mod tests {


    //  #[test]
    // fn test_set_board() {
    //     let mut play = Game::default();
    //     set_board(&mut play, chess_standard().initial_board);
    //     assert_eq!(
    //         play,
    //         Game {
    //             board: HashMap::from([
    //                 piece::of_str("A8", "♜"),
    //                 piece::of_str("B8", "♞"),
    //                 piece::of_str("C8", "♝"),
    //                 piece::of_str("D8", "♛"),
    //                 piece::of_str("E8", "♚"),
    //                 piece::of_str("F8", "♝"),
    //                 piece::of_str("G8", "♞"),
    //                 piece::of_str("H8", "♜"),
    //                 piece::of_str("A7", "♟"),
    //                 piece::of_str("B7", "♟"),
    //                 piece::of_str("C7", "♟"),
    //                 piece::of_str("D7", "♟"),
    //                 piece::of_str("E7", "♟"),
    //                 piece::of_str("F7", "♟"),
    //                 piece::of_str("G7", "♟"),
    //                 piece::of_str("H7", "♟"),
    //                 piece::of_str("A2", "♙"),
    //                 piece::of_str("B2", "♙"),
    //                 piece::of_str("C2", "♙"),
    //                 piece::of_str("D2", "♙"),
    //                 piece::of_str("E2", "♙"),
    //                 piece::of_str("F2", "♙"),
    //                 piece::of_str("G2", "♙"),
    //                 piece::of_str("H2", "♙"),
    //                 piece::of_str("A1", "♖"),
    //                 piece::of_str("B1", "♘"),
    //                 piece::of_str("C1", "♗"),
    //                 piece::of_str("D1", "♕"),
    //                 piece::of_str("E1", "♔"),
    //                 piece::of_str("F1", "♗"),
    //                 piece::of_str("G1", "♘"),
    //                 piece::of_str("H1", "♖"),
    //             ]),
    //             players: HashMap::from([
    //                 (
    //                     Color::White,
    //                     Player {
    //                         color: Color::White,
    //                         captured_pieces: Vec::new(),
    //                         possible_movements: HashSet::new(),
    //                     },
    //                 ),
    //                 (
    //                     Color::Black,
    //                     Player {
    //                         color: Color::Black,
    //                         captured_pieces: Vec::new(),
    //                         possible_movements: HashSet::from([
    //                             Pos::of_str("A6"),
    //                             Pos::of_str("B6"),
    //                             Pos::of_str("C6"),
    //                             Pos::of_str("D6"),
    //                             Pos::of_str("E6"),
    //                             Pos::of_str("F6"),
    //                             Pos::of_str("G6"),
    //                             Pos::of_st//r("H6"),

    //                             Pos::of_str("A5"),
    //                             Pos::of_str("B5"),
    //                             Pos::of_str("C5"),
    //                             Pos::of_str("D5"),
    //                             Pos::of_str("E5"),
    //                             Pos::of_str("F5"),
    //                             Pos::of_str("G5"),
    //                             Pos::of_str("H5"),
    //                         ]),
    //                     },
    //                 ),
    //             ]),
    //             history: Vec::new(),
    //         }
    //   )
    // }

    // #[test]
    // fn move_piece() {}

    // #[test]
    // fn movements_piece() {
    // pra testar aqui, o resto tem que estar funcionando
    // }
}
