use app::{
    app_add_on_change_listener, app_click, app_get_settings, app_init, app_set_board_color,
    app_set_board_set, app_set_dim,
};
use board_color::get_board_color_presets;
use board_set::get_board_set_presets;
use serde::Serialize;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};
use web_sys::{js_sys::Function, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

mod app;
mod board_color;
mod board_set;
mod render;

#[derive(Serialize)]
pub struct Preset {
    pub id: String,
    pub name: String,
}

#[wasm_bindgen]
pub struct EngineInfo {
    board_set: String,
    board_color: String,
}

#[wasm_bindgen]
impl EngineInfo {
    #[wasm_bindgen(getter)]
    pub fn board_set(&self) -> String {
        self.board_set.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn board_color(&self) -> String {
        self.board_color.clone()
    }
}

#[wasm_bindgen(js_name = "engineInit")]
pub fn main_init(canvas: HtmlCanvasElement) {
    if let Ok(Some(context)) = canvas.get_context("2d") {
        app_init(context.dyn_into::<CanvasRenderingContext2d>().map_err(|_| ()).unwrap());
        let closure = Closure::wrap(Box::new(move |e: MouseEvent| {
            if let Some(Ok(element)) =
                e.current_target().map(|target| target.dyn_into::<HtmlCanvasElement>())
            {
                let x = e.page_x() - element.offset_left();
                let y = e.page_y() - element.offset_top();
                app_click(y as u16, x as u16);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

#[wasm_bindgen(js_name = "engineSetDimension")]
pub fn main_set_dim(dim: u16) {
    app_set_dim(dim);
}

#[wasm_bindgen(js_name = "engineSetBoardColor")]
pub fn main_set_board_color(board_color: &str) {
    app_set_board_color(board_color);
}

#[wasm_bindgen(js_name = "engineSetBoardSet")]
pub fn main_set_board_set(board_set: &str) {
    app_set_board_set(board_set);
}

#[wasm_bindgen(js_name = "engineGetSettings")]
pub fn main_get_settings() -> EngineInfo {
    let settings = app_get_settings();
    EngineInfo { board_color: settings.board_color, board_set: settings.board_set }
}

#[wasm_bindgen(js_name = "engineGetBoardSetPresets")]
pub fn main_get_board_set_presets() -> JsValue {
    let groups: Vec<Preset> = get_board_set_presets()
        .iter()
        .map(|g| Preset { id: g.id.into(), name: g.name.into() })
        .collect();
    serde_wasm_bindgen::to_value(&groups).unwrap()
}

#[wasm_bindgen(js_name = "engineGetBoardColorPresets")]
pub fn main_get_board_color_presets() -> JsValue {
    let groups: Vec<Preset> = get_board_color_presets()
        .iter()
        .map(|g| Preset { id: g.id.into(), name: g.name.into() })
        .collect();
    serde_wasm_bindgen::to_value(&groups).unwrap()
}

#[wasm_bindgen(js_name = "engineAddOnChangeListener")]
pub fn main_add_on_change_listener(cb: Function) {
    app_add_on_change_listener(move |_| {
        cb.call0(&JsValue::null()).unwrap();
    });
}
