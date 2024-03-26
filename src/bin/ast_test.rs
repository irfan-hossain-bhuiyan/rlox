use std::{io::stdout, ops::Deref, str::FromStr};

use rlox::{interpreter::Interpreter, lox_error::emit_errors, parser::Parser, token::Scanner};
use ascii::AsciiString;
fn main(){
    let mut stdout=stdout();
    let mut interpreter=Interpreter::new(&mut stdout);
    let a=AsciiString::from_str("var a=10;print(a);print(10+20);").unwrap();
    let tokens=Scanner::new(&a).scan_tokens();
    let tokens=match tokens{
        Ok(x)=>x,
        Err(x)=>{emit_errors(&x);return;}
    };
    let parser=Parser::new(&tokens).parse();
    match parser{
        Ok(x)=>interpreter.interpret(&x),
        Err(x)=>emit_errors(&x),
    }
}
