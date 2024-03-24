pub mod environment;

use std::ops::Deref;

use environment::Environment;

use crate::{ast::statement::Stmt, lox_error};
#[derive(Default,Debug)]
struct Interpreter{
    env:Environment,
}
impl Interpreter{
    fn interpret(&mut self, statements:Vec<Box<dyn Stmt>>){
        for x in statements{
            if let Err(x)=x.execute(&mut self.env){
                lox_error::emit_runtime_error(x.deref());
            }
        }
    }
}
