
use crate::pages::cg_graph::WIDTH;
use crate::pages::cg_graph::HEIGHT;
use specs::WorldExt;
use nalgebra::Vector2 as Vec2;
use seed::log;
use specs::prelude::*;
use crate::components::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use seed::prelude::ElRef;

pub struct Renderer {
    pub canv_ref: ElRef<HtmlCanvasElement>,
}


pub struct LineDraw {
    pub canv_ref: ElRef<HtmlCanvasElement>,
}
impl<'a> System<'a> for LineDraw {

    type SystemData = (
        ReadStorage<'a, Line>,
    );
    fn run(&mut self, (data, ): Self::SystemData) {
        let canvas = self.canv_ref.get().expect("get canvas element");
        let ctx = seed::canvas_context_2d(&canvas);
        ctx.set_fill_style(&JsValue::from("#000000"));
        for line in (&data).join() {
            ctx.begin_path();
            ctx.move_to(line.start.x, line.start.y);
            ctx.line_to(line.end.x, line.end.y);
            ctx.stroke();
        }
    }
}

impl<'a> System<'a> for Renderer {

    type SystemData = (
        ReadStorage<'a, Dimension>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Origin>,
        ReadStorage<'a, Text>,
        ReadStorage<'a, Edge>,
        Entities<'a>,
        ReadStorage<'a, Interactable>
    );
    fn run(&mut self, (dims, poss, origins, texts, edges, ents, states): Self::SystemData) {
        let canvas = self.canv_ref.get().expect("get canvas element");
        let ctx = seed::canvas_context_2d(&canvas);
        ctx.set_fill_style(&JsValue::from("#000000"));
        ctx.clear_rect(0., 0., WIDTH as f64, HEIGHT as f64);
        let mut black = false;
        for (dim, pos, _orig, state) in (&dims, &poss, &origins,  &states).join() {
            match (black, state) {
                (_, Interactable::Hover) => {

                    ctx.set_fill_style(&JsValue::from("#FF0000"));
                    ctx.fill_rect(pos.x, pos.y, dim.w, dim.h);
                    ctx.set_fill_style(&JsValue::from("#000000"));

                }
                (_, Interactable::MouseDown(_, _)) => {

                    ctx.set_fill_style(&JsValue::from("#00FF00"));
                    ctx.fill_rect(pos.x, pos.y, dim.w, dim.h);
                    ctx.set_fill_style(&JsValue::from("#000000"));

                }
                (_, _) => {

                    ctx.set_fill_style(&JsValue::from("#000000"));
                    ctx.fill_rect(pos.x, pos.y, dim.w, dim.h);

                }
            }
            // ctx.fill_text_with_max_width(&txt.st, pos.x, pos.y, dim.w).unwrap();
            // ctx.fill_rect(pos.x, pos.y, dim.w, dim.h);
        }
        for edge in (&edges).join() {
            let left = poss.get(edge.left).unwrap();
            let right = poss.get(edge.right).unwrap();
            ctx.begin_path();
            ctx.move_to(left.x, left.y);
            ctx.line_to(right.x, right.y);
            ctx.stroke();
        }
    }
}

pub struct Hover;

#[derive(Debug, Default)]
pub struct MousePos {
    pub x: f64,
    pub y: f64,
}
#[derive(Debug, Default)]
pub struct UpdateMousePos {
    pub x: f64,
    pub y: f64,
}


impl<'a> System<'a> for UpdateMousePos {

    type SystemData = Write<'a, MousePos>;
    fn run(&mut self, data: Self::SystemData) {
        let mut mpos = data;
        mpos.x = self.x;
        mpos.y = self.y;
    }
}

pub struct Drag;
impl<'a> System<'a> for Drag {

    type SystemData = (
        WriteStorage<'a, Pos>,
        ReadStorage<'a, Interactable>,
        Read<'a, MousePos>
    );
    fn run(&mut self, (mut poss, states, mpos): Self::SystemData) {

        for (mut pos, st8) in (&mut poss, &states).join() {
            match st8  {
                Interactable::MouseDown(x, y) => {
                    pos.x = mpos.x - x;
                    pos.y = mpos.y - y;
                }
                _ => {}
            }
        }
    }
}


impl<'a> System<'a> for Interactable {

    type SystemData = (
        ReadStorage<'a, Dimension>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Origin>,
        WriteStorage<'a, Interactable>,
        Read<'a, MousePos>
    );
    fn run(&mut self, (dims, poss, origins, mut inter, mpos): Self::SystemData) {
        for (dims, pos, inter) in (&dims, &poss, &mut inter).join() {
            let hovering = mpos.x > pos.x && mpos.x < pos.x + dims.w && mpos.y > pos.y && mpos.y < pos.y + dims.h;
            match (hovering, &*inter) {
                (true, Interactable::Nothing) => *inter = Interactable::Hover,
                (true, Interactable::Hover) => {
                    match self {
                        Interactable::MouseDown(a, b) => *inter = Interactable::MouseDown(*a - pos.x, *b - pos.y),
                        Interactable::MouseUp => *inter = Interactable::MouseUp,
                        Interactable::Nothing => unreachable!(),
                        Interactable::Hover => {}
                    }
                }
                (_, Interactable::MouseDown(_,_)) => {
                    if *self == Interactable::MouseUp {
                        *inter = Interactable::Hover
                    }
                }
                (true, Interactable::MouseUp) => *inter = Interactable::Hover,
                (false, Interactable::Hover) => *inter = Interactable::Nothing,
                _ => {}
            }
        }
    }
}

pub enum Systems {
    MouseEvent,
    Hover,
    Renderer,
}
