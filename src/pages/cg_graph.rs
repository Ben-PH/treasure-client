use crate::systems::*;
use crate::components::*;
use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use seed::{prelude::*, *};
use specs::prelude::*;
use std::collections::HashMap;
use web_sys::HtmlCanvasElement;

pub const WIDTH: usize = 900;
pub const HEIGHT: usize = 600;
const RAD: u32 = 50;

pub struct Model {
    // pub pet: DiGraph<UiButton, f32>,
    canvas: ElRef<HtmlCanvasElement>,
    pub world: specs::World,
    tics: usize,
}
impl Model {
    // fn detect_hover(&mut self, mouse_pos: (f32, f32)) {
    //     let positions = self.world.read_storage::<Pos>();
    //     for (pos, mut col) in (&positions, &mut cols).join() {
    //         if (mouse_pos.0 - pos.x as f32) * (mouse_pos.0 - pos.x as f32) + (mouse_pos.1 - pos.y) * (mouse_pos.1 - pos.y) < (RAD * RAD) as f32
    //         {
    //             col.b = 0;
    //         }
    //     }
    //     // log!(mouse_pos);
    // }
}

impl Default for Model {
    fn default() -> Self {
        let mut world = World::new();
        // world.register::<Position>();
        world.register::<Origin>();
        world.register::<Interactable>();
        world.register::<Dimension>();
        world.register::<Pos>();
        world.insert(MousePos::default());
        // world.register::<Color>();
        Self {
            tics: 0,
            // pet: Default::default(),
            // fill_color: Color { r: 0, g: 255, b: 0 },
            canvas: Default::default(),
            world,
        }
    }
}

// #[derive(Debug, Default)]
// struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
// }

// impl Color {
//     fn html_str(&self) -> String {
//         format!("#{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
//     }
// }

#[derive(Debug)]
pub enum Message {
    FetchCGGraph,
    MakeDummyCGGraph,
    CGGraph(fetch::Result<CGGraph>),
    OnTick(RenderInfo),
    CanvasMouse(web_sys::MouseEvent, Ev),
    DotFile,
    Rendered,
    ChangeColor,
}

