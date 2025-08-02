use core::f64;
use std::{cell::RefCell, collections::HashMap};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    js_sys, window, Blob, BlobPropertyBag, CanvasRenderingContext2d, HtmlImageElement, Url,
};

use libre_chess_lib::{
    color::Color,
    game::{
        game::Game,
        rule::{
            check::is_in_check, legal_moves::legal_moves_of_player, move_piece::move_piece,
            turn::evaluate_turn,
        },
        selection::toggle_selection,
    },
    piece::PieceType,
    pos::Pos,
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
        let strs = [
            ("bb", model.settings.board_set.bb),
            ("bk", model.settings.board_set.bk),
            ("bn", model.settings.board_set.bn),
            ("bp", model.settings.board_set.bp),
            ("bq", model.settings.board_set.bq),
            ("br", model.settings.board_set.br),
            ("wb", model.settings.board_set.wb),
            ("wk", model.settings.board_set.wk),
            ("wn", model.settings.board_set.wn),
            ("wp", model.settings.board_set.wp),
            ("wq", model.settings.board_set.wq),
            ("wr", model.settings.board_set.wr),
        ];
        for (name, str_img) in strs {
            let maybe_blob = Blob::new_with_str_sequence_and_options(
                &js_sys::Array::of1(&JsValue::from_str(str_img)),
                BlobPropertyBag::new().type_("image/svg+xml"),
            );
            if let Ok(blob) = maybe_blob {
                let maybe_url = Url::create_object_url_with_blob(&blob);
                if let Ok(url) = maybe_url {
                    let maybe_image_element = HtmlImageElement::new();
                    if let Ok(image_element) = maybe_image_element {
                        image_element.set_src(&url);
                        let closure = Closure::wrap(Box::new({
                            move || {
                                app_render();
                            }
                        }) as Box<dyn FnMut()>);
                        image_element.set_onload(Some(closure.as_ref().unchecked_ref()));
                        closure.forget();
                        model.board_set.insert(name.into(), image_element);
                    }
                }
            }
        }
    });
    app_add_on_change_listener({
        move |prop| {
            app_render();
        }
    });
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
            let strs = [
                ("bb", m.settings.board_set.bb),
                ("bk", m.settings.board_set.bk),
                ("bn", m.settings.board_set.bn),
                ("bp", m.settings.board_set.bp),
                ("bq", m.settings.board_set.bq),
                ("br", m.settings.board_set.br),
                ("wb", m.settings.board_set.wb),
                ("wk", m.settings.board_set.wk),
                ("wn", m.settings.board_set.wn),
                ("wp", m.settings.board_set.wp),
                ("wq", m.settings.board_set.wq),
                ("wr", m.settings.board_set.wr),
            ];
            for (name, str_img) in strs {
                let maybe_blob = Blob::new_with_str_sequence_and_options(
                    &js_sys::Array::of1(&JsValue::from_str(str_img)),
                    BlobPropertyBag::new().type_("image/svg+xml"),
                );
                if let Ok(blob) = maybe_blob {
                    let maybe_url = Url::create_object_url_with_blob(&blob);
                    if let Ok(url) = maybe_url {
                        let maybe_image_element = HtmlImageElement::new();
                        if let Ok(image_element) = maybe_image_element {
                            image_element.set_src(&url);
                            let closure = Closure::wrap(Box::new({
                                move || {
                                    app_render();
                                }
                            })
                                as Box<dyn FnMut()>);
                            image_element.set_onload(Some(closure.as_ref().unchecked_ref()));
                            closure.forget();
                            m.board_set.insert(name.into(), image_element);
                        }
                    }
                }
            }
        });
        on_change(Prop::BoardSet);
    }
}

