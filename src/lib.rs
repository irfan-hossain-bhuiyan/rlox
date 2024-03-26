pub mod basic_function;
///It includes all the syntax tree,for the language,like expression and statememts.
pub mod ast;
///It has token data structure and Scanner that parse the token.
pub mod token;
///Create ast from token.
pub mod parser;
///run the ast
pub mod interpreter;
///This ia the value lox language support,Lox being a dynamic language,all types is actually one
///type.
pub mod lox_object;
///This is for dispalying error.
pub mod lox_error;
///This is to run lox from a file.It 
pub mod lox_runner;
