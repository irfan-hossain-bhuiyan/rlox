use std::error::Error;

use ascii::AsAsciiStr;

use crate::ast::statement::Block;
use crate::parser::Parser;
use crate::token::Scanner;
pub fn code_to_stblock<'a>(code:&'a str)->Result<Block<'a>,Box<dyn Error+'a>>{
    let code=code.as_ascii_str().unwrap();
    let token=match Scanner::new(code).scan_tokens(){
        Ok(x)=>x,
        Err(x)=>return Err(x.into()),
    };
    let ast=match Parser::new(&token).parse(){
        Ok(x)=>x,
        Err(x)=>return Err(x.into()),
    };
    Ok(ast.into())
}
