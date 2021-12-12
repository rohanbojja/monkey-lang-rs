#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Object {
    Integer(i32),
    String(String),
    Null,
    Boolean(bool),
    Return(Box<Object>),
}

pub struct ReturnValue {
    value: Object
}