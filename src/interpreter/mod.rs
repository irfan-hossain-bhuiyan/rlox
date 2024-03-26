#[macro_use]
pub mod environment;

use std::{io::Write, ops::Deref};

use environment::Environment;

use crate::{ast::statement::Stmt, lox_error::emit_error};
#[derive(Debug)]
pub struct Interpreter<'input>{
    env:Environment<'input>,
}
impl<'a> Interpreter<'a>{
    pub fn new(stdout:&'a mut dyn Write)->Self{
        Self{
            env:Environment::new(stdout),
        }
    }
    pub fn interpret(&mut self, statement:&dyn Stmt){
        if let Err(x)=statement.execute(&mut self.env){
            emit_error(x.deref());
        }
    }
    ///Setup the environment espicially for repl,
    pub fn repl_mode(&mut self){
        self.env.create_sub_values();
    }
}
