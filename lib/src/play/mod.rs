use std::collections::HashMap;

use movement::{
    bishop::naive_movements_bishop, king::naive_movements_king, knight::naive_movements_knight,
    pawn::naive_movements_pawn, queen::naive_movements_queen, rook::naive_movements_rook, Movement,
};

use crate::{
    board::{pos::Pos, Board},
    piece::{Color, Piece, Type},
};

pub mod movement;

#[derive(Debug, PartialEq)]
pub struct Player {
    captured_pieces: Vec<Piece>,
    possible_movements: HashMap<Piece, Vec<Pos>>,
}

#[derive(Debug, PartialEq)]
pub struct Play {
    pub board: Board,
    pub history: Vec<Movement>,
    pub white_player: Player,
    pub black_player: Player,
}

impl Default for Play {
    fn default() -> Self {
        Play {
            board: Board::default(),
            history: Vec::new(),
            white_player: Player {
                captured_pieces: Vec::new(),
                possible_movements: HashMap::new(),
            },
            black_player: Player {
                captured_pieces: Vec::new(),
                possible_movements: HashMap::new(),
            },
        }
    }
}

fn is_white_turn(history: &Vec<Movement>) -> bool {
    history.len() % 2 == 0
}

fn is_black_turn(history: &Vec<Movement>) -> bool {
    history.len() % 2 == 1
}

pub fn move_piece(play: &mut Play, movement: Movement) {
    if movement.piece.c == Color::Black && !is_black_turn(&play.history) {
        return;
    }
    if movement.piece.c == Color::White && !is_white_turn(&play.history) {
        return;
    }
    if let Some(captured) = play.board[movement.to.clone()] {
        match movement.piece.c {
            Color::White => play.white_player.captured_pieces.push(captured),
            Color::Black => play.black_player.captured_pieces.push(captured),
        }
    }
    play.board[movement.from.clone()] = None;
    play.board[movement.to.clone()] = Some(movement.piece);
    play.history.push(movement);
    // if is_check && aftermoveischeck() return;
    // if let Some()
    //     capture
    // else {
    //     after move
    //     if 50 moves with no capture return MoveResult::Stalemate
    // }
}

pub fn get_moves(play: &Play, pos: &Pos) -> Vec<Pos> {
    if let Some(piece) = play.board[pos.clone()] {
        if piece.c == Color::Black && !is_black_turn(&play.history) {
            return Vec::new();
        }
        if piece.c == Color::White && !is_white_turn(&play.history) {
            return Vec::new();
        }
        match piece.t {
            Type::Rook => naive_movements_rook(&play.board, pos, &piece.c),
            Type::Knight => naive_movements_knight(&play.board, pos, &piece.c),
            Type::Bishop => naive_movements_bishop(&play.board, pos, &piece.c),
            Type::Queen => naive_movements_queen(&play.board, pos, &piece.c),
            Type::King => naive_movements_king(&play.board, pos, &piece.c),
            Type::Pawn => naive_movements_pawn(&play.board, pos, &piece.c),
        }
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::play::movement::Movement;

    #[test]
    fn test_is_white_turn() {
        assert_eq!(is_white_turn(&Vec::new()), true);
        assert_eq!(is_white_turn(&Vec::from([Movement::of_str("♟", "D2", "D4")])), false);
        assert_eq!(
            is_white_turn(&Vec::from([
                Movement::of_str("♟", "D2", "D4"),
                Movement::of_str("♟", "A2", "A3")
            ])),
            true
        );
    }

    #[test]
    fn test_is_black_turn() {
        assert_eq!(is_black_turn(&Vec::new()), false);
        assert_eq!(is_black_turn(&Vec::from([Movement::of_str("♟", "D2", "D4")])), true);
        assert_eq!(
            is_black_turn(&Vec::from([
                Movement::of_str("♟", "D2", "D4"),
                Movement::of_str("♟", "A2", "A3")
            ])),
            false
        );
    }
}
