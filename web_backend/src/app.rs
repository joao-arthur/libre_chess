use libre_chess_lib::{
    board::{pos::Pos, Board},
    piece::{Color, Type},
    play::Play,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    console, js_sys, window, Blob, BlobPropertyBag, CanvasRenderingContext2d, HtmlImageElement, Url,
};

use crate::{
    board_color::{self, board_color_purple, try_get_board_color, BoardColor},
    board_set::{board_set_normal_1, try_get_board_set, BoardSet},
    render::{get_values_to_render, RenderSettings},
};

#[derive(Debug, PartialEq)]
pub struct AppSettings {
    pub render_settings: RenderSettings,
    pub board_set: BoardSet,
    pub board_set_id: String,
    pub board_color: BoardColor,
    pub board_color_id: String,
    pub selected_squares: Vec<Pos>,
}

#[derive(Debug, PartialEq)]
pub struct Model {
    pub play: Play,
    pub settings: AppSettings,
    pub context: Option<CanvasRenderingContext2d>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            play: Play::default(),
            settings: AppSettings {
                render_settings: RenderSettings { dim: 0 },
                board_set: board_set_normal_1(),
                board_set_id: String::from("normal_1"),
                board_color: board_color_purple(),
                board_color_id: String::from("purple"),
                selected_squares: Vec::new(),
            },
            context: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Prop {
    BoardColor,
    BoardSet,
    Dim,
}

thread_local! {
    static MODEL: RefCell<Model> = RefCell::new(Model::default());
}

thread_local! {
    static LISTENERS: RefCell<Vec<Box<dyn FnMut(Prop) + 'static>>> = RefCell::new(Vec::new());
}

pub fn app_add_on_change_listener<F>(cb: F)
where
    F: FnMut(Prop) + 'static,
{
    LISTENERS.with_borrow_mut(|l| l.push(Box::new(cb)));
}

fn on_change(param: Prop) {
    LISTENERS.with_borrow_mut(|l| {
        for cb in l.iter_mut() {
            cb(param.clone());
        }
    });
}

#[derive(Debug, PartialEq)]
pub struct AppInfo {
    pub board_set: String,
    pub board_color: String,
}

pub fn app_get_settings() -> AppInfo {
    MODEL.with(|i| {
        let m = i.borrow();
        AppInfo {
            board_set: m.settings.board_set_id.clone(),
            board_color: m.settings.board_color_id.clone(),
        }
    })
}
pub fn app_init(context: CanvasRenderingContext2d) {
    MODEL.with(|i| {
        let mut model = i.borrow_mut();
        model.context = Some(context);
        model.play.board = Board::get_initial_board();
    });
    app_add_on_change_listener({
        move |prop| {
            app_render();
        }
    });
    app_render();
}

pub fn app_set_dim(dim: u16) {
    MODEL.with(|i| {
        let mut m = i.borrow_mut();
        m.settings.render_settings.dim = dim;
    });
    on_change(Prop::Dim);
}

pub fn app_set_board_color(board_color: &str) {
    if let Some(preset) = try_get_board_color(board_color) {
        MODEL.with(|i| {
            let mut m = i.borrow_mut();
            m.settings.board_color = preset;
            m.settings.board_color_id = String::from(board_color);
        });
        on_change(Prop::BoardColor);
    }
}

pub fn app_set_board_set(board_set: &str) {
    if let Some(preset) = try_get_board_set(board_set) {
        MODEL.with(|i| {
            let mut m = i.borrow_mut();
            m.settings.board_set = preset;
            m.settings.board_set_id = String::from(board_set);
        });
        on_change(Prop::BoardSet);
    }
}

pub fn app_render() {
    MODEL.with(|i| {
        let m = i.borrow();
        let board = &m.play.board;
        let settings = &m.settings;
        let context = &m.context;
        if let Some(context) = context {
            context.set_fill_style(&"white".into());
            context.fill_rect(
                0.0,
                0.0,
                settings.render_settings.dim as f64,
                settings.render_settings.dim as f64,
            );
            let dimmm = settings.render_settings.dim as f64;
            let cell_size = dimmm / 8.0;
            let mut acc = 0;
            for row in 0..8 {
                acc += 1;
                for col in 0..8 {
                    if acc % 2 == 0 {
                        context.set_fill_style(&settings.board_color.dark.into());
                    } else {
                        context.set_fill_style(&settings.board_color.light.into());
                    }
                    acc += 1;
                    context.fill_rect(
                        col as f64 * cell_size,
                        row as f64 * cell_size,
                        cell_size,
                        cell_size,
                    );
                }
            }
            let values_to_render = get_values_to_render(board, &settings.render_settings);
            let window = window().unwrap();
            for v in values_to_render {
                let piece_str = match v.p.c {
                    Color::White => match v.p.t {
                        Type::Rook => &settings.board_set.wr,
                        Type::Knight => &settings.board_set.wn,
                        Type::Bishop => &settings.board_set.wb,
                        Type::Queen => &settings.board_set.wq,
                        Type::King => &settings.board_set.wk,
                        Type::Pawn => &settings.board_set.wp,
                    },
                    Color::Black => match v.p.t {
                        Type::Rook => &settings.board_set.br,
                        Type::Knight => &settings.board_set.bn,
                        Type::Bishop => &settings.board_set.bb,
                        Type::Queen => &settings.board_set.bq,
                        Type::King => &settings.board_set.bk,
                        Type::Pawn => &settings.board_set.bp,
                    },
                };
                let blob = Blob::new_with_str_sequence_and_options(
                    &js_sys::Array::of1(&JsValue::from_str(piece_str)),
                    BlobPropertyBag::new().type_("image/svg+xml"),
                )
                .unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let img = Rc::new(HtmlImageElement::new().unwrap());
                img.set_src(&url);
                let img_clone = Rc::clone(&img);
                let closure = Closure::wrap(Box::new({
                    let canvas_ctx = context.clone();
                    let url = url.clone();
                    move || {
                        canvas_ctx
                            .draw_image_with_html_image_element_and_dw_and_dh(
                                &img_clone,
                                v.rect.x1,
                                v.rect.y1,
                                v.rect.x2 - v.rect.x1,
                                v.rect.y2 - v.rect.y1,
                            )
                            .unwrap();
                        // Url::revoke_object_url(&url).unwrap();
                    }
                }) as Box<dyn FnMut()>);
                img.set_onload(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            }
        }
    });
}

pub fn app_click() {
    // x, y
    // get the moves for the piece, implement this function on lib
    // select the square
}
