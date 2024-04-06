///program → declaration* EOF ;
///declaration → varDecl  | statement |funDelc;
///statement → exprStmt  | printStmt |  Block | ifStmt | whileStmt | forStmt;
///funDecl → "fun" function ;
///function → IDENTIFIER "(" parameters? ")" block ;
///parameters → IDENTIFIER ( "," IDENTIFIER )* ;
///whileStmt → "while" "(" expression ")" statement ;
///Block -> "{" declaration* "}"
///varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;
///ifStmt → "if" "(" expression ")" statement  ( "else" statement )? ;
///expression → assignment ;
///assignment → IDENTIFIER "=" assignment  | logic_or ;
///logic_or → logic_and ( "or" logic_and )* ;
///logic_and → equality ( "and" equality )* ;
use crate::{
    ast::expression::Expr,
    interpreter::environment::Environment,
    lox_object::{LoxFunction, Values},
    token::Token,
};
use std::{error::Error, fmt::Debug, rc::Rc};

use super::expression::DynExpr;
#[derive(Debug)]
pub struct If<'a> {
    condition: DynExpr<'a>,
    then_b: DynStmt<'a>,
    else_b: Option<DynStmt<'a>>,
}
#[derive(Debug)]
pub struct FunctionDelc<'a> {
    name: Token<'a>,
    paran: Box<[Token<'a>]>,
    body: RcStmt<'a>,
}

impl<'a> FunctionDelc<'a> {
    pub fn new(name: Token<'a>, paran: Box<[Token<'a>]>, body: DynStmt<'a>) -> Self {
        Self { name, paran, body:body.into() }
    }
}
impl<'a> Stmt<'a> for FunctionDelc<'a> {
    fn execute(&self, env: &mut Environment<'a>) -> Result<Option<Values<'a>>, Box<dyn Error>> {
        let func = LoxFunction::new(self.name, self.paran.clone(), self.body.clone(), env.get_current());
        env.define(self.name.to_string(), Values::Fn(Rc::new(func)));
        Ok(None)
        //env.define(self.function.name().to_string(),Values::Fn(self.function.clone()));
        //Ok(None)
    }
}
impl<'a> If<'a> {
    pub fn new(condition: DynExpr<'a>, then_b: DynStmt<'a>, else_b: Option<DynStmt<'a>>) -> Self {
        Self {
            condition,
            then_b,
            else_b,
        }
    }
}
#[derive(Debug)]
pub struct ReturnStmt<'a> {
    expr: DynExpr<'a>,
}
impl<'a> From<DynExpr<'a>> for ReturnStmt<'a> {
    fn from(value: DynExpr<'a>) -> Self {
        Self { expr: value }
    }
}
impl<'a> Stmt<'a> for ReturnStmt<'a> {
    fn execute(&self, env: &mut Environment<'a>) -> Result<Option<Values<'a>>, Box<dyn Error>> {
        let value = self.expr.evaluate_to_val(env)?;
        return Ok(Some(value));
    }
}
impl<'a> Stmt<'a> for If<'a> {
    fn execute(&self, env: &mut Environment<'a>) -> Result<Option<Values<'a>>, Box<dyn Error>> {
        let condition = self.condition.evaluate_to_val(env)?;
        if condition.is_truthy() {
            let ans = self.then_b.execute(env)?;
            if let Some(x) = ans {
                return Ok(Some(x));
            }
        } else if let Some(x) = self.else_b.as_ref() {
            if let Some(x) = x.execute(env)? {
                return Ok(Some(x));
            }
        }
        Ok(None)
    }
}
#[derive(Debug)]
pub struct WhileStmt<'a> {
    condition: DynExpr<'a>,
    body: DynStmt<'a>,
}

impl<'a> Stmt<'a> for WhileStmt<'a> {
    fn execute(&self, env: &mut Environment<'a>) -> Result<Option<Values<'a>>, Box<dyn Error>> {
        while self.condition.evaluate_to_val(env)?.is_truthy() {
            if let Some(x) = self.body.execute(env)? {
                return Ok(Some(x));
            }
        }
        Ok(None)
    }
}

