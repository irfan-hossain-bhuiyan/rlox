use std::{io::stdout, ops::Deref};

use rlox::{basic_function::repl, interpreter::Interpreter, lox_error::emit_error, lox_runner::code_to_stblock};
fn main() {
    let mut stdout=stdout();
    let mut interpreter=Interpreter::new(&mut stdout);
    interpreter.repl_mode();
    repl(|x|eval_expr(x,&mut interpreter));   
}
fn eval_expr(input: &str,interpreter:&mut Interpreter) {
    let ast=match code_to_stblock(input){
        Ok(x)=>x,
        Err(x)=>{
            emit_error(x.deref());return;
        }
    };
    interpreter.interpret(&ast);
   // let Ok(input) = AsciiString::from_str(input) else {
   //     eprintln!("Can't convert to asciistring");
   //     return;
   // };
   // let Ok(token_tree) = Scanner::new(&input).scan_tokens() else{return;};
   // let parsed = Parser::new(&token_tree).parse();
   // match parsed{
   //     Ok(x)=>interpreter.interpret(&x),
   //     Err(x)=>emit_errors(&x),
   // }
}
