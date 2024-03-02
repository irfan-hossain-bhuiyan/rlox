use crate::token::Token;

trait Expr{

}
struct Binary<'a>{
    left:Box<dyn Expr>,
    operator:Token<'a>,
    right:Box<dyn Expr>
}
struct Grouping{
    expression:Box<dyn Expr>
}
struct Literal<'a>{
    token:Token<'a>,
}
struct Unary<'a>{
    operator:Token<'a>,
    right:Box<dyn Expr>
}
impl Expr for Binary<'_>{}
impl Expr for Grouping{}
impl Expr for Literal<'_>{}
impl Expr for Unary<'_>{}

