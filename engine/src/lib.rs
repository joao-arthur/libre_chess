use app::{app_init, app_set_dimension};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CanvasRenderingContext2d;

mod app;
mod domain;

#[wasm_bindgen]
#[derive(Clone)]
pub struct EngineMatrixPoint {
    pub row: u64,
    pub col: u64,
}

#[wasm_bindgen]
impl EngineMatrixPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(row: u64, col: u64) -> EngineMatrixPoint {
        EngineMatrixPoint { row, col }
    }
}

#[wasm_bindgen(js_name = "engineInit")]
pub fn main_init(value: CanvasRenderingContext2d) {
    app_init(value);
}

#[wasm_bindgen(js_name = "engineSetDimension")]
pub fn main_set_dimension(dim: u16) {
    app_set_dimension(dim);
}

#[wasm_bindgen(js_name = "engineClick")]
pub fn main_click(point: EngineMatrixPoint) {


}
