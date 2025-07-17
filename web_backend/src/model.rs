use std::collections::HashSet;

use web_sys::CanvasRenderingContext2d;

use libre_chess_lib::game::{
    mode::standard_chess, rule::init::game_of_mode, selection::Selection, game::Game,
};

use crate::{
    board_color::{board_color_purple, BoardColor},
    board_set::{board_set_normal_1, BoardSet},
    render::RenderSettings,
};

#[derive(Debug, PartialEq)]
pub struct Settings {
    pub render_settings: RenderSettings,
    pub board_set: BoardSet,
    pub board_set_id: String,
    pub board_color: BoardColor,
    pub board_color_id: String,
}

#[derive(Debug, PartialEq)]
pub struct Model {
    pub game: Game,
    pub settings: Settings,
    pub context: Option<CanvasRenderingContext2d>,
    pub selection: Selection,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            game: game_of_mode(standard_chess()),
            settings: Settings {
                render_settings: RenderSettings { dim: 0 },
                board_set: board_set_normal_1(),
                board_set_id: "normal_1".into(),
                board_color: board_color_purple(),
                board_color_id: "purple".into(),
            },
            context: None,
            selection: Selection {
                selected_squares: HashSet::new(),
                selected_pos: None,
            },
        }
    }
}
