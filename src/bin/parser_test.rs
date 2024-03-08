use std::str::FromStr;

use rlox::parser;
use rlox::parser::Parser;
use ascii::AsciiString;
use rlox::token::Scanner;
fn main(){
    let string=AsciiString::from_str("2+3*(4+2)<=32+32*(54-32)").unwrap();
    let tokens=Scanner::new(&string).scan_tokens();
    let parser=Parser::new(&tokens).parse();
    println!("string is:{}",string);
    println!("{}",parser);
}
