use seed::{prelude::*, *};
use shared::{
    LearningObjCollection, Subject, SubjectCollection, SubjectId, Topic, TopicCollection, TopicId,
};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Model {
    pub subjects: SubjectCollection,
}

#[derive(Debug)]
pub enum Message {
    FetchSubjects,
    FetchTopics(SubjectId),
    FetchTasks(TopicId),
    Topics(fetch::Result<(SubjectId, TopicCollection)>),
    Tasks(fetch::Result<(TopicId, LearningObjCollection)>),
    Subjects(fetch::Result<HashMap<SubjectId, Subject>>),
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
        FetchTasks(id) => {
            orders.perform_cmd(async move { Tasks(fetch_tasks(id).await) });
        }
        Subjects(Ok(res)) => {
            mdl.subjects = res;
            log!(mdl)
        }
        Topics(Ok((id, res))) => {
            mdl.subjects.get_mut(&id).unwrap().topics = res;
        }
        // Task(Ok((id, res))) => {
        //     mdl.subjects.get_mut(&id).unwrap().learning_objectives = res;
        // }
        _ => log!("impl me", msg),
    }
}

async fn fetch_tasks(topic_id: TopicId) -> fetch::Result<(SubjectId, LearningObjCollection)> {
    log!("fetching tasks for topic id", topic_id);
    let result = Request::new(format!("api/graph/tasks/{}", topic_id))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await?;
    log!("got this hash-map: {:?}", result);
    Ok((topic_id, result))
}

async fn fetch_topics(subject_id: SubjectId) -> fetch::Result<(SubjectId, TopicCollection)> {
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
async fn fetch_subjects() -> fetch::Result<HashMap<SubjectId, Subject>> {
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
        .map(|(_id, sub)| li_sub(sub, &sub.name));
    ul![
        li![button![
            "get subjects",
            ev(Ev::Click, |_| Message::FetchSubjects)
        ]],
        nodes,
        li![format!("{:?}", model)]
    ]
}

fn li_sub(subject: &Subject, st: &str) -> Node<Message> {
    let subject_name = st.clone();
    let id = subject.id;
    let topic_nodes = subject
        .topics
        .iter()
        .map(|(id, topic)| li_topic(*id, &topic));
    li![
        subject_name,
        ev(Ev::Click, move |_| Message::FetchTopics(id)),
        ul![topic_nodes]
    ]
}

fn li_topic(id: TopicId, topic: &Topic) -> Node<Message> {
    let topic_name = topic.name.clone();
    let tasks = topic.learning_objectives.iter().map(|(_id, task)| {
        let name = task.name.clone();
        let inst = task.instructions.clone();
        nodes![li![name], li![inst]]
    });
    li![
        topic_name,
        ev(Ev::Click, move |_| Message::FetchTasks(id)),
        ul![tasks]
    ]
}
