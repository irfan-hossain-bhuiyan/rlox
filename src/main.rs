use std::{env, fs, io::stdout};

use rlox::{interpreter::Interpreter, lox_runner::code_to_stblock};

fn main() {
    let mut stdout = stdout();
    let args: Vec<String> = env::args().collect();
    let _path = match args.get(1) {
        Some(x) => x.as_str(),
        None => "",
    };
    let path = "tst2.lox";
    let code = fs::read_to_string(path).unwrap();

    let mut interpreter = Interpreter::new(&mut stdout);
    let ast = code_to_stblock(&code).unwrap();
    //println!("{:?}",ast);
    interpreter.interpret(&ast);
}