#[derive(Debug, Default, Serialize, Deserialize, Component)]
#[storage(VecStorage)]
pub struct ConsensusGoal {
    pub id: usize,
    pub plugged: bool,
    pub st8mnt: String,
    pub weight: f32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsensusEdge {
    pub id: usize,
    pub label: String,
    pub left: usize,
    pub right: usize,
    pub weight: f32,
}

pub type CGGraph = (Vec<ConsensusGoal>, Vec<ConsensusEdge>);
// fn draw(
//     model: &Model
//     // model: &petgraph::graph::DiGraph<CGNode, f32>,
//     // canvas: &ElRef<HtmlCanvasElement>,
//     // fill_color: &Color,
// ) {
//     let canvas = model.canvas.get().expect("get canvas element");
//     let ctx = seed::canvas_context_2d(&canvas);

//     ctx.rect(0., 0., (WIDTH as u32).into(), (HEIGHT as u32).into());
//     ctx.set_fill_style(&JsValue::from_str(&model.fill_color.html_str()));
//     ctx.fill();

//     let row_count: u32 = (WIDTH as u32) / (RAD*2);
//     for (i, node) in model.pet.node_references().enumerate() {
//         ctx.begin_path();
//         let x: f64 = (node.1).pos_x;
//         let y: f64 = (node.1).pos_y;
//         ctx.set_fill_style(&JsValue::from_str(&node.1.color.html_str()));
//         ctx.arc(x, y, RAD.into(), 0.0, std::f64::consts::PI * 2.);
//         ctx.fill();
//     }
// }
pub fn update(msg: Message, mdl: &mut Model, orders: &mut impl Orders<Message>) {
    use Message::*;
    match msg {
        Message::OnTick(rend_inf) => {
            let mut rendy = Renderer{canv_ref: mdl.canvas.clone()};
            rendy.run_now(&mdl.world);
            orders.after_next_render(Message::OnTick);
        }
        // Message::ChangeColor => std::mem::swap(&mut mdl.fill_color.b, &mut mdl.fill_color.g),
        // Message::Rendered => {
        //     draw(&mdl);
        //     // We want to call `.skip` to prevent infinite loop.
        //     // (However infinite loops are useful for animations.)
        //     orders.after_next_render(|_| Message::Rendered).skip();
        // }
        MakeDummyCGGraph => {
            let dummy: self::CGGraph = (vec![ConsensusGoal::default(), ConsensusGoal::default()], vec![]);
            orders.perform_cmd(async{CGGraph(Ok(dummy))});
        }
        FetchCGGraph => {
            log!("F");
            orders.perform_cmd(async { CGGraph(fetch_cg_graph().await) });
        }
        CGGraph(Ok(res)) => {
            let mut gr = DiGraph::<ConsensusGoal, f32>::new();
            let mut idx_map: HashMap<usize, NodeIndex> = HashMap::with_capacity(res.0.len());
            let row_count: u32 = (WIDTH as u32) / (RAD * 2);
            for (i, node) in res.0.into_iter().enumerate() {
                let x = ((i as u32 % (row_count as u32)) * (RAD * 2)) as f64;
                let y = ((i as u32 / (row_count as u32)) * (RAD * 2)) as f64;
                log!((x,y));
                let dim = Dimension{
                    w: RAD as f64,
                    h: RAD as f64
                };
                let pos = Pos{x: x as f64, y: y as f64};
                let origin = Origin::Center;
                let r =  255 / (i as u8 + 1);
                let g =  255 - (255 / (i as u8 + 1));
                let b =  255;
                // let col = crate::ametheed::ui::colored_box::UiColorBox::SolidColor(Color{r,g,b});
                mdl.world
                         .create_entity()
                         .with(pos)
                         .with(dim)
                         .with(origin)
                         .with(Interactable::default())
                         .build();
                let idx = gr.add_node(node);
                idx_map.insert(gr.raw_nodes()[idx.index()].weight.id, idx);
            }
            for edge in res.1.into_iter() {
                gr.add_edge(
                    *idx_map.get(&edge.left).unwrap(),
                    *idx_map.get(&edge.right).unwrap(),
                    edge.weight,
                );
            }
            // mdl.pet = gr;
            // for node in mdl.pet.raw_nodes() {
            //     let node = &node.weight;
            //     mdl.specs
            //         .inner
            //         .create_entity()
            //         .with(Position {
            //             x: node.pos_x as f32,
            //             y: node.pos_y as f32,
            //         })
            //         .with(ecs::components::Color {
            //             r: node.color.r,
            //             g: node.color.g,
            //             b: node.color.b,
            //         })
            //         .with(Renderable)
            //         .build();
            // }
            // log!(mdl.pet.raw_nodes());
            orders.after_next_render(Message::OnTick);
        }
        // DotFile => log!(Dot::with_config(&mdl.pet, &[Config::EdgeNoLabel])),
        CanvasMouse(ws_ev, ev) => {
            match ev {
                Ev::MouseDown => {
                    let ox = mdl.canvas.get().unwrap().offset_left() as f64
                        - web_sys::window().unwrap().page_x_offset().unwrap();
                    let oy = mdl.canvas.get().unwrap().offset_top() as f64
                        - web_sys::window().unwrap().page_y_offset().unwrap();
                    let canv_pos = (ws_ev.client_x() as f64 - ox, ws_ev.client_y() as f64 - oy);
                    Interactable::MouseDown(canv_pos.0, canv_pos.1).run_now(&mdl.world);
                    orders.after_next_render(Message::OnTick);
                }
                Ev::MouseUp => {
                    Interactable::MouseUp.run_now(&mdl.world);
                    orders.after_next_render(Message::OnTick);
                }
                Ev::Click => {log!("click")}
                Ev::DblClick => {log!("doubleclick")}
                Ev::MouseMove => {
                    let ox = mdl.canvas.get().unwrap().offset_left() as f64
                        - web_sys::window().unwrap().page_x_offset().unwrap();
                    let oy = mdl.canvas.get().unwrap().offset_top() as f64
                        - web_sys::window().unwrap().page_y_offset().unwrap();
                    let canv_pos = (ws_ev.client_x() as f64 - ox, ws_ev.client_y() as f64 - oy);
                    let mut up_pos = UpdateMousePos{x: canv_pos.0, y: canv_pos.1};
                    up_pos.run_now(&mdl.world);
                    Interactable::Hover.run_now(&mdl.world);
                    Drag.run_now(&mdl.world);
                    orders.after_next_render(Message::OnTick);
                }
                _ => {log!("unhandled event")}
            }
            // mdl.detect_hover((x as f32, y as f32));
        }
        // Task(Ok((id, res))) => {
        //     mdl.subjects.get_mut(&id).unwrap().learning_objectives = res;
        // }
        _ => log!("impl me", msg),
    }
}

async fn fetch_cg_graph() -> fetch::Result<CGGraph> {
    let result = Request::new("api/graph/cg_graph")
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await?;
    Ok(result)
}

pub fn view(model: &Model) -> Node<Message> {
    ul![
        li![button![
            "get cg_graph with backend",
            ev(Ev::Click, |_| Message::FetchCGGraph)
        ]],
        li![button![
            "get cg_graph without backend",
            ev(Ev::Click, |_| Message::MakeDummyCGGraph)
        ]],
        canvas![
            el_ref(&model.canvas),
            attrs![
                At::Width => px(WIDTH),
                At::Height => px(HEIGHT),
            ],
            style![
                St::Border => "1px solid black",
            ],
            mouse_ev(Ev::MouseDown, |mouse_event| {
                Message::CanvasMouse(mouse_event.unchecked_into(), Ev::MouseDown)
            }),
            mouse_ev(Ev::MouseUp, |mouse_event| Message::CanvasMouse(
                mouse_event.unchecked_into(), Ev::MouseUp
            )),
            mouse_ev(Ev::Click, |mouse_event| Message::CanvasMouse(
                mouse_event.unchecked_into(), Ev::Click
            )),
            mouse_ev(Ev::DblClick, |mouse_event| Message::CanvasMouse(
                mouse_event.unchecked_into(), Ev::DblClick
            )),
            mouse_ev(Ev::MouseMove, |mouse_event| Message::CanvasMouse(
                mouse_event.unchecked_into(), Ev::MouseMove
            ))
        ],
        button!["Change color", ev(Ev::Click, |_| Message::ChangeColor)],
        button!["get .dot file", ev(Ev::Click, |_| Message::DotFile)],
        // li![format!("{:?}", model)]
    ]
}
