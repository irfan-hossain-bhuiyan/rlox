use std::{error::Error, fmt::Display, mem::take};

use crate::{
    ast::{
        expression::{Assign, BinaryOp, Expr, Grouping, Literal, Unary, Value, Variable},
        statement::{Expression, Print, Block, Stmt, Var},
    },
    lox_object::{Object, Values},
    token::{Token, TokenType},
};
#[derive(Debug, Clone)]
pub struct Parser<'a, 'b: 'a> {
    source: &'a [Token<'b>],
    index: usize,
    errors: Vec<ParserError<'b>>,
}
#[derive(Debug, Clone, Copy)]
enum ParserErrorType {
    MissingSemicolon,
    InvalidAssignment,
    MissingRightParen,
    MissingValue,
    MissingVariable,
    MissingRightBrace,
}
impl ParserErrorType {
    fn to_str(&self) -> &'static str {
        match self {
            Self::MissingSemicolon => "Semicolon \";\" missing after statement.",
            Self::InvalidAssignment => "Right side of the assignment is not a variable.",
            Self::MissingValue => "There should be a value here.",
            Self::MissingRightParen => "Right Parenthethis \")\" is missing",
            Self::MissingVariable=>"Variable not found",
            Self::MissingRightBrace=>"Right Brace \"}\" is missing",
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ParserError<'a> {
    pos: Token<'a>,
    error_type: ParserErrorType,
}
impl Error for ParserError<'_> {}
pub type ParserErrors<'b> = Vec<ParserError<'b>>;
type Result<'b> = std::result::Result<Block<'b>, ParserErrors<'b>>;
impl Display for ParserError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = self.pos.span();
        let error_str = self.error_type.to_str();
        write!(f, "Error in {}\n Error:{}", pos, error_str)
    }
}
impl<'a> ParserError<'a> {
    fn new(pos: Token<'a>, error_type: ParserErrorType) -> Self {
        Self { pos, error_type }
    }
}
///program → declaration* EOF ;
///declaration → varDecl  | statement ;
///statement → exprStmt  | printStmt |  Block;
///Block -> "{" declaration* "}"
///varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;
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
        Self {
            source,
            index: 0,
            errors: Vec::new(),
        }
    }
    /// parse the syntax tree from the token stream.
    /// As for now it just parse expression.
    pub fn parse(&mut self) -> Result<'b> {
        let mut statements: Vec<Box<dyn Stmt>> = Vec::new();
        while !self.is_at_end() {
            if self.match_withs(&[TokenType::Eof]) {
                break;
            }
            statements.push(self.declaration());
        }
        match self.get_errors() {
            Some(x) => Err(x),
            None => Ok(statements.into()),
        }
    }
    /// expression->equilitypar
    fn expression(&mut self) -> Box<dyn Expr<'b> + 'b> {
        self.assignment()
    }
    fn assignment(&mut self) -> Box<dyn Expr<'b> + 'b> {
        let expr = self.equility();
        if self.match_withs(&[TokenType::Equal]) {
            let right = self.assignment();
            let name = expr.is_var().unwrap_or_else(|| {
                self.error(ParserErrorType::MissingVariable);
                Token::err_token()
            });
            return Box::new(Assign::new(name, right));
        }
        expr
    }
    /// equiltiy->comparasion ("!="|"!" comparasion)*
    fn equility(&mut self) -> Box<dyn Expr<'b> + 'b> {
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
    fn comparision(&mut self) -> Box<dyn Expr<'b> + 'b> {
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
    fn term(&mut self) -> Box<dyn Expr<'b> + 'b> {
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
    fn factor(&mut self) -> Box<dyn Expr<'b> + 'b> {
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
    fn unary(&mut self) -> Box<dyn Expr<'b> + 'b> {
        use TokenType::{Bang, Minus};
        if self.match_withs(&[Bang, Minus]) {
            let operator: Token<'_> = self.previous_token();
            let right: Box<dyn Expr + 'b> = self.unary();
            return Box::new(Unary::new(operator, right));
        }
        self.primary()
    }
    /// primary-> Literal | "(" expression ")"
    fn primary(&mut self) -> Box<dyn Expr<'b> + 'b> {
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
            if !self.match_withs(&[RightParen]) {
                self.error(ParserErrorType::MissingRightParen);
            }
            return Box::new(Grouping::new(expr));
        }
        self.error(ParserErrorType::MissingValue);
        Box::new(Value::new(Object::Value(Values::Null)))
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
    fn consume(&mut self, token_type: TokenType, error_type: ParserErrorType) -> Token<'b> {
        if self.is_at_end() || !self.match_withs(&[token_type]) {
            self.error(error_type);
        }
        self.previous_token()
    }

    fn statement(&mut self) -> Box<dyn Stmt + 'b> {
        if self.match_withs(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_withs(&[TokenType::LeftBrace]){
            return self.block_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Box<dyn Stmt + 'b> {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, ParserErrorType::MissingSemicolon);
        Box::new(Print::new(expr))
    }

    fn expression_statement(&mut self) -> Box<dyn Stmt + 'b> {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, ParserErrorType::MissingSemicolon);
        return Box::new(Expression::new(expr));
    }

    fn declaration(&mut self) -> Box<dyn Stmt + 'b> {
        if self.match_withs(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Box<dyn Stmt + 'b> {
        let name = self.consume(TokenType::Identifier, ParserErrorType::InvalidAssignment);
        let initilizer = if self.match_withs(&[TokenType::Equal]) {
            self.expression()
        } else {
            Into::<Object>::into(Values::Null).into()
        };
        self.consume(TokenType::Semicolon, ParserErrorType::MissingSemicolon);
        Box::new(Var::new(name, initilizer))
    }

    fn error(&mut self, error_type: ParserErrorType) {
        self.errors
            .push(ParserError::new(self.previous_token(), error_type));
    }

    fn get_errors(&mut self) -> Option<ParserErrors<'b>> {
        if self.errors.is_empty() {
            return None;
        }
        Some(take(&mut self.errors))
    }

    fn block_statement(&mut self) -> Box<dyn Stmt+'b> {
       let mut statements:Vec<Box<dyn Stmt>>=Vec::new();
       while !self.checks(&[TokenType::RightBrace]) || ! self.is_at_end() {
           statements.push(self.declaration());
       }
       self.consume(TokenType::RightBrace, ParserErrorType::MissingRightBrace);
       Box::new(Block::new(statements))
    }

    fn checks(&self, token_types: &[TokenType]) -> bool {
        let Some(token)=self.current_token() else{return false;};
        return token.matches_token(token_types);
    }

}
