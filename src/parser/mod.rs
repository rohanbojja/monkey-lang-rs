use crate::{
    ast::{self, Expression, Identifier, Statement, Sticky},
    lexer::Lexer,
    token::Token,
};

#[derive(Debug)]
pub struct Parser<'a> {
    l: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            l,
            current_token: Token::EOF,
            peek_token: Token::EOF,
            errors: vec![],
        };
        p.next_token();
        p.next_token();
        p
    }

    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        if let Some(expr) = self.parse_expression(Sticky::LOWEST) {
            let stmt = Statement::ExpressionStatement(expr);
            println!("Statement: {:?}", stmt);
            Some(stmt)
        } else {
            None
        }
    }

    fn exp_to_sticky(token: &Token) -> Sticky {
        match token {
            Token::LParen => Sticky::CALL,
            Token::Plus | Token::Minus => Sticky::SUM,
            Token::Asterisk | Token::Slash => Sticky::PRODUCT,
            Token::Lt | Token::Gt => Sticky::LESSGREATER,
            Token::Eq | Token::NotEq => Sticky::EQUALS,
            _ => Sticky::LOWEST,
        }
    }

    fn peek_stickiness(&self) -> Sticky {
        let peek_token = &self.peek_token;
        Self::exp_to_sticky(peek_token)
    }

    fn cur_stickiness(&self) -> Sticky {
        let cur_token = &self.current_token;
        Self::exp_to_sticky(cur_token)
    }

    fn parse_infix_expression(&mut self, left: ast::Expression) -> Option<ast::Expression> {
        let operator = self.current_token.clone();
        let stick = self.cur_stickiness();
        self.next_token();
        match self.parse_expression(stick) {
            Some(right) => Some(Expression::Infix(operator, Box::new(left), Box::new(right))),
            None => None,
        }
    }

    fn parse_bool_literal(&mut self) -> Option<ast::Expression> {
        if let Token::True = self.current_token {
            Some(ast::Expression::Boolean(true))
        } else {
            Some(ast::Expression::Boolean(false))
        }
    }

    fn parse_grouped_expression(&mut self) -> Option<ast::Expression> {
        self.next_token();
        let expr = self.parse_expression(Sticky::LOWEST);
        if Token::RParen.ne(&self.peek_token) {
            self.next_token();
            self.next_token();
            None
        } else {
            self.next_token();
            expr
        }
    }

    fn parse_if_expression(&mut self) -> Option<ast::Expression> {
        //"if (x < y) { x }"
        if self.expect_peek(Token::LParen) {
            if let Some(condition) = self.parse_expression(Sticky::LOWEST) {
                if self.expect_peek(Token::RParen) {
                    let consequence = self.parse_block_expression();
                    if self.peek_token == Token::Else {
                        self.expect_peek(Token::Else);
                        let alternative = self.parse_block_expression();
                        Some(Expression::If(Box::new(condition), consequence, alternative))
                    } else {
                        Some(Expression::If(Box::new(condition), consequence, None))
                    }
                } else {
                    None
                }
            } else {
                self.log_error("Invalid IF condition".to_string());
                None
            }
        } else {
            None
        }
    }

    fn parse_block_expression(&mut self) -> Option<ast::BlockStatement> {
        if self.current_token == Token::LBrace {
            self.next_token();
            let mut block = ast::BlockStatement { statements: vec![] };
            loop {
                match self.current_token {
                    Token::RBrace | Token::EOF => break,
                    _ => match self.parse_statement() {
                        Some(x) => block.statements.push(x),
                        None => {}
                    },
                }
                self.next_token();
            }
            Some(block)
        } else {
            None
        }
    }

    fn parse_function_params(&mut self) -> Vec<ast::Identifier> {
        let mut identifiers: Vec<ast::Identifier> = vec![];
        if self.peek_token != Token::RParen {
            loop {
                if let Token::Ident(ref x) = self.current_token {
                    let iden = ast::Identifier {
                        token: self.current_token.clone(),
                        value: x.to_string(),
                    };
                    identifiers.push(iden);
                    match self.peek_token {
                        Token::Comma => self.expect_peek(Token::Comma),
                        Token::RParen => break,
                        _ => {
                            self.log_error("Error parsing function parameters".to_string());
                            break;
                        }
                    };
                } else {
                    self.log_error("Error parsing function parameters".to_string());
                }
            }
        }
        identifiers
    }

    fn parse_function(&mut self) -> Option<ast::Expression> {
        //fn(x, y) { x + y; }
        if self.expect_peek(Token::LParen) {
            let params = self.parse_function_params();
            if self.expect_peek(Token::RParen) {
                let block = self.parse_block_expression();
                Some(Expression::Function(params, block))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_expression(&mut self, stick: Sticky) -> Option<ast::Expression> {
        let mut left = match self.current_token {
            Token::Function => self.parse_function(),
            Token::Ident(_) => self.parse_identifier(),
            Token::If => self.parse_if_expression(),
            Token::Int(..) => {
                let left_exp = self.parse_integer_literal();
                left_exp
            }
            Token::Bang | Token::Minus => {
                let operator = self.current_token.clone();
                self.next_token();
                let right_exp = self.parse_expression(Sticky::PREFIX);
                if let Some(exp) = right_exp {
                    let prefix_exp = Expression::Prefix(operator, Box::new(exp));
                    Some(prefix_exp)
                } else {
                    None
                }
            }
            Token::True | Token::False => {
                let left_exp = self.parse_bool_literal();
                left_exp
            }
            Token::LParen => {
                let left_exp = self.parse_grouped_expression();
                left_exp
            }
            _ => {
                self.log_error("Unknown expression".to_string());
                None
            }
        };

        /*
        infix stuff
         */
        while self.peek_token != Token::Semicolon && stick < self.peek_stickiness() {
            match self.peek_token {
                Token::LParen => {
                    self.next_token();
                    left = self.parse_call_arguments(left.unwrap());
                }
                Token::Asterisk
                | Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Eq
                | Token::NotEq
                | Token::Gt
                | Token::Lt => {
                    self.next_token();
                    left = self.parse_infix_expression(left.unwrap());
                }
                _ => left = left,
            }
        }
        left
    }

    fn parse_call_arguments(&mut self, identifier: ast::Expression) -> Option<ast::Expression> {
        let mut arguments: Vec<ast::Expression> = vec![];
        if self.peek_token != Token::RParen {
            self.next_token();
            loop {
                if let Some(expr) = self.parse_expression(Sticky::LOWEST) {
                    arguments.push(expr);
                } else {
                    self.log_error("Error parsing function arguments".to_string());
                }
                match self.peek_token {
                    Token::Comma => self.expect_peek(Token::Comma),
                    Token::RParen => break,
                    _ => {
                        self.log_error("Error parsing function parameters".to_string());
                        break;
                    }
                };
            }
        }
        self.expect_peek(Token::RParen);
        Some(ast::Expression::Call(Box::new(identifier), arguments))
    }

    fn parse_identifier(&self) -> Option<ast::Expression> {
        if let Token::Ident(ref x) = self.current_token {
            return Some(ast::Expression::Ident(x.to_string()));
        } else {
            None
        }
    }

    fn parse_integer_literal(&mut self) -> Option<ast::Expression> {
        if let Token::Int(x) = self.current_token {
            Some(ast::Expression::Integer(x))
        } else {
            None
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.to_owned();
        self.peek_token = self.l.next_token();
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token {
            Token::Illegal => None,
            Token::Semicolon => None,
            Token::Let => {
                self.next_token();
                let var = &self.current_token;
                if let Token::Ident(ref val) = var {
                    let iden = Identifier {
                        token: var.clone(),
                        value: val.to_string(),
                    };
                    self.expect_peek(Token::Assign);
                    if let Some(expr) = self.parse_expression(Sticky::LOWEST) {
                        Some(Statement::LetStatement(iden, expr))
                    } else {
                        None
                    }
                } else {
                    self.log_error(Parser::<'a>::peek_error(
                        &Token::Ident("IDEN".to_string()),
                        &self.peek_token,
                    ));
                    None
                }
            }
            Token::Return => {
                self.next_token();
                if let Some(expr) = self.parse_expression(Sticky::LOWEST) {
                    loop {
                        if self.current_token != Token::Semicolon {
                            self.next_token();
                        } else {
                            break;
                        }
                    }
                    Some(Statement::ReturnStatement(expr))
                } else { None }
            }
            _ => {
                // //parse expr
                self.parse_expression_statement()
            }
        }
    }

    pub fn log_error(&mut self, err: String) {
        self.errors.push(err);
    }

    fn expect_peek(&mut self, expected_token: Token) -> bool {
        if self.peek_token == expected_token {
            self.next_token();
            self.next_token();
            true
        } else {
            self.log_error(Self::peek_error(&expected_token, &self.peek_token));
            false
        }
    }

    pub fn peek_error(expected: &Token, received: &Token) -> String {
        format!("Expected {:?}, Got {:?}", expected, received).to_string()
    }
    pub fn parse_program(&mut self) -> Result<ast::Program, Vec<String>> {
        let mut program = ast::Program { statements: vec![] };

        loop {
            match self.current_token {
                Token::EOF => break,
                _ => match self.parse_statement() {
                    Some(x) => program.statements.push(x),
                    None => {}
                },
            }
            self.next_token();
        }
        if self.errors.len() > 0 {
            Err(self.errors.clone())
        } else {
            Ok(program)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Statement, lexer};

    use super::Parser;

    #[test]
    fn test_let_statement() {
        let input = "
        let x = 5;
        let y=10;
        let foobar = 6969;";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(program.statements.len() == 3)
    }

    #[test]
    fn test_let_errors() {
        let input = "
        let x 5;
   let = 10;
   let 838383;";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        match program {
            Ok(_) => panic!("Catching errors failed!"),
            Err(x) => {
                for s in x {
                    println!("{:?}", s)
                }
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "
        return 5;
        return 10;
        return 696969;";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(program.statements.len() == 3)
    }

    #[test]
    fn test_iden_expr() {
        let input = "foobar;";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(
            matches!(program.statements[0], Statement::ExpressionStatement(..)),
            "Not an expression statement!"
        )
    }

    #[test]
    fn test_int_expr() {
        let input = "5;";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(
            matches!(program.statements[0], Statement::ExpressionStatement(..)),
            "Not an expression statement!"
        )
    }

    #[test]
    fn test_parsing_prefix_expression() {
        let input = "!5; -5;";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(
            matches!(program.statements[0], Statement::ExpressionStatement(..)),
            "Not an expression statement!"
        )
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let input = "a + b + c";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(
            matches!(program.statements[0], Statement::ExpressionStatement(..)),
            "Not an expression statement!"
        )
    }

    #[test]
    fn test_bool_expr() {
        let input = "false;";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(
            matches!(program.statements[0], Statement::ExpressionStatement(..)),
            "Not an expression statement!"
        )
    }

    #[test]
    fn test_operator_precedence() {
        let tests = vec![
            ("1 + (2 + 3) + 4", "((1+ (2+3)) + 4 )"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];
        for test in tests {
            let l1 = lexer::Lexer::new(test.0);
            let l2 = lexer::Lexer::new(test.1);
            let mut p1 = Parser::new(l1);
            let mut p2 = Parser::new(l2);
            let program1 = p1.parse_program().unwrap();
            let program2 = p2.parse_program().unwrap();
            for i in 0..program1.statements.len() {
                assert!(program1.statements[i] == program2.statements[i])
            }
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(
            matches!(program.statements[0], Statement::ExpressionStatement(..)),
            "Not an expression statement!"
        )
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(
            matches!(program.statements[0], Statement::ExpressionStatement(..)),
            "Not an expression statement!"
        )
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y; }";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(
            matches!(program.statements[0], Statement::ExpressionStatement(..)),
            "Not an expression statement!"
        )
    }

    #[test]
    fn test_call_expression_parsing() {
        let input = "add(1, 2 * 3, 4 + 5);";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        for s in program.statements.iter() {
            println!("{:?}", &s)
        }
        assert!(
            matches!(program.statements[0], Statement::ExpressionStatement(..)),
            "Not an expression statement!"
        )
    }
}
