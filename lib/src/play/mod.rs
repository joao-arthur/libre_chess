use std::collections::{HashMap, HashSet};

use movement::{naive_movements_board, naive_movements_piece, Movement};
use player::Player;

use crate::{
    board::{pos::Pos, Board},
    color::Color,
    piece::Type,
};

pub mod movement;
pub mod variant;
mod player;
mod turn;

#[derive(Debug, PartialEq)]
pub struct Play {
    pub board: Board,
    pub players: HashMap<Color, Player>,
    pub history: Vec<Movement>,
}

impl Default for Play {
    fn default() -> Self {
        Play {
            board: Board::default(),
            players: HashMap::from([
                (
                    Color::White,
                    Player {
                        color: Color::White,
                        captured_pieces: Vec::new(),
                        possible_movements: HashSet::new(),
                    },
                ),
                (
                    Color::Black,
                    Player {
                        color: Color::Black,
                        captured_pieces: Vec::new(),
                        possible_movements: HashSet::new(),
                    },
                ),
            ]),
            history: Vec::new(),
        }
    }
}

fn set_board(play: &mut Play, board: Board) {
    play.board = board;
    for entry in play.players.iter_mut() {
        if entry.0 != &Color::White {
            entry.1.possible_movements = naive_movements_board(&play.board, &entry.0);
        }
    }
}



