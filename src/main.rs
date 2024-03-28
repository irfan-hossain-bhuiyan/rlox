use std::{env, fs, io::stdout};

use rlox::{interpreter::Interpreter, lox_runner::code_to_stblock };

fn main(){
    let args:Vec<String>=env::args().collect();
    let path=match args.get(1){
        Some(x)=>x.as_str(),
        None=>"",
    };
    let path="test.lox";
    let code=fs::read_to_string(path).unwrap();
    let ast=code_to_stblock(&code).unwrap();
    println!("{:?}",ast);
    let mut stdout=stdout();
    let mut interpreter=Interpreter::new(&mut stdout);
    interpreter.interpret(&ast);
}
