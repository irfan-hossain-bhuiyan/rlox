use std::{error::Error, fmt::{Debug, Display}};

use crate::{ast::expression::Expr, interpreter::environment::Environment,
    token::Token,
};
///program → declaration* EOF ;
///declaration → varDecl  | statement ;
///statement → exprStmt  | printStmt ;
///varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;
pub trait Stmt: Display+Debug {
    fn execute(&self, env: &mut Environment) -> Result<(), Box<dyn Error>>;
}
#[derive(Debug,Default)]
pub struct Statements<'a>{
    source:Vec<Box<dyn Stmt+'a>>,
}

impl<'a> Statements<'a> {
    pub fn new(source: Vec<Box<dyn Stmt+'a>>) -> Self {
        Self { source }
    }
}
impl<'a> From<Vec<Box<dyn Stmt+'a>>> for Statements<'a>{
    fn from(value: Vec<Box<dyn Stmt+'a>>) -> Self {
        Self::new(value)
    }
}
impl Display for Statements<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.source.iter(){
            write!(f,"{};\n",x)?
        }
        Ok(())
    }
}
impl Stmt for Statements<'_>{
    fn execute(&self, env: &mut Environment) -> Result<(), Box<dyn Error>> {
        for x in self.source.iter(){
            x.execute(env)?
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct Expression<'a> {
    expression: Box<dyn Expr<'a> + 'a>,
}
#[derive(Debug)]
pub struct Var<'a> {
    name: Token<'a>,
    initializer: Box<dyn Expr<'a> + 'a>,
}
impl Display for Var<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} = {}",self.name,self.initializer)
    }
}

impl<'a> Var<'a> {
    pub fn new(name: Token<'a>, initializer: Box<dyn Expr<'a> + 'a>) -> Self {
        Self { name, initializer }
    }
}
impl Stmt for Var<'_> {
    fn execute(&self, env: &mut Environment) -> Result<(), Box<dyn Error>> {
        let val = self.initializer.evaluate_to_val(env)?;
        env.define(self.name.to_string(), val);
        Ok(())
    }
}
impl<'a> Expression<'a> {
    pub fn new(expression: Box<dyn Expr<'a> + 'a>) -> Self {
        Self { expression }
    }
}
impl Display for Expression<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{};",self.expression)
    }
}
#[derive(Debug)]
pub struct Print<'a> {
    expression: Box<dyn Expr<'a> + 'a>,
}

impl<'a> Print<'a> {
    pub fn new(expression: Box<dyn Expr<'a> + 'a>) -> Self {
        Self { expression }
    }
}
impl Display for Print<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Print({})",self.expression)
    }
}

impl Stmt for Expression<'_> {
    fn execute(&self, env: &mut Environment) -> Result<(), Box<dyn Error>> {
        self.expression.evaluate_to_val(env)?;
        Ok(())
    }
}
impl Stmt for Print<'_> {
    fn execute(&self, env: &mut Environment) -> Result<(), Box<dyn Error>> {
        let value=self.expression.evaluate_to_val(env)?;
        env.writeln(&value.to_string())?;
        Ok(())
    }
}
