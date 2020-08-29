use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use seed::{prelude::*, *};
use shared::learning_trajectory;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Model {
    pub pet: DiGraph<learning_trajectory::ConsensusGoal, f32>,
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
}

pub fn update(msg: Message, mdl: &mut Model, orders: &mut impl Orders<Message>) {
    use Message::*;
    match msg {
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
            log!("{:?}", Dot::with_config(&mdl.pet, &[Config::EdgeNoLabel]));
        }
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
    log!("got this cg_graph: {:?}", result);
    Ok(result)
}

pub fn view(model: &Model) -> Node<Message> {
    ul![
        li![button![
            "get cg_graph",
            ev(Ev::Click, |_| Message::FetchCGGraph)
        ]],
        li![format!("{:?}", model)]
    ]
}