pub fn piece_move(play: &mut Play, movement: Movement) {
    let curr_turn = turn(play);
    if movement.piece.c != curr_turn {
        return;
    }
    // if (is_in_check() && is_check_after_move()) {
    //     return;
    // }
    play.board.remove(&movement.from);
    if let Some(player) = play.players.get_mut(&movement.piece.c) {
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
        let curr_turn = turn(play);
        if piece.c != curr_turn {
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
            naive_movements = naive_movements.into_iter().filter(|mov| !other_pos.contains(&mov)).collect();
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
    let curr_turn = turn(play);
    for entry in play.board.iter() {
        if entry.1.t == Type::King && entry.1.c == curr_turn {
            for player in play.players.values() {
                if player.color != curr_turn && player.possible_movements.contains(entry.0) {
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
    use super::*;

    use crate::{board, piece::Piece, play::variant::standard_initial_board};

    #[test]
    fn test_play() {
        assert_eq!(
            Play::default(),
            Play {
                board: Board::default(),
                players: HashMap::from([
                    (
                        Color::White,
                        Player {
                            color: Color::White,
                            captured_pieces: Vec::new(),
                            possible_movements: HashSet::new(),
                        },
                    ),
                    (
                        Color::Black,
                        Player {
                            color: Color::Black,
                            captured_pieces: Vec::new(),
                            possible_movements: HashSet::new(),
                        },
                    ),
                ]),
                history: Vec::new(),
            }
        );
    }

    #[test]
    fn test_set_board() {
        let mut play = Play::default();
        set_board(&mut play, standard_initial_board());
        assert_eq!(
            play,
            Play {
                board: HashMap::from([
                    (Pos::of_str("A8"), Piece::of_str("♜")),
                    (Pos::of_str("B8"), Piece::of_str("♞")),
                    (Pos::of_str("C8"), Piece::of_str("♝")),
                    (Pos::of_str("D8"), Piece::of_str("♛")),
                    (Pos::of_str("E8"), Piece::of_str("♚")),
                    (Pos::of_str("F8"), Piece::of_str("♝")),
                    (Pos::of_str("G8"), Piece::of_str("♞")),
                    (Pos::of_str("H8"), Piece::of_str("♜")),
                    (Pos::of_str("A7"), Piece::of_str("♟")),
                    (Pos::of_str("B7"), Piece::of_str("♟")),
                    (Pos::of_str("C7"), Piece::of_str("♟")),
                    (Pos::of_str("D7"), Piece::of_str("♟")),
                    (Pos::of_str("E7"), Piece::of_str("♟")),
                    (Pos::of_str("F7"), Piece::of_str("♟")),
                    (Pos::of_str("G7"), Piece::of_str("♟")),
                    (Pos::of_str("H7"), Piece::of_str("♟")),
                    (Pos::of_str("A2"), Piece::of_str("♙")),
                    (Pos::of_str("B2"), Piece::of_str("♙")),
                    (Pos::of_str("C2"), Piece::of_str("♙")),
                    (Pos::of_str("D2"), Piece::of_str("♙")),
                    (Pos::of_str("E2"), Piece::of_str("♙")),
                    (Pos::of_str("F2"), Piece::of_str("♙")),
                    (Pos::of_str("G2"), Piece::of_str("♙")),
                    (Pos::of_str("H2"), Piece::of_str("♙")),
                    (Pos::of_str("A1"), Piece::of_str("♖")),
                    (Pos::of_str("B1"), Piece::of_str("♘")),
                    (Pos::of_str("C1"), Piece::of_str("♗")),
                    (Pos::of_str("D1"), Piece::of_str("♕")),
                    (Pos::of_str("E1"), Piece::of_str("♔")),
                    (Pos::of_str("F1"), Piece::of_str("♗")),
                    (Pos::of_str("G1"), Piece::of_str("♘")),
                    (Pos::of_str("H1"), Piece::of_str("♖")),
                ]),
                players: HashMap::from([
                    (
                        Color::White,
                        Player {
                            color: Color::White,
                            captured_pieces: Vec::new(),
                            possible_movements: HashSet::new(),
                        },
                    ),
                    (
                        Color::Black,
                        Player {
                            color: Color::Black,
                            captured_pieces: Vec::new(),
                            possible_movements: HashSet::from([
                                Pos::of_str("A6"),
                                Pos::of_str("B6"),
                                Pos::of_str("C6"),
                                Pos::of_str("D6"),
                                Pos::of_str("E6"),
                                Pos::of_str("F6"),
                                Pos::of_str("G6"),
                                Pos::of_str("H6"),
                                Pos::of_str("A5"),
                                Pos::of_str("B5"),
                                Pos::of_str("C5"),
                                Pos::of_str("D5"),
                                Pos::of_str("E5"),
                                Pos::of_str("F5"),
                                Pos::of_str("G5"),
                                Pos::of_str("H5"),
                            ]),
                        },
                    ),
                ]),
                history: Vec::new(),
            }
        )
    }

    #[test]
    fn test_move_piece() {}

    #[test]
    fn test_movements_piece() {
        // pra testar aqui, o resto tem que estar funcionando
    }

    #[test]
    fn test_is_in_check_false() {
        assert_eq!(is_in_check(
            &Play {
                board: board::of_str([
                    "    ♚   ",
                    "    ♟   ",
                    "        ",
                    "        ",
                    "        ",
                    "    ♖   ",
                    "        ",
                    "    ♔   ",
                ]),
                players: HashMap::from([
                    (
                        Color::White,
                        Player {
                            color: Color::White,
                            captured_pieces: Vec::new(),
                            possible_movements: HashSet::new(),
                        },
                    ),
                    (
                        Color::Black,
                        Player {
                            color: Color::Black,
                            captured_pieces: Vec::new(),
                            possible_movements: HashSet::from([
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                                Pos::of_str(""),
                            ]),
                        },
                    ),
                ]),
                history: Vec::new(),
            }
        ), false);
    }

    #[test]
    fn test_is_in_check_true() {
        assert_eq!(is_in_check(
            &Play {
                board: board::of_str([
                    "    ♚   ",
                    "   ♙ ♟  ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "    ♔   ",
                ]),
                players: HashMap::from([
                    (
                        Color::White,
                        Player {
                            color: Color::White,
                            captured_pieces: Vec::new(),
                            possible_movements: HashSet::new(),
                        },
                    ),
                    (
                        Color::Black,
                        Player {
                            color: Color::Black,
                            captured_pieces: Vec::new(),
                            possible_movements: HashSet::from([
                                Pos::of_str("A6"),
                                Pos::of_str("B6"),
                                Pos::of_str("C6"),
                                Pos::of_str("D6"),
                                Pos::of_str("E6"),
                                Pos::of_str("F6"),
                                Pos::of_str("G6"),
                                Pos::of_str("H6"),
                                Pos::of_str("A5"),
                                Pos::of_str("B5"),
                                Pos::of_str("C5"),
                                Pos::of_str("D5"),
                                Pos::of_str("E5"),
                                Pos::of_str("F5"),
                                Pos::of_str("G5"),
                                Pos::of_str("H5"),
                            ]),
                        },
                    ),
                ]),
                history: Vec::new(),
            }
        ), true);
    }

    // criar função pra criar PLAY::from_historico
}
