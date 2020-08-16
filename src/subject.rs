use std::hash::Hasher;
use std::rc::Rc;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
pub struct Subject {
    name: String,
    field: Field,
    topics: HashSet<Topic>,
}

#[derive(Debug)]
pub struct Topic {
    name: String,
    pre_req_to: HashSet<Rc<Topic>>,
    supported_by: HashSet<Rc<Topic>>
}

impl Subject {
    pub fn init(name: String, field: Field) -> Self {
        Self{name, field, topics: std::collections::HashSet::with_capacity(3)}
    }
}

impl PartialEq for Subject {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Subject {}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Field {
    ComputerScience
}
