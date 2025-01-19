use app::{app_init, app_set_dimension};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CanvasRenderingContext2d;

mod app;
mod domain;

#[wasm_bindgen(js_name = "engineInit")]
pub fn main_init(value: CanvasRenderingContext2d) {
    app_init(value);
}

#[wasm_bindgen(js_name = "engineSetDimension")]
pub fn main_set_dimension(dim: u16) {
    app_set_dimension(dim);
}
