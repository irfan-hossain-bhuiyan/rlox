use std::str::FromStr;

use ascii::AsciiString;
use rlox::{basic_function::{to_string,repl}, parser::Parser, token::Scanner};
fn main() {
    repl(eval_expr);   
}
fn eval_expr(input: &str) {
    let Ok(input) = AsciiString::from_str(input) else {
        eprintln!("Can't convert to asciistring");
        return;
    };
    let token_tree = Scanner::new(&input).scan_tokens();
    let value = Parser::new(&token_tree).parse().evaluate();
    println!("Value is {}", to_string(value));
}
