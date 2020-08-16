use std::collections::HashSet;
use seed::{prelude::*, *};
#[derive(Default, Debug)]
pub struct Model {
    inners: HashSet<crate::subject::Subject>,
}


#[derive(Debug)]
pub enum Message {
    Nothing
}

pub fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
}
