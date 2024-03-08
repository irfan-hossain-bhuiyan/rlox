use std::fmt::{  Debug, Display};

use crate::token::Token;

pub trait Expr:Debug+Display{
}
#[derive(Debug)]
pub struct BinaryOp<'a>{
    left:Box<dyn Expr+'a>,
    operator:Token<'a>,
    right:Box<dyn Expr+'a>
}

impl<'a> BinaryOp<'a> {
    pub fn new(left: Box<dyn Expr+'a>, operator: Token<'a>, right: Box<dyn Expr+'a>) -> Self {
        Self { left, operator, right }
    }
}
#[derive(Debug)]
pub struct Grouping<'a>{
    expression:Box<dyn Expr+'a>
}

impl<'a> Grouping<'a> {
    pub fn new(expression: Box<dyn Expr+'a>) -> Self {
        Self { expression }
    }
}
#[derive(Debug,Clone)]
pub struct Literal<'a>{
    token:Token<'a>,
}

impl<'a> Literal<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}
#[derive(Debug)]
pub struct Unary<'a>{
    operator:Token<'a>,
    right:Box<dyn Expr+'a>
}

impl<'a> Unary<'a> {
    pub fn new(operator: Token<'a>, right: Box<dyn Expr+'a>) -> Self {
        Self { operator, right }
    }
}
impl Display for BinaryOp<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({}{} {})",self.operator.to_string(),self.left.to_string(),self.right.to_string())
    }
}

impl Expr for BinaryOp<'_>{}
impl Expr for Grouping<'_>{}
impl Display for Grouping<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"(Group {})",self.expression.to_string())
    }
}
impl Display for Literal<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.token.to_string())
    }
}
impl Expr for Literal<'_>{}
impl Display for Unary<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!{f,"({}{})",self.operator.to_string(),self.right.to_string()}
    }
}
impl Expr for Unary<'_>{}

