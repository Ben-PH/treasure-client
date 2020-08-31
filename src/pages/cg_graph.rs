mod ecs_stuff;
use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
use seed::{prelude::*, *};
use shared::learning_trajectory;
use std::collections::HashMap;
use web_sys::{HtmlCanvasElement, MouseEvent};

const WIDTH: usize = 450;
const HEIGHT: usize = 300;
const RAD: u32 = 10;

#[derive(Debug)]
pub struct Model {
    pub pet: DiGraph<CGNode, f32>,
    fill_color: Color,
    canvas: ElRef<HtmlCanvasElement>,
    specs: ecs_stuff::MyWorld,
}


impl Default for Model {
    fn default() -> Self {
        Self {
            pet: Default::default(),
            fill_color: Color{r: 0, g: 255, b: 0},
            canvas: Default::default(),
            specs: ecs_stuff::MyWorld::init_world(),
        }
    }
}

#[derive(Debug, Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn html_str(&self) -> String {
        format!("#{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
    }
}



struct Myf32(f32);

impl Default for Myf32 {
    fn default() -> Self {
        Myf32(0.0)
    }
}

#[derive(Debug)]
pub enum Message {
    FetchCGGraph,
    CGGraph(fetch::Result<learning_trajectory::CGGraph>),
    CanvasMouse(MouseEvent),
    DotFile,
    Rendered,
    ChangeColor,
}



#[derive (Debug)]
pub struct CGNode {
    color: Color,
    pos_x: f64,
    pos_y: f64,
    cg: shared::learning_trajectory::ConsensusGoal,
}
fn draw(
    model: &Model
    // model: &petgraph::graph::DiGraph<CGNode, f32>,
    // canvas: &ElRef<HtmlCanvasElement>,
    // fill_color: &Color,
) {
    let canvas = model.canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    ctx.rect(0., 0., (WIDTH as u32).into(), (HEIGHT as u32).into());
    ctx.set_fill_style(&JsValue::from_str(&model.fill_color.html_str()));
    ctx.fill();


    let row_count: u32 = (WIDTH as u32) / (RAD*2);
    for (i, node) in model.pet.node_references().enumerate() {
        ctx.begin_path();
        let x: f64 = (node.1).pos_x;
        let y: f64 = (node.1).pos_y;
        log!(&JsValue::from_str(&node.1.color.html_str()));
        ctx.set_fill_style(&JsValue::from_str(&node.1.color.html_str()));
        ctx.arc(x, y, RAD.into(), 0.0, std::f64::consts::PI * 2.);
        ctx.fill();
    }
}
pub fn update(msg: Message, mdl: &mut Model, orders: &mut impl Orders<Message>) {
    use Message::*;
    match msg {
        Message::ChangeColor => {
            std::mem::swap(&mut mdl.fill_color.b, &mut mdl.fill_color.g)
        }
        Message::Rendered => {
            draw(&mdl);
            // We want to call `.skip` to prevent infinite loop.
            // (However infinite loops are useful for animations.)
            orders.after_next_render(|_| Message::Rendered).skip();
        }
        FetchCGGraph => {
            orders.perform_cmd(async { CGGraph(fetch_cg_graph().await) });
        }
        CGGraph(Ok(res)) => {
            let mut gr = DiGraph::<CGNode, f32>::new();
            let mut idx_map: HashMap<usize, NodeIndex> = HashMap::with_capacity(res.0.len());

            let row_count: u32 = (WIDTH as u32) / (RAD*2);
            for (i, node) in res.0.into_iter().enumerate() {
                let x = (RAD + (i as u32 % (row_count as u32)) * (RAD * 2)).into();
                let y = (RAD + (i as u32 / (row_count as u32)) * (RAD * 2)).into();
                let g_node = CGNode {
                    color: Color{r: 255/(i as u8 +1),g: 255 - (255/(i as u8 +1)), b: 255},
                    pos_x: x,
                    pos_y: y,
                    cg: node
                };

                let idx = gr.add_node(g_node);
                idx_map.insert(gr.raw_nodes()[idx.index()].weight.cg.id, idx);
            }
            for edge in res.1.into_iter() {
                gr.add_edge(
                    *idx_map.get(&edge.left).unwrap(),
                    *idx_map.get(&edge.right).unwrap(),
                    edge.weight,
                );
            }
            mdl.pet = gr;
        }
        DotFile => log!(Dot::with_config(&mdl.pet, &[Config::EdgeNoLabel])),
        // Task(Ok((id, res))) => {
        //     mdl.subjects.get_mut(&id).unwrap().learning_objectives = res;
        // }
        _ => log!("impl me", msg),
    }
}

async fn fetch_cg_graph() -> fetch::Result<learning_trajectory::CGGraph> {
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
            "get cg_graph",
            ev(Ev::Click, |_| Message::FetchCGGraph)
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
            mouse_ev(Ev::MouseEnter, |mouse_event| Message::CanvasMouse(mouse_event)),
            mouse_ev(Ev::MouseLeave, |mouse_event| Message::CanvasMouse(mouse_event)),
            mouse_ev(Ev::MouseMove, |mouse_event| Message::CanvasMouse(mouse_event))
        ],
        button!["Change color", ev(Ev::Click, |_| Message::ChangeColor)],
        button!["get .dot file", ev(Ev::Click, |_| Message::DotFile)],
        li![format!("{:?}", model)]
    ]
}
