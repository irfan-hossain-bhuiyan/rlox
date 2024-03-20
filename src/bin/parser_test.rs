use ascii::AsciiString;
use rlox::parser::Parser;
use rlox::token::Scanner;
use std::io;
use std::str::FromStr;
//"2+3+(2>3)+(2<3)+3*4"
fn main()->io::Result<()> {
    loop{
        let mut x=String::new();
        io::stdin().read_line(&mut x)?;
        if x.as_str()=="q"{break;}
        let string = AsciiString::from_str(&x).unwrap();
        let tokens = Scanner::new(&string).scan_tokens();
        let parser = Parser::new(&tokens).parse();
        println!("string is:{}", string);
        println!("tokens is:{:?}", tokens);
        println!("parsed is:{}", parser);
    }
    Ok(())
}
