use libre_chess_lib::{
    board::Board,
    piece::{Color, Type},
    play::Play,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    console, js_sys, window, Blob, BlobPropertyBag, CanvasRenderingContext2d, HtmlImageElement, Url,
};

use crate::{
    render::{get_values_to_render, RenderSettings},
    sets::{set_1, set_maurizio_monge_fantasy},
};

#[derive(Debug, PartialEq, Clone)]
pub struct AppSettings {
    pub render_settings: RenderSettings,
    // pub board_set: BoardSet,
    // pub board_colors:
    // pub selected_squares: Vec<BoardPos>
}

pub struct Model {
    pub play: Play,
    pub settings: AppSettings,
    pub context: Option<CanvasRenderingContext2d>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            play: Play::default(),
            settings: AppSettings { render_settings: RenderSettings { dim: 0 } },
            context: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Prop {
    Dim,
}

thread_local! {
    static MODEL: RefCell<Model> = RefCell::new(Model::default());
}

thread_local! {
    static LISTENERS: RefCell<Vec<Box<dyn FnMut(Prop) + 'static>>> = RefCell::new(Vec::new());
}

pub fn add_on_change_listener<F>(cb: F)
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

pub fn app_init(context: CanvasRenderingContext2d) {
    MODEL.with(|i| {
        let mut model = i.borrow_mut();
        model.context = Some(context);
        model.play.board = Board::get_initial_board();
    });
    add_on_change_listener({
        move |prop| {
            render();
        }
    });
    render();
}

pub fn app_set_dimension(dim: u16) {
    MODEL.with(|i| {
        let mut m = i.borrow_mut();
        m.settings.render_settings.dim = dim;
    });
    on_change(Prop::Dim);
}

const BLACK: &str = "#b88762";
const WHITE: &str = "#edd6b0";

pub fn app_render() {
    MODEL.with(|i| {
        let m = i.borrow();
        let board = &m.play.board;
        let settings = &m.settings;
        let context = &m.context;
        if let Some(context) = context {
            let dimmm = settings.render_settings.dim as f64;
            let cell_size = dimmm / 8.0;
            let mut acc = 0;
            for row in 0..8 {
                acc += 1;
                for col in 0..8 {
                    if acc % 2 == 0 {
                        context.set_fill_style(&BLACK.into());
                    } else {
                        context.set_fill_style(&WHITE.into());
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
            let set_ = set_1();
            let window = window().unwrap();
            for v in values_to_render {
                let piece_str = match v.p.c {
                    Color::White => match v.p.t {
                        Type::Rook => &set_.wr,
                        Type::Knight => &set_.wn,
                        Type::Bishop => &set_.wb,
                        Type::Queen => &set_.wq,
                        Type::King => &set_.wk,
                        Type::Pawn => &set_.wp,
                    },
                    Color::Black => match v.p.t {
                        Type::Rook => &set_.br,
                        Type::Knight => &set_.bn,
                        Type::Bishop => &set_.bb,
                        Type::Queen => &set_.bq,
                        Type::King => &set_.bk,
                        Type::Pawn => &set_.bp,
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
                        Url::revoke_object_url(&url).unwrap();
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