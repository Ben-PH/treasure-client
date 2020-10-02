#[macro_use] extern crate serde;
#[macro_use] extern crate specs;

use seed::{prelude::*, *};

mod pages;
mod subject;
// mod ametheed;
mod components;
mod systems;

fn init(mut _url: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");

    orders
        .perform_cmd(async {
            match Request::new("/api/auth").method(Method::Get).fetch().await {
                Ok(fetch) => match fetch.check_status() {
                    Ok(good_resp) => Message::LoginMsg(pages::login::Message::GoodLogin(
                        good_resp.json().await.unwrap(),
                    )),
                    Err(_) => Message::LoginMsg(pages::login::Message::Unauth),
                },
                Err(e) => Message::NetworkError(e),
            }
        });
    Model::default()
}
impl Model {
    fn default() -> Self {
        Self {
            login: Some(pages::login::Model::default()),
            subjects: pages::cg_graph::Model::default(),
        }
    }
}

#[derive(Default)]
struct Model {

    login: Option<pages::login::Model>,
    subjects: pages::cg_graph::Model,
}

#[derive(Debug)]
pub enum Message {
    GoodLogin(shared::User),
    LoginMsg(pages::login::Message),
    CGGraphMessage(pages::cg_graph::Message),
    NetworkError(fetch::FetchError),
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    // log("updating");

    use Message::*;
    match msg {
        GoodLogin(_usr) => {
            model.login = None;
            log!("good login");
        }
        LoginMsg(msg) => {
            if let Some(GoodLogin(usr)) = pages::login::update(
                msg,
                model.login.as_mut().unwrap(),
                &mut orders.proxy(LoginMsg),
            ) {
                orders.perform_cmd(async move { GoodLogin(usr) });
            }
        }
        CGGraphMessage(msg) => {
            pages::cg_graph::update(msg, &mut model.subjects, &mut orders.proxy(CGGraphMessage))
        }
        _ => log!("impl me: ", msg),
    }
}

// ------ ------
//     View
// ------ ------

fn view(mdl: &Model) -> Vec<Node<Message>> {
    let main_view = match &mdl.login {
        Some(login) => nodes![pages::login::view(&login)].map_msg(Message::LoginMsg),
        None => nodes![div!["foobar"]],
    };
    nodes![
        main_view,
        pages::cg_graph::view(&mdl.subjects).map_msg(Message::CGGraphMessage),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
