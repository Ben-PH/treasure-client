use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
use seed::{prelude::*, *};
use shared::learning_trajectory;
use std::collections::HashMap;
use web_sys::HtmlCanvasElement;

const WIDTH: usize = 450;
const HEIGHT: usize = 300;
const RAD: u32 = 10;

#[derive(Default, Debug)]
pub struct Model {
    pub pet: DiGraph<CGNode, f32>,
    fill_color: Color,
    canvas: ElRef<HtmlCanvasElement>,
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
    CanvasClick,
    DotFile,
    Rendered,
    ChangeColor,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
    A,
    B,
}

impl Color {
    fn as_str(&self) -> &str {
        match self {
            Self::A => "blue",
            Self::B => "green",
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::A
    }
}

#[derive (Debug)]
pub struct CGNode {
    color: [u8;3],
    pos_x: f64,
    pos_y: f64,
    cg: shared::learning_trajectory::ConsensusGoal,
}
fn draw(
    model: &petgraph::graph::DiGraph<CGNode, f32>,
    canvas: &ElRef<HtmlCanvasElement>,
    fill_color: Color,
) {
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    ctx.rect(0., 0., (WIDTH as u32).into(), (HEIGHT as u32).into());
    ctx.set_fill_style(&JsValue::from_str(fill_color.as_str()));
    ctx.fill();


    let row_count: u32 = (WIDTH as u32) / (RAD*2);
    for (i, node) in model.node_references().enumerate() {
        ctx.begin_path();
        let x: f64 = (node.1).pos_x;
        let y: f64 = (node.1).pos_y;
        ctx.arc(x, y, RAD.into(), 0.0, std::f64::consts::PI * 2.);
        ctx.set_fill_style(&JsValue::from_str(format!("#{:x}{:x}{:x}", node.1.color[0], node.1.color[1], node.1.color[2]).as_str()));
        ctx.fill();
    }
}
pub fn update(msg: Message, mdl: &mut Model, orders: &mut impl Orders<Message>) {
    use Message::*;
    match msg {
        Message::ChangeColor => {
            mdl.fill_color = if mdl.fill_color == Color::A {
                Color::B
            } else {
                Color::A
            };
        }
        Message::Rendered => {
            draw(&mdl.pet, &mdl.canvas, mdl.fill_color);
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
                let g_node = CGNode {
                    color: [255,255,255],
                    pos_x: (RAD + (i as u32 % (row_count as u32)) * (RAD * 2)).into(),
                    pos_y: (RAD + (i as u32 / (row_count as u32)) * (RAD * 2)).into(),
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
            ev(Ev::Click, |_| Message::CanvasClick)
        ],
        button!["Change color", ev(Ev::Click, |_| Message::ChangeColor)],
        button!["get .dot file", ev(Ev::Click, |_| Message::DotFile)],
        li![format!("{:?}", model)]
    ]
}
