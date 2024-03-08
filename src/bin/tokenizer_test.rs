use std::str::FromStr;

use ascii::AsciiString;
use rlox::token::Scanner;

fn main(){
    let s:AsciiString=AsciiString::from_str("(1+3.5)=4;
                                            for x in range(10):
                                                print x").unwrap();
    let v=Scanner::new(&s).scan_tokens();
    println!("{:?}",v);
    
}
