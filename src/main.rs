use std::{env, fs};

fn main(){
    let args:Vec<String>=env::args().collect();
    let path=match args.get(1){
        Some(x)=>x,
        None=>todo!(),
    };
    match fs::read_to_string(path) {
        Some(x)=>lox_runner(x),
        None=>todo!(),
    }

}
