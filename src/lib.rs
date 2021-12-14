extern crate core;
extern crate proc_macro;

pub mod lexer;
pub mod token;
pub mod ast;
pub mod parser;
pub(crate) mod object;
pub mod evaluator;

#[cfg(test)]
mod tests {
    
}