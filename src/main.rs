extern crate monkey_lang_rs;

use std::{io::Write, str::FromStr};
use monkey_lang_rs::evaluator::{self};

use monkey_lang_rs::{lexer, parser};

fn main() {
    let mut evaluator = evaluator::Evaluator::new();
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input == String::from_str("exit").unwrap() {
            break;
        }
        let mut lexer = lexer::Lexer::new(&input);
        let mut p = parser::Parser::new(lexer);
        let program = p.parse_program().unwrap();
        evaluator.eval_statements(&program.statements);
        std::io::stdout().flush();
    }
}
