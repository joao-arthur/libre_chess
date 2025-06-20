use crate::{
    board::pos::Pos,
    game::{board::Board, movement::Movement},
};

pub fn white_king_can_short_castling(board: &Board, history: &Vec<Movement>) -> bool {
    board.get(&Pos::of_str("F1")).is_none()
        && board.get(&Pos::of_str("G1")).is_none()
        && history.iter().find(|mov| mov.from == Pos::of_str("H1")).is_none()
        && history.iter().find(|mov| mov.from == Pos::of_str("E1")).is_none()
}

pub fn black_king_can_short_castling(board: &Board, history: &Vec<Movement>) -> bool {
    board.get(&Pos::of_str("F8")).is_none()
        && board.get(&Pos::of_str("G8")).is_none()
        && history.iter().find(|mov| mov.from == Pos::of_str("H8")).is_none()
        && history.iter().find(|mov| mov.from == Pos::of_str("E8")).is_none()
}

pub fn white_king_can_long_castling(board: &Board, history: &Vec<Movement>) -> bool {
    board.get(&Pos::of_str("D1")).is_none()
        && board.get(&Pos::of_str("C1")).is_none()
        && board.get(&Pos::of_str("B1")).is_none()
        && history.iter().find(|mov| mov.from == Pos::of_str("A1")).is_none()
        && history.iter().find(|mov| mov.from == Pos::of_str("E1")).is_none()
}

pub fn black_king_can_long_castling(board: &Board, history: &Vec<Movement>) -> bool {
    board.get(&Pos::of_str("D8")).is_none()
        && board.get(&Pos::of_str("C8")).is_none()
        && board.get(&Pos::of_str("B8")).is_none()
        && history.iter().find(|mov| mov.from == Pos::of_str("A8")).is_none()
        && history.iter().find(|mov| mov.from == Pos::of_str("E8")).is_none()
}
