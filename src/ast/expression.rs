use crate::interpreter::environment::Environment;
use crate::lox_object::{ Values};
use crate::token::{Token, TokenType};
use std::error::Error;
use std::{
    fmt::{Debug, Display},
    result::Result,
};
pub type DynExpr<'a> = Box<dyn Expr<'a> + 'a>;

pub enum ExprMetaData<'tok> {
    None,
    Var { token: Token<'tok> },
}
pub trait Expr<'tok>: Debug {
//    fn evaluate_to_obj(&self, env: &mut Environment<'tok>) -> Result<Object<'tok>, String>;
    fn evaluate_to_val(&self, env: &mut Environment<'tok>) -> Result<Values<'tok>, Box<dyn Error>> ;
//        self.evaluate_to_obj(env)?.into_value(env)
//    }
    fn metadata(&self) -> ExprMetaData<'tok> {
        ExprMetaData::None
    }
}
#[derive(Debug)]
pub struct BinaryOp<'a> {
    left: DynExpr<'a>,
    operator: Token<'a>,
    right: DynExpr<'a>,
}
#[derive(Debug)]
pub struct CallExpr<'a> {
    callee: DynExpr<'a>,
    paren: Token<'a>,
    arguments: Box<[DynExpr<'a>]>,
}

impl<'a> CallExpr<'a> {
    pub fn new(callee: DynExpr<'a>, paren: Token<'a>, arguments: Box<[DynExpr<'a>]>) -> Self {
        Self {
            callee,
            paren,
            arguments,
        }
    }
}
impl<'a> Expr<'a> for CallExpr<'a> {
    fn evaluate_to_val(&self, env: &mut Environment<'a>) -> Result<Values<'a>, Box<dyn Error>> {
        let callee = self.callee.evaluate_to_val(env)?;
        let mut arguments = Vec::with_capacity(self.arguments.len());
        for x in self.arguments.iter() {
            arguments.push(x.evaluate_to_val(env)?);
        }
        let arguments = arguments.into_boxed_slice();
        let Values::Fn(function) = callee else {
            return Err("Can only call function and classes..".into());
        };
        if function.arity()!=arguments.len(){
            return Err("Function have different arguments.".into());
        }
        return function.call(env, &arguments);
    }
}

#[derive(Debug)]
pub struct Variable<'a> {
    name: Token<'a>,
}
#[derive(Debug)]
pub struct Logical<'a> {
    left: DynExpr<'a>,
    operator: Token<'a>,
    right: DynExpr<'a>,
}

impl<'a> Logical<'a> {
    pub fn new(left: DynExpr<'a>, operator: Token<'a>, right: DynExpr<'a>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
impl<'a> Expr<'a> for Logical<'a> {
    fn evaluate_to_val(&self, env: &mut Environment<'a>) -> Result<Values<'a>,Box<dyn Error>> {
        let left = self.left.evaluate_to_val(env)?;
        let ans = match (self.operator.get_type(), left.is_truthy()) {
            (TokenType::Or, true) => left,
            (TokenType::And, false) => left,
            (x, y) if matches!(x, TokenType::Or | TokenType::And) => {
                self.right.evaluate_to_val(env)?
            }
            _ => panic!("There shoudn't be other toekn."),
        };
        return Ok(ans);
    }
}

impl Display for Variable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name, f)
    }
}
impl<'a> Expr<'a> for Variable<'a> {
    fn evaluate_to_val(&self, env: &mut Environment<'a>) -> Result<Values<'a>,Box<dyn Error>> {
        env.get(self.name.as_str()).ok_or("Variable not defined.".into()).cloned()
    }
    fn metadata(&self) -> ExprMetaData<'a> {
        ExprMetaData::Var { token: self.name }
    }
}
impl<'a> Variable<'a> {
    pub fn new(name: Token<'a>) -> Self {
        Variable { name }
    }
}
#[derive(Debug)]
pub struct Assign<'b> {
    name: Token<'b>,
    value: DynExpr<'b>,
}

