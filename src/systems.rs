
use nalgebra::Vector2;
use seed::log;
use specs::prelude::*;
use crate::components::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use seed::prelude::ElRef;
pub struct Renderer {
    pub canv_ref: ElRef<HtmlCanvasElement>,
}
impl<'a> System<'a> for Renderer {

    type SystemData = (
        ReadStorage<'a, Dimension>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Origin>,
    );
    fn run(&mut self, (dims, poss, origins): Self::SystemData) {
        let canvas = self.canv_ref.get().expect("get canvas element");
        let ctx = seed::canvas_context_2d(&canvas);
        ctx.set_fill_style(&JsValue::from("#000000"));
        for (dim, pos, _orig) in (&dims, &poss, &origins).join() {
            ctx.fill_rect(pos.x, pos.y, dim.w, dim.h);
        }
    }
}

pub enum MouseEvent {
    MouseDown,
    MouseUp,
    Dragging {
        offset_from_mouse: Vector2<f64>,
        new_position: Vector2<f64>,
    },
    HoverStart,
    HoverStop
}

pub struct Hover;

#[derive(Default)]
pub struct MousePos {
    x: f64,
    y: f64,
}

impl<'a> System<'a> for Hover {

    type SystemData = (
        ReadStorage<'a, Dimension>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Origin>,
        Read<'a, MousePos>,
        ReadStorage<'a, Interactable>,
    );
    fn run(&mut self, (dims, poss, origins, mouse_pos, _): Self::SystemData) {
        log!("checking hover");
    }
}
