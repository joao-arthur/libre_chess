use std::collections::HashSet;

use web_sys::CanvasRenderingContext2d;

use libre_chess_lib::game::{mode::standard_chess, rule::init::init_game, Game};

use crate::{
    board_color::board_color_purple, board_set::board_set_normal_1, render::RenderSettings,
    settings::AppSettings,
};

#[derive(Debug, PartialEq)]
pub struct Model {
    pub game: Game,
    pub settings: AppSettings,
    pub context: Option<CanvasRenderingContext2d>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            game: init_game(standard_chess()),
            settings: AppSettings {
                render_settings: RenderSettings { dim: 0 },
                board_set: board_set_normal_1(),
                board_set_id: "normal_1".into(),
                board_color: board_color_purple(),
                board_color_id: "purple".into(),
                selected_squares: HashSet::new(),
                selected_piece: None,
                selected_piece_movements: HashSet::new(),
            },
            context: None,
        }
    }
}
