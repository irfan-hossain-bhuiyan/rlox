use crate::{
    ast::{BinaryOp, Expr, Grouping, Literal, Unary},
    token::{Token, TokenType},
};
#[derive(Debug, Clone)]
pub struct Parser<'a, 'b: 'a> {
    source: &'a [Token<'b>],
    index: usize,
}
/// expression->equility
/// equiltiy->comparasion ("!="|"==" comparasion)*
/// comparasion -> term ("<"|"<="|">"|">=" term)*
/// term ->factor ("+"|"-" factor)*
/// factor-> unary ("*"|"/" unary)*
/// unary->  ("!"|"-" unary) | primary
/// primary-> Literal | "(" expression ")"
impl<'a, 'b: 'a> From<&'a [Token<'b>]> for Parser<'a, 'b> {
    fn from(value: &'a [Token<'b>]) -> Self {
        Self::new(value)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> {
    pub fn new(source: &'a [Token<'b>]) -> Self {
        Self { source, index: 0 }
    }
    /// parse the syntax tree from the token stream.
    /// As for now it just parse expression.
    pub fn parse(&mut self) -> Box<dyn Expr + 'b> {
        self.expression()
    }
    /// expression->equility
    fn expression(&mut self) -> Box<dyn Expr + 'b> {
        self.equility()
    }
    /// equiltiy->comparasion ("!="|"!" comparasion)*
    fn equility(&mut self) -> Box<dyn Expr + 'b> {
        use TokenType::{EqualEqual, BangEqual};
        let mut expr: Box<dyn Expr + 'b> = self.comparision();
        while self.match_withs(&[EqualEqual, BangEqual]) {
            let operator: Token<'_> = self.previous_token();
            let right: Box<dyn Expr + 'b> = self.comparision();
            expr = Box::new(BinaryOp::new(expr, operator, right));
        }
        expr
    }
    /// comparasion -> term ("<"|"<="|">"|">=" term)*
    fn comparision(&mut self) -> Box<dyn Expr + 'b> {
        use TokenType::{Greater, GreaterEqual, Less, LessEqual};
        let mut expr = self.term();
        while self.match_withs(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator: Token<'_> = self.previous_token();
            let right: Box<dyn Expr + 'b> = self.term();
            expr = Box::new(BinaryOp::new(expr, operator, right));
        }
        expr
    }
    /// term ->factor ("+"|"-" factor)*
    fn term(&mut self) -> Box<dyn Expr + 'b> {
        use TokenType::{Minus, Plus};
        let mut expr: Box<dyn Expr> = self.factor();
        while self.match_withs(&[Plus, Minus]) {
            let operator = self.previous_token();
            let right = self.factor();
            expr = Box::new(BinaryOp::new(expr, operator, right));
        }
        expr
    }
    /// factor-> unary ("*"|"/" unary)*
    fn factor(&mut self) -> Box<dyn Expr + 'b> {
        use TokenType::{Slash, Star};
        let mut expr: Box<dyn Expr + 'b> = self.unary();
        while self.match_withs(&[Slash, Star]) {
            let operator: Token<'_> = self.previous_token();
            let right = self.unary();
            expr = Box::new(BinaryOp::new(expr, operator, right));
        }
        expr
    }
    /// unary->  ("!"|"-" unary) | primary
    fn unary(&mut self) -> Box<dyn Expr + 'b> {
        use TokenType::{Bang, Minus};
        if self.match_withs(&[Bang, Minus]) {
            let operator: Token<'_> = self.previous_token();
            let right: Box<dyn Expr + 'b> = self.unary();
            return Box::new(Unary::new(operator, right));
        }
        self.primary()
    }
    /// primary-> Literal | "(" expression ")"
    fn primary(&mut self) -> Box<dyn Expr + 'b> {
        use TokenType::{False, LeftParen, Nil, Number, RightParen, String, True};
        if self.match_withs(&[True, False, Nil, Number, String]) {
            let literal = self.previous_token();
            return Box::new(Literal::new(literal));
        }
        if self.match_withs(&[LeftParen]) {
            let expr: Box<dyn Expr + 'b> = self.expression();
            if self.match_withs(&[RightParen]) {
                return Box::new(Grouping::new(expr));
            }
            panic!("There is no right pranthetics in the right side of the expression {}",expr);
        }
        panic!("The token is {:?} \nThere should be a expression here.",self.current_token());
    }
    fn match_withs(&mut self, token_types: &[TokenType]) -> bool {
        let Some(current_token) = self.current_token() else {
            return false;
        };
        if current_token.matches_token(token_types) {
            self.advance();
            return true;
        }
        false
    }
    fn current_token(&self) -> Option<Token<'b>> {
        self.source.get(self.index).cloned()
    }
    fn previous_token(&self) -> Token<'b> {
        self.source[self.index - 1]
    }
    fn advance(&mut self) {
        if self.is_at_end() {
            return;
        }
        self.index += 1;
    }
    fn is_at_end(&self) -> bool {
        self.source.len() <= self.index
    }
}
