use seed::{prelude::*, *};


struct Model;

fn init(url: Url, orders: &mut impl Orders<Message>) -> Model {
    Model
}

enum Message {
    Nothing
}

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {}

fn view(mdl: &Model) -> impl IntoNodes<Message> {
    empty![]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
