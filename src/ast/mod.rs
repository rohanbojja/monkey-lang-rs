use crate::token::Token;

#[derive(Debug,PartialEq, Eq, Clone)]
pub enum Statement{
    LetStatement(Identifier, Expression),
    ReturnStatement(Expression),
    ExpressionStatement(Expression)
}
#[derive(Debug,PartialEq, Eq, Clone)]
pub struct BlockStatement{
    pub statements: Vec<Statement>
}

#[derive(Debug,PartialEq, Eq, Clone)]
pub enum Expression{
    EMPTY,
    Ident(String),
    Integer(i32),
    Prefix(Token, Box<Expression>), // Operator, RightExpression
    Infix(Token, Box<Expression>, Box<Expression>),//Op,left,right
    Boolean(bool),
    Null,
    If(Box<Expression>, Option<BlockStatement>, Option<BlockStatement>), // (Condition, Consequence, Alternative)
    Function(Vec<Identifier>, Option<BlockStatement>),
    Call(Box<Expression>, Vec<Expression>) // Identifier or function literal
}

#[derive(Debug,PartialEq, Eq, Clone)]
pub struct Identifier{
    pub token: Token,
    pub value: String,
}
pub struct Program{
    pub statements: Vec<Statement>
}

#[derive(PartialEq, PartialOrd,Debug)]
pub enum Sticky {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL
}
