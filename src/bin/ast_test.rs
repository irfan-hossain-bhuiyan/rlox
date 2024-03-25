use std::str::FromStr;

use rlox::{interpreter::Interpreter, parser::Parser, token::Scanner, *};
use ascii::AsciiString;
fn main(){
    let mut interpreter=Interpreter::default();
    let a=AsciiString::from_str("var a=10;print(a);print(10+20);").unwrap();
    let tokens=Scanner::new(&a).scan_tokens();
    let parser=Parser::new(&tokens).parse();
    interpreter.interpret(&parser);
}
