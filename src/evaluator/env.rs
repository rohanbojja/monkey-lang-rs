use std::collections::HashMap;
use crate::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Env {
    pub(crate) store: HashMap<String, Object>,
    pub(crate) outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_closure(env_func: &Env, env: &Env) -> Self {
        Env {
            store: env_func.store.clone(),
            outer: Some(Rc::new(RefCell::new(env.clone()))),
        }
    }
    pub fn get(&mut self, key: &str) -> Object {
        match self.store.get(key) {
            None => {
                if let Some(global) = &self.outer {
                    global.borrow_mut().get(key)
                } else {
                    Object::Null
                }
            }
            Some(x) => x.clone()
        }
    }
    pub fn set(&mut self, key: &str, value: Object) {
        self.store.insert(key.to_string(), value);
    }
}