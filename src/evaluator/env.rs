use std::collections::HashMap;
use crate::object::Object;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Env {
    pub(crate) store: HashMap<String, Object>,
    pub(crate) closure_store: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            closure_store: HashMap::new(),
        }
    }
    pub fn get(&mut self, key: &str) -> Object {
        match self.store.get(key) {
            None => Object::Null,
            Some(x) => x.clone()
        }
    }
    pub fn set(&mut self, key: &str, value: Object) {
        self.store.insert(key.to_string(), value);
    }
}