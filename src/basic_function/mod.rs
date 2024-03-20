use std::{fmt::Display, io, os::unix::process};

pub fn to_string(x: Result<impl Display, impl Display>) -> String {
    match x {
        Ok(x) => x.to_string(),
        Err(x) => format!("Error:{}", x),
    }
}
pub fn print(x: Result<impl Display, impl Display>) {
    println!("{}", to_string(x));
}
pub fn repl(f: impl Fn(&str)) {
    loop {
    let mut x: String = String::new();
         if let Err(x)=io::stdin().read_line(&mut x){
             eprintln!("{}",x);
             continue;
         }
         if x.as_str()=="q"{return;}
         f(&x);
    }
}
