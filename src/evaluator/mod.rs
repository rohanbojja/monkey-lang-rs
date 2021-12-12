mod env;

use crate::ast::{self, Statement, Expression};
use crate::object::{self, Object};
use crate::token::Token;
use ast::BlockStatement;
use std::collections::HashMap;

pub struct Evaluator {
    env: env::Env
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            env: env::Env {
                store: HashMap::new()
            }
        }
    }

    fn is_truthy(obj: &Object) -> bool {
        match obj {
            Object::Null | Object::Boolean(false) => false,
            _ => true
        }
    }

    fn eval_int_infix(op: &Token, a: i32, b: i32) -> Object {
        match op {
            Token::Plus => { Object::Integer(a + b) }
            Token::Minus => { Object::Integer(a - b) }
            Token::Asterisk => { Object::Integer(a * b) }
            Token::Slash => { Object::Integer(a / b) }
            Token::Lt => { Object::Boolean(a < b) }
            Token::Gt => { Object::Boolean(a > b) }
            Token::Eq => { Object::Boolean(a == b) }
            Token::NotEq => { Object::Boolean(a != b) }
            _ => Object::Null
        }
    }

    fn eval_block_statements(&mut self, block: &Vec<Statement>) -> Object {
        let mut res = Object::Null;
        for s in block {
            res = self.eval_statement(&s);
            match &res {
                Object::Return(x) => return res,
                _ => {}
            }
        }
        res
    }
    fn eval_expr(&mut self, expr: &ast::Expression) -> object::Object {
        match expr {
            Expression::Ident(s) => {
                self.env.get(s)
            }
            Expression::If(condition, consequence, alternative) => {
                let c = self.eval_expr(condition);
                println!("Condition: {:?}", c);
                if Evaluator::is_truthy(&c) {
                    if let Some(block) = consequence {
                        self.eval_block_statements(&block.statements)
                    } else {
                        Object::Null
                    }
                } else {
                    if let Some(block) = alternative {
                        self.eval_block_statements(&block.statements)
                    } else {
                        Object::Null
                    }
                }
            }
            Expression::Integer(x) => Object::Integer(*x),
            Expression::Boolean(x) => Object::Boolean(*x),
            Expression::Prefix(tok, exp) => {
                let right = self.eval_expr(exp);
                match tok {
                    Token::Minus => {
                        match right {
                            Object::Integer(x) => { Object::Integer(-x) }
                            _ => Object::Null
                        }
                    }
                    Token::Bang => {
                        match right {
                            Object::Boolean(x) => { Object::Boolean(!x) }
                            _ => Object::Null
                        }
                    }
                    _ => Object::Null
                }
            }
            Expression::Infix(op, left, right) => {
                let left_o = self.eval_expr(left);
                let right_o = self.eval_expr(right);
                match (left_o, right_o) {
                    (Object::Integer(a), Object::Integer(b)) => {
                        Evaluator::eval_int_infix(op, a, b)
                    }
                    (Object::Boolean(a), Object::Boolean(b)) => {
                        match op {
                            Token::Eq => { Object::Boolean(a == b) }
                            Token::NotEq => { Object::Boolean(a != b) }
                            _ => Object::Null
                        }
                    }
                    _ => Object::Null
                }
            }
            _ => Object::Null
        }
    }

    fn eval_statement(&mut self, expr: &ast::Statement) -> object::Object {
        match expr {
            Statement::LetStatement(x, y) => {
                let value = self.eval_expr(y);
                self.env.set(&x.value, value);
                Object::Null
            }
            Statement::ReturnStatement(val) => {
                Object::Return(Box::new(self.eval_expr(val)))
            }
            Statement::ExpressionStatement(expr) => {
                self.eval_expr(expr)
            }
        }
    }

    pub fn eval_statements(&mut self, statements: &Vec<Statement>) -> Object {
        let mut res = Object::Null;
        for s in statements {
            res = self.eval_statement(&s);
            match res {
                Object::Return(x) => {
                    return *x;
                }
                _ => {}
            }
            println!("Output: {:?}", res);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::parser::Parser;
    use crate::object::Object;
    use crate::evaluator::{self};

    /*

           {"let a = 5; a;", 5},
           {"let a = 5 * 5; a;", 25},
           {"let a = 5; let b = a; b;", 5},
           {"let a = 5; let b = a; let c = a + b + 5; c;", 15},

     */
    #[test]
    fn test_return_value() {
        let tests = vec![
            ("return 10;", Object::Integer(10)),
            ("return 10; 9;", Object::Integer(10)),
            ("return 2 * 5; 9;", Object::Integer(10)),
            ("9; return 2 * 5; 9;", Object::Integer(10)),
            ("if (10 > 1) {
     if (10 > 1) {
       return 10;
} return 1; }", Object::Integer(10))
        ];
        for test in tests.iter() {
            let l1 = lexer::Lexer::new(test.0);
            let mut p1 = Parser::new(l1);
            let program1 = p1.parse_program().unwrap();
            let mut evaluator = evaluator::Evaluator::new();
            assert_eq!(evaluator.eval_statements(&program1.statements), test.1);
        }
    }

    #[test]
    fn test_if_else_expr() {
        let tests = vec![
            ("if (true) { 10 }", Object::Integer(10)),
            ("if (false) { 10 }", Object::Null),
            ("if (1) { 10 }", Object::Integer(10)),
            ("if (1 < 2) { 10 }", Object::Integer(10)),
            ("if (1 > 2) { 10 }", Object::Null),
            ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
            ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
        ];
        for test in tests.iter() {
            let l1 = lexer::Lexer::new(test.0);
            let mut p1 = Parser::new(l1);
            let program1 = p1.parse_program().unwrap();
            let mut evaluator = evaluator::Evaluator::new();
            assert_eq!(evaluator.eval_statements(&program1.statements), test.1)
        }
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![("true", "!false"), ("false", "!true")];
        for test in tests {
            let l1 = lexer::Lexer::new(test.0);
            let l2 = lexer::Lexer::new(test.1);
            let mut p1 = Parser::new(l1);
            let mut p2 = Parser::new(l2);
            let program1 = p1.parse_program().unwrap();
            let program2 = p2.parse_program().unwrap();
            let mut evaluator1 = evaluator::Evaluator::new();
            let mut evaluator2 = evaluator::Evaluator::new();
            assert_eq!(evaluator1.eval_statements(&program1.statements), evaluator2.eval_statements(&program2.statements))
        }
    }

    #[test]
    fn test_negation_operator() {
        let tests = vec![("-5", Object::Integer(-5)), ("-90", Object::Integer(-90))];
        for test in tests.iter() {
            let l1 = lexer::Lexer::new(test.0);
            let mut p1 = Parser::new(l1);
            let program1 = p1.parse_program().unwrap();
            let mut evaluator = evaluator::Evaluator::new();
            for i in 0..program1.statements.len() {
                assert_eq!(evaluator.eval_statements(&program1.statements), test.1)
            }
        }
    }

    #[test]
    fn test_infix_expr() {
        let tests = vec![
            ("6-5", Object::Integer(1)),
            ("100-(45+45)", Object::Integer(10)),
            ("50 / 2 * 2 + 10", Object::Integer(60)),
            ("2 * (5 + 10)", Object::Integer(30))
        ];
        for test in tests.iter() {
            let l1 = lexer::Lexer::new(test.0);
            let mut p1 = Parser::new(l1);
            let program1 = p1.parse_program().unwrap();
            let mut evaluator = evaluator::Evaluator::new();
            assert_eq!(evaluator.eval_statements(&program1.statements), test.1)
        }
    }

    #[test]
    fn test_infix_bool_expr() {
        let tests = vec![
            ("true", Object::Boolean(true)),
            ("false", Object::Boolean(false)),
            ("1 < 2", Object::Boolean(true)),
            ("1 > 2", Object::Boolean(false)),
            ("1 < 1", Object::Boolean(false)),
            ("1 > 1", Object::Boolean(false)),
            ("1 == 1", Object::Boolean(true)),
            ("1 != 1", Object::Boolean(false)),
            ("1 == 2", Object::Boolean(false)),
            ("1 != 2", Object::Boolean(true)),
            ("true == true", Object::Boolean(true)),
            ("false == false", Object::Boolean(true)),
            ("true == false", Object::Boolean(false)),
            ("true != false", Object::Boolean(true)),
            ("false != true", Object::Boolean(true)),
            ("(1 < 2) == true", Object::Boolean(true)),
            ("(1 < 2) == false", Object::Boolean(false)),
            ("(1 > 2) == true", Object::Boolean(false)),
            ("(1 > 2) == false", Object::Boolean(true)),
        ];
        for test in tests.iter() {
            let l1 = lexer::Lexer::new(test.0);
            let mut p1 = Parser::new(l1);
            let program1 = p1.parse_program().unwrap();
            let mut evaluator = evaluator::Evaluator::new();
            assert_eq!(evaluator.eval_statements(&program1.statements), test.1)
        }
    }
}
