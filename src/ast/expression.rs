use crate::interpreter::environment::Environment;
use crate::lox_object::{Object, Values};
use crate::token::{Token, TokenType};
use std::{
    fmt::{Debug, Display},
    result::Result,
};
pub trait Expr<'tok>: Display + Debug {
    fn evaluate_to_obj(&self, env: &mut Environment) -> Result<Object, String>;
    fn evaluate_to_val(&self, env: &mut Environment) -> Result<Values, String> {
        self.evaluate_to_obj(env)?.into_value(env)
    }
    fn is_var(& self) -> Option<Token<'tok>> {
        None
    }
}
#[derive(Debug)]
pub struct BinaryOp<'a> {
    left: Box<dyn Expr<'a> + 'a>,
    operator: Token<'a>,
    right: Box<dyn Expr<'a> + 'a>,
}
#[derive(Debug)]
pub struct Variable<'a> {
    name:  Token<'a>,
}
impl Display for Variable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name, f)
    }
}
impl<'a> Expr<'a> for Variable<'a> {
    fn evaluate_to_obj(&self, _env: &mut Environment) -> Result<Object, String> {
        Ok(Object::Var {
            name: self.name.to_string(),
        })
    }
    fn is_var(&self) -> Option<Token<'a>> {
        Some(self.name)
    }
}
impl<'a> Variable<'a> {
    pub fn new(name:Token<'a>) -> Self {
        Variable { name }
    }
}
#[derive(Debug)]
pub struct Assign<'b> {
    name: Token<'b>,
    value: Box<dyn Expr<'b> + 'b>,
}

impl<'b> Assign<'b> {
    pub fn new(name: Token<'b>, value: Box<dyn Expr<'b> + 'b>) -> Self {
        Self { name, value }
    }
}
impl Display for Assign<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}
impl<'b> Expr<'b> for Assign<'b> {
    fn evaluate_to_obj(&self, env: &mut Environment) -> Result<Object, String> {
        let value = self.value.evaluate_to_val(env)?;
        env.redefine(self.name.as_str(), value)?;
        Ok(Values::Null.into())
    }
}

#[derive(Debug)]
pub struct Value(Object);
impl From<Object> for Box<dyn Expr<'_>> {
    fn from(value: Object) -> Self {
        Box::new(Value(value))
    }
}
impl Value {
    pub fn new(object: Object) -> Self {
        Value(object)
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl Expr<'_> for Value {
    fn evaluate_to_obj(&self, _env: &mut Environment) -> Result<Object, String> {
        Ok(self.0.clone())
    }
}
impl<'a> BinaryOp<'a> {
    pub fn new(left: Box<dyn Expr<'a> + 'a>, operator: Token<'a>, right: Box<dyn Expr<'a> + 'a>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
#[derive(Debug)]
pub struct Grouping<'a> {
    expression: Box<dyn Expr<'a> + 'a>,
}

impl<'a> Grouping<'a> {
    pub fn new(expression: Box<dyn Expr<'a> + 'a>) -> Self {
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
    right: Box<dyn Expr<'a> + 'a>,
}

impl<'a> Unary<'a> {
    pub fn new(operator: Token<'a>, right: Box<dyn Expr<'a> + 'a>) -> Self {
        Self { operator, right }
    }
}
impl Display for BinaryOp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}{} {})",
            self.operator.to_string(),
            self.left.to_string(),
            self.right.to_string()
        )
    }
}

impl Expr<'_> for BinaryOp<'_> {
    fn evaluate_to_obj(&self, env: &mut Environment) -> Result<Object, String> {
        let left = self.left.evaluate_to_val(env)?;
        let right = self.right.evaluate_to_val(env)?;
        use TokenType::{
            BangEqual, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Minus, Plus, Slash, Star,
        };
        match self.operator.get_type() {
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
        }
        .map(|x| x.into())
    }
}
impl Expr<'_> for Grouping<'_> {
    fn evaluate_to_obj(&self, env: &mut Environment) -> Result<Object, String> {
        self.expression.evaluate_to_obj(env)
    }
}
impl Display for Grouping<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Group {})", self.expression.to_string())
    }
}
impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.to_string())
    }
}
impl Expr<'_> for Literal<'_> {
    fn evaluate_to_obj(&self, _env: &mut Environment) -> Result<Object, String> {
        use TokenType::{False, Number, String, True};
        let ans = match self.token_type() {
            String => Values::Str(self.to_string()),
            Number => Values::Number(self.token.as_str().parse().unwrap()),
            True => Values::Boolean(1.0),
            False => Values::Boolean(0.0),
            _ => return Err("Unexpected value,wanted boolean,number or string".into()),
        };
        Ok(ans.into())
    }
}
impl Display for Unary<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! {f,"({}{})",self.operator.to_string(),self.right.to_string()}
    }
}
impl Expr<'_> for Unary<'_> {
    fn evaluate_to_obj(&self, env: &mut Environment) -> Result<Object, String> {
        let right = self.right.evaluate_to_val(env)?;
        use TokenType::*;
        match self.operator.get_type() {
            Minus => right.negative(),
            Bang => right.is_truthy().not(),
            _ => Err("Other operator is not allowed".into()),
        }
        .map(|x| x.into())
    }
}
