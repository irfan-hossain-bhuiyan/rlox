use std::{io::stdout, str::FromStr};

use ascii::AsciiString;
use rlox::ast::statement::{Block, Stmt};
use rlox::interpreter;
use rlox::interpreter::environment::Environment;
use rlox::lox_error::emit_error;
use rlox::{interpreter::Interpreter, parser::Parser, token::Scanner};
fn main() {
    let mut stdout=stdout();
    let a = AsciiString::from_str("var a=10;print(a);print(10+20);").unwrap();
    let tokens = Scanner::new(&a).scan_tokens();
    let tokens = match tokens {
        Ok(x) => x,
        Err(x) => {
            emit_error(&x);
            return;
        }
    };
    let parser = Parser::new(&tokens).parse();
    let ast: Block<'_> = match parser {
        Ok(x) => {
            let x: Box<[Box<dyn Stmt>]> = x;
            let block: Block = x.into();
            block
        }
        Err(x) => {
            emit_error(&x);
            return;
        }
    };
    let mut interpreter=Interpreter::new(&mut stdout);
    interpreter.interpret(&ast);

}
