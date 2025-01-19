use std::cell::RefCell;
use web_sys::CanvasRenderingContext2d;
use crate::domain::play::Play;

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
    }
}
