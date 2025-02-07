use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use movement::{get_naive_movements, naive_movements_piece, Movement};
use player::Player;

use crate::{
    board::{pos::Pos, Board},
    color::Color,
    piece::Type,
};

pub mod movement;
mod player;

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
    for player in play.players.iter_mut() {
        player.1.possible_movements = get_naive_movements(&play.board, &player.0);
    }
}

fn get_turn(play: &Play) -> Color {
    if play.history.len() % 2 == 0 {
        Color::White
    } else {
        Color::Black
    }
}

pub fn move_piece(play: &mut Play, movement: Movement) {
    let turn = get_turn(play);
    if movement.piece.c != turn {
        return;
    }
    play.board.remove(&movement.from);
    if let Some(captured) = play.board.insert(movement.to.clone(), movement.piece) {
        if let Some(player) = play.players.get_mut(&movement.piece.c) {
            player.captured_pieces.push(captured);
        }
    }
    if let Some(player) = play.players.get_mut(&movement.piece.c) {
        player.possible_movements = get_naive_movements(&play.board, &player.color);
    }
    play.history.push(movement);
    // match movement.piece.c {
    //     Color::White => {
    //         play.white_player.possible_movements = naive_movements_piece();
    //     },
    //     Color::Black => {
    //         play.black_player.possible_movements = naive_movements_piece();
    //     },
    // }

    // if is_check && aftermoveischeck() return;
    // if let Some()
    //     capture
    // else {
    //     after move
    //     if 50 moves with no capture return MoveResult::Stalemate
    // }
}

pub fn get_moves(play: &Play, pos: &Pos) -> Vec<Pos> {
    if let Some(piece) = play.board.get(&pos) {
        let turn = get_turn(play);
        if piece.c != turn {
            return Vec::new();
        }
        naive_movements_piece(&play.board, pos)
        // add special movements here!
        // check!
        // en passant!
        // castling!
    } else {
        Vec::new()
    }
}

pub fn is_in_check(play: &Play) -> bool {
    let turn = get_turn(play);
    for row in 0..8 {
        for col in 0..8 {
            if let Some(pos) = Pos::try_of_idx(row, col) {
                if let Some(piece) = play.board.get(&pos) {
                    if piece.c == turn {
                        if piece.t == Type::King {
                            for player in play.players.values() {
                                if player.color != turn {
                                    if player.possible_movements.contains(&pos) {
                                        return true;
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::play::movement::Movement;

    #[test]
    fn test_get_turn() {
        assert_eq!(get_turn(&Play { history: Vec::new(), ..Default::default() }), Color::White);
        assert_eq!(
            get_turn(&Play {
                history: Vec::from([Movement::of_str("♟", "D2", "D4")]),
                ..Default::default()
            }),
            Color::Black
        );
        assert_eq!(
            get_turn(&Play {
                history: Vec::from([
                    Movement::of_str("♟", "D2", "D4"),
                    Movement::of_str("♟", "A2", "A3")
                ]),
                ..Default::default()
            }),
            Color::White
        );
    }
}
