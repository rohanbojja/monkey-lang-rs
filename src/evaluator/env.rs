use std::collections::HashMap;
use crate::object::Object;

pub struct Env {
    pub(crate) store: HashMap<String, Object>
}

impl Env {
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