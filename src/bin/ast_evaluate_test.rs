use std::{io::stdout, str::FromStr};

use ascii::AsciiString;
use rlox::{basic_function::repl, interpreter::Interpreter, lox_error::emit_errors, parser::Parser, token::Scanner};
fn main() {
    let mut stdout=stdout();
    let mut interpreter=Interpreter::new(&mut stdout);
    interpreter.repl_mode();
    repl(|x|eval_expr(x,&mut interpreter));   
}
fn eval_expr(input: &str,interpreter:&mut Interpreter) {
    let Ok(input) = AsciiString::from_str(input) else {
        eprintln!("Can't convert to asciistring");
        return;
    };
    let Ok(token_tree) = Scanner::new(&input).scan_tokens() else{return;};
    let parsed = Parser::new(&token_tree).parse();
    match parsed{
        Ok(x)=>interpreter.interpret(&x),
        Err(x)=>emit_errors(&x),
    }
}
