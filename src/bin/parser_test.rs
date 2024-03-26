use ascii::AsciiString;
use rlox::lox_error::emit_errors;
use rlox::parser::Parser;
use rlox::token::Scanner;
use std::io;
use std::str::FromStr;
//"2+3+(2>3)+(2<3)+3*4"
fn main() {
    loop{
        let mut x=String::new();
        io::stdin().read_line(&mut x).unwrap();
        if x.as_str()=="q"{break;}
        let string = AsciiString::from_str(&x).unwrap();
        let tokens = Scanner::new(&string).scan_tokens();
        let tokens=match tokens{
            Ok(x)=>x,
            Err(x)=>{emit_errors(&x);return ;}
        };
        let parser = Parser::new(&tokens).parse();
        println!("string is:{}", string);
        println!("tokens is:{:?}", tokens);
        println!("parsed is:{}", parser.unwrap());
    }
}
