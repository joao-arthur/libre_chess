use std::collections::HashSet;

use crate::{board::pos::Pos, color::Color, game::board::Board, piece::Type};

use super::{
    movement::{
        Movement,
        naive::{naive_movements_board, naive_movements_piece},
    },
    Game,
    turn::get_turn,
};

fn set_board(play: &mut Game, board: Board) {
    play.board = board;
    for (color, player) in play.players.iter_mut() {
        if color != &Color::White {
            player.possible_movements = naive_movements_board(&play.board, &color);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        board::pos::Pos,
        color::Color,
        game::{mode::chess_standard, game::Game, player::Player},
    };


}
