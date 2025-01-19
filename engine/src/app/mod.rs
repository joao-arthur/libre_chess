use std::{cell::RefCell, rc::Rc};
use sets::{set_1, set_2, set_maurizio_monge_fantasy, set_maurizio_monge_spatial};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{console, js_sys, window, Blob, BlobPropertyBag, CanvasRenderingContext2d, HtmlImageElement, Url};
use crate::domain::play::Play;

mod sets;

#[derive(Debug, PartialEq)]
pub struct RectF64 {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RenderSettings {
    pub dim: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AppSettings {
    pub render_settings: RenderSettings,
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
            settings: AppSettings {
                render_settings: RenderSettings { dim: 0 },
            },
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
    MODEL.with(|i| i.borrow_mut().context = Some(context));
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

pub fn render() {
    let (play, settings, context) = MODEL.with(|i| {
        let m = i.borrow();
        (m.play.clone(), m.settings.clone(), m.context.clone())
    });
    if let Some(context) = context {
        let dimmm =settings.render_settings.dim as f64;
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
                context.fill_rect(col as f64 * cell_size, row as f64 * cell_size, cell_size, cell_size);
            }
        }
    // Access the window object
    let window = window().ok_or("No window object available").unwrap();

    // Create a Blob from the SVG string
    let blob = Blob::new_with_str_sequence_and_options(
        &js_sys::Array::of1(&JsValue::from_str(&set_maurizio_monge_spatial().bb)),
        BlobPropertyBag::new().type_("image/svg+xml"),
    ).unwrap();

    // Create an object URL for the Blob
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Create an Image element wrapped in Rc
    let img = Rc::new(HtmlImageElement::new().unwrap());
    img.set_src(&url);

    // Clone Rc for use in the closure
    let img_clone = Rc::clone(&img);
    let closure = Closure::wrap(Box::new({
        let canvas_ctx = context.clone();
        let url = url.clone();
        move || {
            // Draw the image onto the canvas
            canvas_ctx
                .draw_image_with_html_image_element_and_dw_and_dh(&img_clone, 0.0, 0.0, cell_size, cell_size)
                .unwrap();

            // Revoke the object URL
            Url::revoke_object_url(&url).unwrap();
        }
    }) as Box<dyn FnMut()>);

    // Attach the onload event
    img.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget(); // Prevent the closure from being dropped prematurely


        //     let img1 = new Image();
   //     let svg = new Blob([piece], {type: 'image/svg+xml'});
   //     let url = DOMURL.createObjectURL(svg);
   //     img1.onload = function() {
   //        ctx.drawImage(img1, 25, 70);
   //        DOMURL.revokeObjectURL(url);
   //     }
   //     img1.src = url;
//





    }
}
