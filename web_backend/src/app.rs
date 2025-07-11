use core::f64;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    js_sys, window, Blob, BlobPropertyBag, CanvasRenderingContext2d, HtmlImageElement, Url,
};

use libre_chess_lib::{
    board::pos::Pos,
    color::Color,
    game::{
        mode::standard_chess,
        movement::movement::GameMovement,
        rule::{move_piece::app_move_piece, turn::evaluate_turn},
        selection::{toggle_selection, Selection},
        Game,
    },
    movement::Movement,
    piece::Type,
};

use crate::{
    app_info::AppInfo, board_color::try_get_board_color, board_set::try_get_board_set,
    model::Model, prop::Prop, render::get_values_to_render,
};

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
        model.game.board = standard_chess().initial_board;
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
            m.settings.board_color_id = board_color.into();
        });
        on_change(Prop::BoardColor);
    }
}

pub fn app_set_board_set(board_set: &str) {
    if let Some(preset) = try_get_board_set(board_set) {
        MODEL.with(|i| {
            let mut m = i.borrow_mut();
            m.settings.board_set = preset;
            m.settings.board_set_id = board_set.into();
        });
        on_change(Prop::BoardSet);
    }
}

pub fn app_render() {
    MODEL.with(|i| {
        let m = i.borrow();
        let board = &m.game.board;
        let settings = &m.settings;
        let selection = &m.selection;
        let context = &m.context;
        if let Some(context) = context {
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
                let piece_str = match v.piece.color {
                    Color::White => match v.piece.t {
                        Type::Rook => &settings.board_set.wr,
                        Type::Knight => &settings.board_set.wn,
                        Type::Bishop => &settings.board_set.wb,
                        Type::Queen => &settings.board_set.wq,
                        Type::King => &settings.board_set.wk,
                        Type::Pawn => &settings.board_set.wp,
                    },
                    Color::Black => match v.piece.t {
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
            if selection.selected_squares.len() > 0 {
                context.set_fill_style(&"#f0ec0088".into());
                selection.selected_squares.iter().for_each(|pos| {
                    context.fill_rect(
                        pos.col as f64 * cell_size,
                        (settings.render_settings.dim as f64)
                            - (pos.row as f64) * cell_size
                            - cell_size,
                        cell_size,
                        cell_size,
                    );
                })
            }
            if selection.selected_piece_movements.len() > 0 {
                context.set_fill_style(&"#3d3d3dee".into());
                selection.selected_piece_movements.iter().for_each(|movement| {
                    let maybe_pos = match movement {
                        GameMovement::Default(mov) => Some(&mov.movement),
                        GameMovement::EnPassant(mov) => Some(&mov.movement),
                        GameMovement::Castling(mov) => Some(&mov.movement),
                        _ => None,
                    };
                    if let Some(mov) = maybe_pos {
                        context.begin_path();
                        let _ = context.arc(
                            mov.to.col as f64 * cell_size + cell_size / 2.0,
                            ((settings.render_settings.dim as f64)
                                - (mov.to.row as f64) * cell_size
                                - cell_size)
                                + cell_size / 2.0,
                            cell_size / (2.0 * f64::consts::PI),
                            0.0,
                            2.0 * f64::consts::PI,
                        );
                        context.fill();
                    }
                })
            }
        }
    });
}

pub fn app_click(row: u16, col: u16) {
    let dim = MODEL.with(|m| m.borrow().settings.render_settings.dim as f64);
    let bounds = MODEL.with(|m| m.borrow().game.bounds);

    MODEL.with(|i| {
        let mut m = i.borrow_mut();
        let Model { game: Game { board, bounds, players, history }, selection, .. } = &mut *m;
        let cell_size = dim / 8.0;
        let cell_row = (8 - ((((row as f64) / cell_size).floor() as u8) as i16)) as u8 - 1;
        let cell_col = ((col as f64) / cell_size).floor() as u8;
        let pos = Pos { row: cell_row, col: cell_col };

        if let Some(movement) = &selection.selected_piece_movements.iter().find(|mov| match mov {
            GameMovement::Default(mov) => mov.movement.to == pos,
            GameMovement::EnPassant(mov) => mov.movement.to == pos,
            GameMovement::Castling(mov) => mov.movement.to == pos,
            _ => false,
        }) {
            app_move_piece(board, &bounds, players, history, movement);
        }
        toggle_selection(selection, board, players, history, pos.clone());
    });
    on_change(Prop::BoardSet);
}
