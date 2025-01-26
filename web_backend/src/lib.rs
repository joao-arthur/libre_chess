use app::{app_init, app_set_dimension};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

mod app;
mod render;
mod board_set;
mod board_color;

#[wasm_bindgen(js_name = "engineInit")]
pub fn main_init(canvas: HtmlCanvasElement) {
    if let Ok(Some(context)) = canvas.get_context("2d") {
        app_init(context.dyn_into::<CanvasRenderingContext2d>().map_err(|_| ()).unwrap());
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            web_sys::console::log_1(
                &format!("Clicked at: {}, {}", event.client_x(), event.client_y()).into(),
            );
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

#[wasm_bindgen(js_name = "engineSetDimension")]
pub fn main_set_dimension(dim: u16) {
    app_set_dimension(dim);
}