// when iniciating the app, must cache the piece image, so it does not flicker
pub fn app_render() {
    MODEL.with(|i| {
        let m = i.borrow();
        let board = &m.game.board;
        let bounds = &m.game.bounds;
        let players = &m.game.players;
        let settings = &m.settings;
        let selection = &m.selection;
        let context = &m.context;
        let board_set = &m.board_set;
        let turn = evaluate_turn(&m.game.history);
        let in_check = is_in_check(&m.game.board, &m.game.players, &m.game.history);
        if let Some(context) = context {
            let dimmm = settings.render_settings.dim as f64;
            let cell_size = dimmm / 8.0;
            let mut acc = 0;
            for row in bounds.iter_row() {
                acc += 1;
                for col in bounds.iter_col() {
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
            let values_to_render = get_values_to_render(board, &bounds, &settings.render_settings);
            let window = window().unwrap();
            for v in values_to_render {
                let piece_str_name = match v.piece.color {
                    Color::White => match v.piece.typ {
                        PieceType::Rook => "wr",
                        PieceType::Knight => "wn",
                        PieceType::Bishop => "wb",
                        PieceType::Queen => "wq",
                        PieceType::King => "wk",
                        PieceType::Pawn => "wp",
                    },
                    Color::Black => match v.piece.typ {
                        PieceType::Rook => "br",
                        PieceType::Knight => "bn",
                        PieceType::Bishop => "bb",
                        PieceType::Queen => "bq",
                        PieceType::King => "bk",
                        PieceType::Pawn => "bp",
                    },
                };
                if in_check && v.piece.typ == PieceType::King && v.piece.color == turn {
                    let grid_horizontal = context.create_linear_gradient(
                        v.rect.min.x,
                        v.rect.min.y,
                        v.rect.max.x,
                        v.rect.min.y,
                    );
                    grid_horizontal.add_color_stop(0.0, "#D20103");
                    grid_horizontal.add_color_stop(0.1, "transparent");
                    grid_horizontal.add_color_stop(0.9, "transparent");
                    grid_horizontal.add_color_stop(1.0, "#D20103");

                    let grid_vertical = context.create_linear_gradient(
                        v.rect.min.x,
                        v.rect.min.y,
                        v.rect.min.x,
                        v.rect.max.y,
                    );
                    grid_vertical.add_color_stop(0.0, "#D20103");
                    grid_vertical.add_color_stop(0.1, "transparent");
                    grid_vertical.add_color_stop(0.9, "transparent");
                    grid_vertical.add_color_stop(1.0, "#D20103");

                    context.set_fill_style(&grid_horizontal.into());
                    context.fill_rect(
                        v.rect.min.x,
                        v.rect.min.y,
                        v.rect.max.x - v.rect.min.x,
                        v.rect.max.y - v.rect.min.y,
                    );
                    context.set_fill_style(&grid_vertical.into());
                    context.fill_rect(
                        v.rect.min.x,
                        v.rect.min.y,
                        v.rect.max.x - v.rect.min.x,
                        v.rect.max.y - v.rect.min.y,
                    );
                }
                if let Some(html_el) = board_set.get(&piece_str_name.to_string()) {
                    context
                        .draw_image_with_html_image_element_and_dw_and_dh(
                            html_el,
                            v.rect.min.x,
                            v.rect.min.y,
                            v.rect.max.x - v.rect.min.x,
                            v.rect.max.y - v.rect.min.y,
                        )
                        .unwrap()
                }
            }
            if !selection.selected_squares.is_empty() {
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
            if let Some(from) = &selection.selected_pos {
                if let Some(selected_piece) = board.get(from) {
                    if let Some(player) = players.get(&selected_piece.color) {
                        if let Some(piece_moves) = player.moves.get(from) {
                            for to in piece_moves.keys() {
                                context.set_fill_style(&"#09056b88".into());
                                context.begin_path();
                                let _ = context.arc(
                                    to.col as f64 * cell_size + cell_size / 2.0,
                                    ((settings.render_settings.dim as f64)
                                        - (to.row as f64) * cell_size
                                        - cell_size)
                                        + cell_size / 2.0,
                                    cell_size / (2.0 * f64::consts::PI),
                                    0.0,
                                    2.0 * f64::consts::PI,
                                );
                                context.fill();
                            }
                        }
                    }
                }
            }
        }
    });
}

pub fn app_click(row: u16, col: u16) {
    let dim = MODEL.with(|m| m.borrow().settings.render_settings.dim as f64);
    let bounds = MODEL.with(|m| m.borrow().game.bounds.clone());

    MODEL.with(|i| {
        let mut m = i.borrow_mut();
        let Model { game: Game { board, players, history, .. }, selection, .. } = &mut *m;
        let cell_size = dim / 8.0;
        let cell_row = (8 - ((((row as f64) / cell_size).floor() as u8) as i16)) as u8 - 1;
        let cell_col = ((col as f64) / cell_size).floor() as u8;
        let pos = Pos { row: cell_row, col: cell_col };
        move_piece(board, history, players, &bounds, selection, &pos);
        toggle_selection(selection, board, players, history, pos);
        let mut tempmoves = HashMap::new();
        let players_temp = players.clone();
        for color in players_temp.keys() {
            tempmoves.insert(
                color,
                legal_moves_of_player(board, &bounds, history, &players_temp, color),
            );
        }
        for (color, moves) in tempmoves {
            players.get_mut(color).unwrap().moves = moves;
        }
    });
    on_change(Prop::BoardSet);
}
