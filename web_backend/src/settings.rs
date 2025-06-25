use std::collections::HashSet;

use libre_chess_lib::board::pos::Pos;

use crate::{board_color::BoardColor, board_set::BoardSet, render::RenderSettings};

#[derive(Debug, PartialEq)]
pub struct AppSettings {
    pub render_settings: RenderSettings,
    pub board_set: BoardSet,
    pub board_set_id: String,
    pub board_color: BoardColor,
    pub board_color_id: String,
    pub selected_squares: HashSet<Pos>,
    pub selected_piece: Option<Pos>,
    pub selected_piece_movements: HashSet<Pos>,
}
