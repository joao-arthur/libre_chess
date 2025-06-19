use crate::{board::pos::Pos, game::{board::Board, movement::Movement}};

pub fn can_short_castling(board: &Board, history: &Vec<Movement>) -> bool {
    board.get(&Pos::of_str("F1")).is_none() &&
    board.get(&Pos::of_str("G1")).is_none() &&
    history.iter().find(|mov| mov.from == Pos::of_str("H1")).is_none() &&
    history.iter().find(|mov| mov.from == Pos::of_str("E1")).is_none()
}

pub fn can_long_castling(board: &Board, history: &Vec<Movement>) -> bool {
    board.get(&Pos::of_str("D1")).is_none() &&
    board.get(&Pos::of_str("C1")).is_none() &&
    board.get(&Pos::of_str("B1")).is_none() &&
    history.iter().find(|mov| mov.from == Pos::of_str("A1")).is_none() &&
    history.iter().find(|mov| mov.from == Pos::of_str("E1")).is_none()
}
