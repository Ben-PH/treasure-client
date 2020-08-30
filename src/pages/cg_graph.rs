use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
use seed::{prelude::*, *};
use shared::learning_trajectory;
use std::collections::HashMap;
use web_sys::HtmlCanvasElement;

const WIDTH: usize = 450;
const HEIGHT: usize = 300;

#[derive(Default, Debug)]
pub struct Model {
    pub pet: DiGraph<learning_trajectory::ConsensusGoal, f32>,
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
fn draw(
    model: &petgraph::graph::DiGraph<shared::learning_trajectory::ConsensusGoal, f32>,
    canvas: &ElRef<HtmlCanvasElement>,
    fill_color: Color,
) {
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    ctx.rect(0., 0., (WIDTH as u32).into(), (HEIGHT as u32).into());
    ctx.set_fill_style(&JsValue::from_str(fill_color.as_str()));
    ctx.fill();

    let radius: u32 = 10;
    let diam: u32 = radius * 2;
    let row_count: u32 = (WIDTH as u32) / diam;

    let len = model.node_count();
    for (i, node) in model.node_references().enumerate() {
        let x: f64 = (radius + (i as u32 % (row_count as u32)) * diam).into();
        let y: f64 = (radius + (i as u32 / (row_count as u32)) * diam).into();
        ctx.begin_path();
        ctx.arc(x, y, radius.into(), 0.0, std::f64::consts::PI * 2.);
        ctx.set_fill_style(&JsValue::from_str("white"));
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
            let mut gr = DiGraph::<learning_trajectory::ConsensusGoal, f32>::new();
            let mut idx_map: HashMap<usize, NodeIndex> = HashMap::with_capacity(res.0.len());
            for node in res.0.into_iter() {
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
        ],
        button!["Change color", ev(Ev::Click, |_| Message::ChangeColor)],
        button!["get .dot file", ev(Ev::Click, |_| Message::DotFile)],
        li![format!("{:?}", model)]
    ]
}
