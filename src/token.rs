#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token{
    Illegal,
    EOF,
    // Identifiers + Literals
    Ident(String),
    Int(i32),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

#[cfg(test)]
mod tests {

    use crate::lexer::Lexer;
    use crate::token::Token;
    #[test]
    fn next_token() {
        let input = "let five = 5;
        let ten = 10;
           let add = fn(x, y) {
             x + y;
        };
           let result = add(five, ten);";
        let tests = [
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::EOF,
        ];
        let mut lexer = Lexer::new(input);
        for token in tests {
            let lex_token = lexer.next_token();
            match lex_token {
                Token::Ident(ref x) => {
                    assert!(
                        token == lex_token,
                        "Expected token: {:?}, Got: {:?} with val {}",
                        &token,
                        &lex_token,
                        x
                    );
                }
                _ => {
                    assert!(
                        token == lex_token,
                        "Expected token: {:?}, Got: {:?}",
                        &token,
                        &lex_token
                    );
                }
            }
        }
    }
}