impl<'a> WhileStmt<'a> {
    pub fn new(condition: DynExpr<'a>, body: DynStmt<'a>) -> Self {
        Self { condition, body }
    }
}
pub type DynStmt<'a> = Box<dyn Stmt<'a> + 'a>;
pub type RcStmt<'a> = Rc<dyn Stmt<'a> + 'a>;
pub trait Stmt<'a>: Debug {
    fn execute(&self, env: &mut Environment<'a>) -> Result<Option<Values<'a>>, Box<dyn Error>>;
}
#[derive(Debug, Default)]
pub struct Block<'a> {
    source: Box<[DynStmt<'a>]>,
}

impl<'a> From<Statements<'a>> for Block<'a> {
    fn from(value: Statements<'a>) -> Self {
        Self::from(value.source)
    }
}
#[derive(Debug, Default)]
pub struct Statements<'a> {
    source: Box<[DynStmt<'a>]>,
}

impl<'a> From<Box<[DynStmt<'a>]>> for Statements<'a> {
    fn from(source: Box<[DynStmt<'a>]>) -> Self {
        Self { source }
    }
}

impl<'a> From<Box<[DynStmt<'a>]>> for Block<'a> {
    fn from(source: Box<[DynStmt<'a>]>) -> Self {
        Self { source }
    }
}
impl<'a> Stmt<'a> for Statements<'a> {
    fn execute(&self, env: &mut Environment<'a>) -> Result<Option<Values<'a>>, Box<dyn Error>> {
        for x in self.source.iter() {
            if let Some(x) = x.execute(env)? {
                return Ok(Some(x));
            }
        }
        Ok(None)
    }
}
impl<'a> From<Vec<DynStmt<'a>>> for Statements<'a> {
    fn from(value: Vec<DynStmt<'a>>) -> Self {
        Self {
            source: value.into(),
        }
    }
}
impl<'a> From<Vec<DynStmt<'a>>> for Block<'a> {
    fn from(value: Vec<DynStmt<'a>>) -> Self {
        Self {
            source: value.into(),
        }
    }
}
//impl Display for Block<'_>{
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        for x in self.source.iter(){
//            write!(f,"{};\n",x)?
//        }
//        Ok(())
//    }
//}
impl<'a> Stmt<'a> for Block<'a> {
    fn execute(&self, env: &mut Environment<'a>) -> Result<Option<Values<'a>>, Box<dyn Error>> {
        env.create_sub_values();
        for x in self.source.iter() {
            if let Some(x) = x.execute(env)? {
                return Ok(Some(x));
            }
        }
        env.delete_sub_values();
        Ok(None)
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
impl<'a> Var<'a> {
    pub fn new(name: Token<'a>, initializer: Box<dyn Expr<'a> + 'a>) -> Self {
        Self { name, initializer }
    }
}

impl<'a> Stmt<'a> for Var<'a> {
    fn execute(&self, env: &mut Environment<'a>) -> Result<Option<Values<'a>>, Box<dyn Error>> {
        let val = self.initializer.evaluate_to_val(env)?;
        env.define(self.name.to_string(), val);
        Ok(None)
    }
}
impl<'a> Expression<'a> {
    pub fn new(expression: Box<dyn Expr<'a> + 'a>) -> Self {
        Self { expression }
    }
}
//#[derive(Debug)]
//pub struct Print<'a> {
//    expression: Box<dyn Expr<'a> + 'a>,
//}
//
//impl<'a> Print<'a> {
//    pub fn new(expression: Box<dyn Expr<'a> + 'a>) -> Self {
//        Self { expression }
//    }
//}
impl<'a> Stmt<'a> for Expression<'a> {
    fn execute(&self, env: &mut Environment<'a>) -> Result<Option<Values<'a>>, Box<dyn Error>> {
        self.expression.evaluate_to_val(env)?;
        Ok(None)
    }
}
//impl<'a> Stmt<'a> for Print<'a> {
//    fn execute(&self, env: &mut Environment<'a>) -> Result<(), Box<dyn Error>> {
//        let value = self.expression.evaluate_to_val(env)?;
//        env.writeln(&value.to_string())?;
//        Ok(())
//    }
//}
