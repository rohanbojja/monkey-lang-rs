use crate::evaluator::env::Env;
use crate::ast::{Identifier, BlockStatement};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Object {
    Integer(i32),
    String(String),
    Null,
    Boolean(bool),
    Return(Box<Object>),
    Function(Vec<Identifier>, BlockStatement, Env),
}

pub struct ReturnValue {
    value: Object
}