impl<'b> Assign<'b> {
    pub fn new(name: Token<'b>, value: DynExpr<'b>) -> Self {
        Self { name, value }
    }
}
impl<'b> Expr<'b> for Assign<'b> {
    fn evaluate_to_val(&self, env: &mut Environment<'b>) -> Result<Values<'b>, Box<dyn Error>> {
        let value = self.value.evaluate_to_val(env)?;
        env.redefine(self.name.as_str(), value)?;
        Ok(Values::Null)
    }
}

#[derive(Debug)]
pub struct ValueStmt<'a>(Values<'a>);
impl<'a> From<Values<'a>> for Box<dyn Expr<'a>+'a> {
    fn from(value:Values<'a>) -> Self {
        Box::new(ValueStmt(value))
    }
}

impl<'a> From<Values<'a>> for ValueStmt<'a> {
    fn from(value: Values<'a>) -> Self {
        ValueStmt(value)
    }
}

impl Display for ValueStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl<'a> Expr<'a> for ValueStmt<'a> {
    fn evaluate_to_val(&self, _env: &mut Environment<'a>) -> Result<Values<'a>,Box<dyn Error>> {
        Ok(self.0.clone())
    }
}
impl<'a> BinaryOp<'a> {
    pub fn new(left: DynExpr<'a>, operator: Token<'a>, right: DynExpr<'a>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
#[derive(Debug)]
pub struct Grouping<'a> {
    expression: DynExpr<'a>,
}

impl<'a> Grouping<'a> {
    pub fn new(expression: DynExpr<'a>) -> Self {
        Self { expression }
    }
}
#[derive(Debug, Clone)]
pub struct Literal<'a> {
    token: Token<'a>,
}

impl<'a> Literal<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
    fn token_type(&self) -> TokenType {
        self.token.get_type()
    }
}
#[derive(Debug)]
pub struct Unary<'a> {
    operator: Token<'a>,
    right: DynExpr<'a>,
}

impl<'a> Unary<'a> {
    pub fn new(operator: Token<'a>, right: DynExpr<'a>) -> Self {
        Self { operator, right }
    }
}
impl<'a> Expr<'a> for BinaryOp<'a> {
    fn evaluate_to_val(&self, env: &mut Environment<'a>) -> Result<Values<'a>,Box<dyn Error>> {
        let left = self.left.evaluate_to_val(env)?;
        let right = self.right.evaluate_to_val(env)?;
        use TokenType::{
            BangEqual, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Minus, Plus, Slash, Star,
        };
        let ans=match self.operator.get_type() {
            Minus => left.sub(right),
            Plus => left.add(right),
            Star => left.mul(right),
            Slash => left.div(right),
            Greater => left.greater(&right),
            GreaterEqual => left.greater_equal(&right),
            Less => left.less(&right),
            LessEqual => left.less_equal(&right),
            EqualEqual => Ok(left.eq(&right)),
            BangEqual => Ok(left.neq(&right)),
            _ => Err("mismatched type sin binary operation.".into()),
        }?;
        Ok(ans)
    }
}
impl<'a> Expr<'a> for Grouping<'a> {
    fn evaluate_to_val(&self, env: &mut Environment<'a>) -> Result<Values<'a>,Box<dyn Error>> {
        self.expression.evaluate_to_val(env)
    }
}
impl<'a> Expr<'a> for Literal<'_> {
    fn evaluate_to_val(&self, _env: &mut Environment<'a>) -> Result<Values<'a>,Box<dyn Error>> {
        use TokenType::{False, Number, String, True};
        let ans = match self.token_type() {
            String => Values::Str(self.token.to_string()),
            Number => Values::Number(self.token.as_str().parse().unwrap()),
            True => Values::Boolean(1.0),
            False => Values::Boolean(0.0),
            _ => return Err("Unexpected value,wanted boolean,number or string".into()),
        };
        Ok(ans)
    }
}
impl<'a> Expr<'a> for Unary<'a> {
    fn evaluate_to_val(&self, env: &mut Environment<'a>) -> Result<Values<'a>,Box<dyn Error>> {
        let right:Values<'a> = self.right.evaluate_to_val(env)?;
        use TokenType::*;
        let ans=match self.operator.get_type() {
            Minus => right.negative(),
            Bang => right.cast_to_boolean().not(),
            _ => Err("Other operator is not allowed".into()),
        }?;
        Ok(ans)
    }
}

