use std::str::FromStr;

use ascii::AsciiString;
use rlox::{basic_function::repl, interpreter::Interpreter, parser::Parser, token::Scanner};
fn main() {
    let mut interpreter=Interpreter::default();
    repl(|x|eval_expr(x,&mut interpreter));   
}
fn eval_expr(input: &str,interpreter:&mut Interpreter) {
    let Ok(input) = AsciiString::from_str(input) else {
        eprintln!("Can't convert to asciistring");
        return;
    };
    let token_tree = Scanner::new(&input).scan_tokens();
    let ast = Parser::new(&token_tree).parse();
    interpreter.interpret(&ast);
}
