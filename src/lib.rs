extern crate core;

use wasm_bindgen::prelude::*;
use crate::evaluator::Evaluator;

pub mod lexer;
pub mod token;
pub mod ast;
pub mod parser;
pub(crate) mod object;
pub mod evaluator;

#[wasm_bindgen]
pub fn eval_monkey_program(input: String) -> String {
    let mut evaluator = evaluator::Evaluator::new();
    let mut lexer = lexer::Lexer::new(&input);
    let mut p = parser::Parser::new(lexer);
    let program = p.parse_program().unwrap();
    let final_object = evaluator.eval_statements(&program.statements);
    Evaluator::unwrap_object(final_object)
}
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[cfg(test)]
mod tests {
    use crate::eval_monkey_program;
    use std::str::FromStr;

    #[test]
    fn test_fibonacci(){
        let input = "let fibonacci = fn(x) {
                              if (x == 0) {
                                0
                              } else {
                                if (x == 1) {
                                  return 1;
                                } else {
                                  fibonacci(x - 1) + fibonacci(x - 2);
                                }
                              }
                            };
                            fibonacci(20)";
        let ans = i32::from_str(&eval_monkey_program(input.to_string())).unwrap();
        assert_eq!(ans, 6765);
    }
}