use crate::token::Token;
#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    current_char: u8,
}

impl<'a> Lexer<'a> {
    fn is_letter(&self) -> bool {
        let letter = self.current_char;
        (letter >= b'a' && letter <= b'z') || (letter >= b'A' && letter <= b'Z') || letter == b'_'
    }

    fn read_identifier(&mut self) -> Token {
        let initial_position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        let identifier = &self.input[initial_position..self.position];
        let kw = identifier;
        match kw {
            "let" => Token::Let,
            "fn" => Token::Function,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(identifier.to_string()),
        }
    }

    fn read_digit(&mut self) -> Token {
        let initial_position = self.position;
        while self.is_digit() {
            self.read_char();
        }
        let num = &self.input[initial_position..self.position];
        Token::Int(
            num.parse::<i32>()
                .expect("Invalid identifier or number!"),
        )
    }

    fn skip_whitespaces(&mut self) {
        while self.current_char == b' '
            || self.current_char == b'\t'
            || self.current_char == b'\n'
            || self.current_char == b'\r'
        {
            self.read_char();
        }
    }

    fn is_digit(&self) -> bool {
        let letter = self.current_char;
        letter >= b'0' && letter <= b'9'
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();
        match self.current_char {
            // Symbols
            0 => Token::EOF,
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Eq
                } else {
                    self.read_char();
                    Token::Assign
                }
            }
            b';' => {
                self.read_char();
                Token::Semicolon
            }
            b'(' => {
                self.read_char();
                Token::LParen
            }
            b')' => {
                self.read_char();
                Token::RParen
            }
            b',' => {
                self.read_char();
                Token::Comma
            }
            b'+' => {
                self.read_char();
                Token::Plus
            }
            b'{' => {
                self.read_char();
                Token::LBrace
            }
            b'}' => {
                self.read_char();
                Token::RBrace
            }
            b'-' => {
                self.read_char();
                Token::Minus
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::NotEq
                } else {
                    self.read_char();
                    Token::Bang
                }
            }
            b'*' => {
                self.read_char();
                Token::Asterisk
            }
            b'/' => {
                self.read_char();
                Token::Slash
            }
            b'<' => {
                self.read_char();
                Token::Lt
            }
            b'>' => {
                self.read_char();
                Token::Gt
            }
            _ => {
                //isLetter
                if self.is_letter() {
                    self.read_identifier()
                } else if self.is_digit() {
                    self.read_digit()
                } else {
                    Token::Illegal
                }
            }
        }
    }

    fn peek_char(&self) -> u8 {
        let l = self.input.len();
        if self.read_position >= l {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    fn read_char(&mut self) {
        let l = self.input.len();
        if self.read_position >= l {
            self.current_char = 0;
        } else {
            self.current_char = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            current_char: 0,
        };
        l.read_char();
        l
    }
}
