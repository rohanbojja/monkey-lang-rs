extern crate monkey_lang_rs;

use std::{io::Write, str::FromStr};

use monkey_lang_rs::lexer;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = lexer::Lexer::new(&input);
        if input == String::from_str("exit").unwrap() {
            break;
        }
        while let tok = lexer.next_token() {
            match tok {
                monkey_lang_rs::token::Token::EOF => break,
                m => println!("{:?}", m)
            }
        }
        std::io::stdout().flush();
    }
}
