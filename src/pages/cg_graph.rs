use seed::{prelude::*, *};
use shared::learning_trajectory;


#[derive(Default, Debug)]
pub struct Model {
    pub graph: learning_trajectory::CGGraph,
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
            mdl.graph = res;
            log!(mdl);
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

