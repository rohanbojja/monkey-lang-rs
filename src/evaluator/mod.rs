use crate::ast::{self, Statement, Expression};
use crate::object::{self, Object};
use crate::token::Token;

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

fn eval_expr(expr: &ast::Expression) -> object::Object {
    match expr {
        Expression::Integer(x) => Object::Integer(*x),
        Expression::Boolean(x) => Object::Boolean(*x),
        Expression::Prefix(tok, exp) => {
            let right = eval_expr(exp);
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
            let left_o = eval_expr(left);
            let right_o = eval_expr(right);
            match (left_o, right_o) {
                (Object::Integer(a), Object::Integer(b)) => {
                    eval_int_infix(op, a, b)
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

fn eval_statement(expr: &ast::Statement) -> object::Object {
    match expr {
        Statement::LetStatement(_, _) => Object::Null,
        Statement::ReturnStatement(_) => Object::Null,
        Statement::ExpressionStatement(expr) => {
            eval_expr(expr)
        }
    }
}

pub fn eval_statements(statements: Vec<Statement>) -> Object {
    let mut res = Object::Null;
    for s in statements {
        res = eval_statement(&s);
        println!("OBJECT: {:?}", res);
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::parser::Parser;
    use crate::evaluator::{eval_statement};
    use crate::object::Object;

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
            for i in 0..program1.statements.len() {
                assert_eq!(eval_statement(&program1.statements[i]), eval_statement(&program2.statements[i]))
            }
        }
    }

    #[test]
    fn test_negation_operator() {
        let tests = vec![("-5", Object::Integer(-5)), ("-90", Object::Integer(-90))];
        for test in tests.iter() {
            let l1 = lexer::Lexer::new(test.0);
            let mut p1 = Parser::new(l1);
            let program1 = p1.parse_program().unwrap();
            for i in 0..program1.statements.len() {
                assert_eq!(eval_statement(&program1.statements[i]), test.1)
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
            for i in 0..program1.statements.len() {
                assert_eq!(eval_statement(&program1.statements[i]), test.1)
            }
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
            for i in 0..program1.statements.len() {
                assert_eq!(eval_statement(&program1.statements[i]), test.1)
            }
        }
    }
}
