use seed::{prelude::*, *};
use shared::SubjectId;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Model {
    pub subjects: shared::SubjectCollection,
}

#[derive(Debug)]
pub enum Message {
    FetchSubjects,
    FetchTopics(shared::SubjectId),
    Topics(fetch::Result<(shared::SubjectId, shared::TopicCollection)>),
    Subjects(fetch::Result<HashMap<shared::SubjectId, shared::Subject>>),
}

pub fn update(msg: Message, mdl: &mut Model, orders: &mut impl Orders<Message>) {
    use Message::*;
    match msg {
        FetchSubjects => {
            orders.perform_cmd(async { Subjects(fetch_subjects().await) });
        }
        FetchTopics(id) => {
            orders.perform_cmd(async move { Topics(fetch_topics(id).await) });
        }
        Subjects(Ok(res)) => {
            mdl.subjects = res;
            log!(mdl)
        }
        Topics(Ok((id, res))) => {
            mdl.subjects.get_mut(&id).unwrap().topics = res;
        }
        _ => log!("impl me", msg),
    }
}

async fn fetch_topics(
    subject_id: shared::SubjectId,
) -> fetch::Result<(shared::SubjectId, shared::TopicCollection)> {
    let result = Request::new(format!("api/graph/topics/{}", subject_id))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await?;
    log!("got this hash-map: {:?}", result);
    Ok((subject_id, result))
}
async fn fetch_subjects() -> fetch::Result<HashMap<SubjectId, shared::Subject>> {
    let result = Request::new("api/graph/subjects")
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await?;
    log!("got this hash-map: {:?}", result);
    Ok(result)
}

pub fn view(model: &Model) -> Node<Message> {
    let nodes = model
        .subjects
        .iter()
        .map(|(id, sub)| li_sub(*id, &sub.name));
    ul![
        li![button![
            "get subjects",
            ev(Ev::Click, |_| Message::FetchSubjects)
        ]],
        nodes,
        li![format!("{:?}", model)]
    ]
}

fn li_sub(id: shared::SubjectId, st: &String) -> Node<Message> {
    let subject_name = st.clone();
    li![
        subject_name,
        ev(Ev::Click, move |_| Message::FetchTopics(id))
    ]
}
