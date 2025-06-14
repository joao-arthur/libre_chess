use std::collections::HashSet;

use crate::{board::pos::Pos, color::Color, game::board::Board, piece::Type};

use super::{
    movement::{
        Movement,
        naive::{naive_movements_board, naive_movements_piece},
    },
    play::Play,
    turn::get_turn,
};

fn set_board(play: &mut Play, board: Board) {
    play.board = board;
    for (color, player) in play.players.iter_mut() {
        if color != &Color::White {
            player.possible_movements = naive_movements_board(&play.board, &color);
        }
    }
}

pub fn piece_move(play: &mut Play, movement: Movement) {
    let curr_turn = get_turn(play);
    if movement.piece.color != curr_turn {
        return;
    }
    // if (is_in_check() && is_check_after_move()) {
    //     return;
    // }
    play.board.remove(&movement.from);
    if let Some(player) = play.players.get_mut(&movement.piece.color) {
        if let Some(captured) = play.board.insert(movement.to.clone(), movement.piece) {
            player.captured_pieces.push(captured);
        }
        player.possible_movements = naive_movements_board(&play.board, &player.color);
    }
    play.history.push(movement);
    // if 50 moves with no capture return MoveResult::Stalemate
}

pub fn piece_movements(play: &Play, pos: &Pos) -> Vec<Pos> {
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
        // add special movements here!
        // check!
        // en passant!
        // castling!
        return naive_movements;
    } else {
        return Vec::new();
    }
}

pub fn is_in_check(play: &Play) -> bool {
    let curr_turn = get_turn(play);
    for (pos, piece) in play.board.iter() {
        if piece.t == Type::King && piece.color == curr_turn {
            for player in play.players.values() {
                if player.color != curr_turn && player.possible_movements.contains(pos) {
                    return true;
                }
            }
            break;
        }
    }
    false
}

#[cfg(test)]
mod tests {

    use std::collections::{HashMap, HashSet};

    use crate::{
        board::pos::Pos,
        color::Color,
        game::{mode::chess_standard, play::Play, player::Player},
    };

    use super::{is_in_check, piece_move, piece_movements, set_board};

    //  #[test]
    // fn test_set_board() {
    //     let mut play = Play::default();
    //     set_board(&mut play, chess_standard().initial_board);
    //     assert_eq!(
    //         play,
    //         Play {
    //             board: HashMap::from([
    //                 (piece::of_str("A8", "♜")),
    //                 (piece::of_str("B8", "♞")),
    //                 (piece::of_str("C8", "♝")),
    //                 (piece::of_str("D8", "♛")),
    //                 (piece::of_str("E8", "♚")),
    //                 (piece::of_str("F8", "♝")),
    //                 (piece::of_str("G8", "♞")),
    //                 (piece::of_str("H8", "♜")),
    //                 (piece::of_str("A7", "♟")),
    //                 (piece::of_str("B7", "♟")),
    //                 (piece::of_str("C7", "♟")),
    //                 (piece::of_str("D7", "♟")),
    //                 (piece::of_str("E7", "♟")),
    //                 (piece::of_str("F7", "♟")),
    //                 (piece::of_str("G7", "♟")),
    //                 (piece::of_str("H7", "♟")),
    //                 (piece::of_str("A2", "♙")),
    //                 (piece::of_str("B2", "♙")),
    //                 (piece::of_str("C2", "♙")),
    //                 (piece::of_str("D2", "♙")),
    //                 (piece::of_str("E2", "♙")),
    //                 (piece::of_str("F2", "♙")),
    //                 (piece::of_str("G2", "♙")),
    //                 (piece::of_str("H2", "♙")),
    //                 (piece::of_str("A1", "♖")),
    //                 (piece::of_str("B1", "♘")),
    //                 (piece::of_str("C1", "♗")),
    //                 (piece::of_str("D1", "♕")),
    //                 (piece::of_str("E1", "♔")),
    //                 (piece::of_str("F1", "♗")),
    //                 (piece::of_str("G1", "♘")),
    //                 (piece::of_str("H1", "♖")),
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

    //  #[test]
    //   fn is_in_check_false() {
    // assert_eq!(is_in_check(
    //     &Play {
    //         board: game::board::of_str([
    //             "    ♚   ",
    //             "    ♟   ",
    //             "        ",
    //             "        ",
    //             "        ",
    //             "    ♖   ",
    //             "        ",
    //             "    ♔   ",
    //         ]),
    //         players: HashMap::from([
    //             (
    //                 Color::White,
    //                 Player {
    //                     color: Color::White,
    //                     captured_pieces: Vec::new(),
    //                     possible_movements: HashSet::new(),
    //                 },
    //             ),
    //             (
    //                 Color::Black,
    //                 Player {
    //                     color: Color::Black,
    //                     captured_pieces: Vec::new(),
    //                     possible_movements: HashSet::from([
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                     ]),
    //                 },
    //             ),
    //         ]),
    //         history: Vec::new(),
    //     }
    // ), false);
    //    }

    //   #[test]
    //   fn is_in_check_true() {
    // assert_eq!(is_in_check(
    //     &Play {
    //         board: game::board::of_str([
    //             "    ♚   ",
    //             "   ♙ ♟  ",
    //             "        ",
    //             "        ",
    //             "        ",
    //             "        ",
    //             "        ",
    //             "    ♔   ",
    //         ]),
    //         players: HashMap::from([
    //             (
    //                 Color::White,
    //                 Player {
    //                     color: Color::White,
    //                     captured_pieces: Vec::new(),
    //                     possible_movements: HashSet::new(),
    //                 },
    //             ),
    //             (
    //                 Color::Black,
    //                 Player {
    //                     color: Color::Black,
    //                     captured_pieces: Vec::new(),
    //                     possible_movements: HashSet::from([
    //                         Pos::of_str("A6"),
    //                         Pos::of_str("B6"),
    //                         Pos::of_str("C6"),
    //                         Pos::of_str("D6"),
    //                         Pos::of_str("E6"),
    //                         Pos::of_str("F6"),
    //                         Pos::of_str("G6"),
    //                         Pos::of_str("H6"),
    //                         Pos::of_str("A5"),
    //                         Pos::of_str("B5"),
    //                         Pos::of_str("C5"),
    //                         Pos::of_str("D5"),
    //                         Pos::of_str("E5"),
    //                         Pos::of_str("F5"),
    //                         Pos::of_str("G5"),
    //                         Pos::of_str("H5"),
    //                     ]),
    //                 },
    //             ),
    //         ]),
    //         history: Vec::new(),
    //     }
    // ), true);
    //  }

    // criar função pra criar PLAY::from_historico
}
