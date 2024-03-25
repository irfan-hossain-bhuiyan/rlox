pub mod environment;

use std::ops::Deref;

use environment::Environment;

use crate::{ast::statement::Stmt, lox_error::emit_runtime_error};
#[derive(Default,Debug)]
pub struct Interpreter{
    env:Environment,
}
impl Interpreter{
    pub fn interpret(&mut self, statement:&dyn Stmt){
        if let Err(x)=statement.execute(&mut self.env){
            emit_runtime_error(x.deref());
        }
    }
}
