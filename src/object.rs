#[derive(Debug, Eq, PartialEq)]
pub enum Object {
    Integer(i32),
    String(String),
    Null,
    Boolean(bool),
}