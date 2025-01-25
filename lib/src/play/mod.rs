use std::collections::HashMap;

use movement::Movement;

use crate::{
    board::{pos::Pos, Board},
    piece::Piece,
};

mod movement;

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

// fn move_piece(play: &Play, pos: &PlayMove) {
//     if pos.p.c == Color::Black && !is_black_turn(p)  {
//         return;
//     }
//     if pos.p.c == Color::White && !is_white_turn(p)  {
//         return;
//     }
//     if is_check && aftermoveischeck() return;
//     if let Some() 
//         capture
//     else {
//         after move
//         if 50 moves with no capture return MoveResult::Stalemate
//     }
// }

// fn get_moves(play: &Play, pos: &Pos) -> Vec<Pos> {
//    
// }



#[cfg(test)]
mod test {
    use super::*;

    use crate::play::movement::Movement;

    #[test]
    fn test_is_white_turn() {
        assert_eq!(is_white_turn(&Vec::new()), true);
        assert_eq!(is_white_turn(&Vec::from([Movement::of_str("♟", "D2", "D4")])), false);
        assert_eq!(is_white_turn(&Vec::from([Movement::of_str("♟", "D2", "D4"), Movement::of_str("♟", "A2", "A3")])), true);
    }

    #[test]
    fn test_is_black_turn() {
        assert_eq!(is_black_turn(&Vec::new()), false);
        assert_eq!(is_black_turn(&Vec::from([Movement::of_str("♟", "D2", "D4")])), true);
        assert_eq!(is_black_turn(&Vec::from([Movement::of_str("♟", "D2", "D4"),Movement::of_str("♟", "A2", "A3") ])), false);
    }
}