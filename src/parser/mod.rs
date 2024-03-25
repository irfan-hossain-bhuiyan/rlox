
use crate::{
    ast::{
        expression::{Assign, BinaryOp, Expr, Grouping, Literal, Unary, Variable},
        statement::{Expression, Print, Statements, Stmt, Var},
    },
    lox_object::{Object, Values},
    token::{Token, TokenType},
};
#[derive(Debug, Clone)]
pub struct Parser<'a, 'b: 'a> {
    source: &'a [Token<'b>],
    index: usize,
}
/// expression->assignment
/// assignment → IDENTIFIER "=" assignment  | equality ;
/// equiltiy->comparasion ("!="|"==" comparasion)*
/// comparasion -> term ("<"|"<="|">"|">=" term)*
/// term ->factor ("+"|"-" factor)*
/// factor-> unary ("*"|"/" unary)*
/// unary->  ("!"|"-" unary) | primary
/// primary-> Literal | "(" expression ")"
/// LIteral-> Values | Variable
impl<'a, 'b: 'a> From<&'a [Token<'b>]> for Parser<'a, 'b> {
    fn from(value: &'a [Token<'b>]) -> Self {
        Self::new(value)
    }
}

impl<'a, 'b: 'b> Parser<'a, 'b> {
    pub fn new(source: &'a [Token<'b>]) -> Self {
        Self { source, index: 0 }
    }
    /// parse the syntax tree from the token stream.
    /// As for now it just parse expression.
    pub fn parse(&mut self) -> Statements<'b> {
        let mut statements: Vec<Box<dyn Stmt>> = Vec::new();
        while !self.is_at_end() {
            if self.match_withs(&[TokenType::Eof]) {
                break;
            }
            statements.push(self.declaration());
        }
        return statements.into();
    }
    /// expression->equilitypar
    fn expression(&mut self) -> Box<dyn Expr + 'b> {
        self.assignment()
    }
    fn assignment(&mut self) -> Box<dyn Expr + 'b> {
        let expr: Box<dyn Expr + 'b> = self.equility();
        if self.match_withs(&[TokenType::Equal]) {
            let right = self.assignment();
            if expr.is_var().is_none() {
                panic!("invalid assignment.Left side of the expression is not valid.");
            };
            return Box::new(Assign::new(expr, right));
        }
        expr
    }
    /// equiltiy->comparasion ("!="|"!" comparasion)*
    fn equility(&mut self) -> Box<dyn Expr + 'b> {
        use TokenType::{BangEqual, EqualEqual};
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
        let mut expr: Box<dyn Expr + 'b> = self.factor();
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
        use TokenType::{False, Identifier, LeftParen, Nil, Number, RightParen, String, True};
        if self.match_withs(&[True, False, Nil, Number, String]) {
            let literal = self.previous_token();
            return Box::new(Literal::new(literal));
        }
        if self.match_withs(&[Identifier]) {
            return Box::new(Variable::new(self.previous_token()));
        }
        if self.match_withs(&[LeftParen]) {
            let expr: Box<dyn Expr + 'b> = self.expression();
            if self.match_withs(&[RightParen]) {
                return Box::new(Grouping::new(expr));
            }
            panic!(
                "There is no right pranthetics in the right side of the expression {}",
                expr
            );
        }
        panic!(
            "The token is {:?} \nThere should be a expression here.",
            self.current_token()
        );
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
    fn consume(&mut self, token_type: TokenType, panic_msg: &str) -> Token<'b> {
        if self.is_at_end() || !self.match_withs(&[token_type]) {
            panic!("{}", panic_msg);
        }
        self.previous_token()
    }

    fn statement(&mut self) -> Box<dyn Stmt + 'b> {
        if self.match_withs(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Box<dyn Stmt + 'b> {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expected ; after statement.");
        Box::new(Print::new(expr))
    }

    fn expression_statement(&mut self) -> Box<dyn Stmt + 'b> {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expected ; after statment.");
        return Box::new(Expression::new(expr));
    }

    fn declaration(&mut self) -> Box<dyn Stmt + 'b> {
        if self.match_withs(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Box<dyn Stmt + 'b> {
        let name = self.consume(TokenType::Identifier, "expected variable name.");
        let initilizer = if self.match_withs(&[TokenType::Equal]) {
            self.expression()
        } else {
            Into::<Object>::into(Values::Null).into()
        };
        self.consume(TokenType::Semicolon, "Expected ; after statement.");
        Box::new(Var::new(name, initilizer))
    }
}
