use std::{io::stdout, str::FromStr};

use rlox::ast::statement::Block;
use rlox::{interpreter::Interpreter, parser::Parser, token::Scanner};
use rlox::lox_error::emit_error;
use ascii::AsciiString;
fn main(){
    let mut stdout=stdout();
    let mut interpreter=Interpreter::new(&mut stdout);
    let a=AsciiString::from_str("var a=10;print(a);print(10+20);").unwrap();
    let tokens=Scanner::new(&a).scan_tokens();
    let tokens=match tokens{
        Ok(x)=>x,
        Err(x)=>{emit_error(&x);return;}
    };
    let parser=Parser::new(&tokens).parse();
    match parser{
        Ok(x)=>{
            let x:Block=x.into();
            interpreter.interpret(&x)
        },
        Err(x)=>emit_error(&x),
    }
}
