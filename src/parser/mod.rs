use std::{error::Error, fmt::Display, mem::take};

use crate::{
    ast::{
        expression::{
            Assign, BinaryOp, CallExpr, DynExpr, Expr, ExprMetaData, Grouping, Literal, Logical,
            Unary, ValueStmt, Variable,
        },
        statement::{Block, DynStmt, Expression, FunctionDelc, If, ReturnStmt, Stmt, Var, WhileStmt},
    },
    lox_error::Errors,
    lox_object::{LoxFunction, Values},
    token::{Token, TokenType},
};
#[derive(Debug, Clone)]
pub struct Parser<'a, 'b: 'a> {
    source: &'a [Token<'b>],
    index: usize,
    errors: Vec<ParserError<'b>>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum ParserErrorType {
    MissingSemicolon,
    InvalidAssignment,
    MissingRightParen,
    MissingValue,
    MissingVariable,
    MissingRightBrace,
    MissingLeftParen,
    MissingLeftBrace,
    MissingIdentifier(&'static str),
}
impl ParserErrorType {
    fn to_str(&self) -> &'static str {
        match self {
            Self::MissingSemicolon => "Semicolon \";\" missing after statement.",
            Self::InvalidAssignment => "Right side of the assignment is not a variable.",
            Self::MissingValue => "There should be a value here.",
            Self::MissingRightParen => "Right Parenthethis \")\" is missing",
            Self::MissingLeftParen => "Left Parenthethis \"(\" is missing",
            Self::MissingVariable => "Variable not found",
            Self::MissingRightBrace => "Right Brace \"}\" is missing",
            Self::MissingLeftBrace => "Left Brace \"{\" is missing",
            Self::MissingIdentifier(x) => "Missing Identifier",
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ParserError<'a> {
    pos: Token<'a>,
    error_type: ParserErrorType,
}
impl Error for ParserError<'_> {}
pub type ParserErrors<'b> = Errors<ParserError<'b>>;
type Stmts<'b> = Box<[Box<dyn Stmt<'b> + 'b>]>;
type Result<'b> = std::result::Result<Stmts<'b>, ParserErrors<'b>>;
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
/// unary->  ("!"|"-") unary | primary
///call → primary ( "(" arguments? ")" )* ;
///arguments → expression ( "," expression )* ;
/// primary-> Literal | "(" expression ")"
/// LIteral-> Values | Variable
impl<'a, 'b: 'a> From<&'a [Token<'b>]> for Parser<'a, 'b> {
    fn from(value: &'a [Token<'b>]) -> Self {
        Self::new(value)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> {
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
        while !self.is_eof() {
            //if self.match_withs(&[TokenType::Eof]) {
            //    break;
            //}
            statements.push(self.declaration());
        }
        match self.get_errors() {
            Some(x) => Err(x),
            None => Ok(statements.into()),
        }
    }
    /// expression->equilitypar
    fn expression(&mut self) -> DynExpr<'b> {
        self.assignment()
    }
    fn assignment(&mut self) -> DynExpr<'b> {
        let expr = self.or();
        if self.match_withs(&[TokenType::Equal]) {
            let right = self.assignment();
            let name = match expr.metadata() {
                ExprMetaData::Var { token } => token,
                ExprMetaData::None => {
                    self.error(ParserErrorType::MissingVariable);
                    Token::err_token()
                }
            };
            return Box::new(Assign::new(name, right));
        }
        expr
    }
    fn or(&mut self) -> DynExpr<'b> {
        let mut expr = self.and();
        while self.match_with(TokenType::Or) {
            let operator = self.previous_token();
            let right = self.and();
            expr = Box::new(Logical::new(expr, operator, right));
        }
        return expr;
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
        self.call()
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
        Box::new(ValueStmt::from(Values::Null))
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
    ///Eof means end of file.
    fn is_eof(&self) -> bool {
        match self.current_token() {
            None => true,
            Some(x) => x.match_token(&TokenType::Eof),
        }
    }
    fn is_at_end(&self) -> bool {
        self.source.len() <= self.index
    }
    fn consume(&mut self, token_type: TokenType, error_type: ParserErrorType) -> Token<'b> {
        if self.is_at_end() || !self.match_withs(&[token_type]) {
            self.error(error_type);
            return Token::err_token();
        }
        self.previous_token()
    }

    fn statement(&mut self) -> Box<dyn Stmt<'b> + 'b> {
        //if self.match_withs(&[TokenType::Print]) {
        //    return self.print_statement();
        //}
        if self.match_withs(&[TokenType::If]) {
            return self.if_statement();
        }
        else if self.match_with(TokenType::While) {
            return self.while_statement();
        }
        else if self.match_withs(&[TokenType::LeftBrace]) {
            return self.block_statement();
        }
        else if self.match_with(TokenType::For) {
            return self.for_statement();
        }
        else if self.match_with(TokenType::Return){
            return self.return_statement();
        }
        self.expression_statement()
    }

    // fn print_statement(&mut self) -> Box<dyn Stmt<'b> + 'b> {
    //     let expr = self.expression();
    //     self.consume(TokenType::Semicolon, ParserErrorType::MissingSemicolon);
    //     Box::new(Print::new(expr))
    // }

    fn expression_statement(&mut self) -> Box<dyn Stmt<'b> + 'b> {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, ParserErrorType::MissingSemicolon);
        return Box::new(Expression::new(expr));
    }

    fn declaration(&mut self) -> Box<dyn Stmt<'b> + 'b> {
        if self.match_with(TokenType::Fun) {
            return self.function("function");
        }
        if self.match_withs(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Box<dyn Stmt<'b> + 'b> {
        let name = self.consume(TokenType::Identifier, ParserErrorType::InvalidAssignment);
        let initilizer = if self.match_withs(&[TokenType::Equal]) {
            self.expression()
        } else {
            Values::Null.into()
        };
        self.consume(TokenType::Semicolon, ParserErrorType::MissingSemicolon);
        Box::new(Var::new(name, initilizer))
    }

    fn error(&mut self, error_type: ParserErrorType) {
        self.errors
            .push(ParserError::new(self.current_token().unwrap(), error_type));
        // if error_type==ParserErrorType::MissingValue{
        //     self.advance();
        // }

        self.recovery();
    }

    fn get_errors(&mut self) -> Option<ParserErrors<'b>> {
        if self.errors.is_empty() {
            return None;
        }
        Some(take(&mut self.errors).into())
    }

    fn block_statement(&mut self) -> DynStmt<'b> {
        let mut statements: Vec<Box<dyn Stmt>> = Vec::new();
        while !self.checks(&[TokenType::RightBrace]) && !self.is_eof() {
            statements.push(self.declaration());
        }
        self.consume(TokenType::RightBrace, ParserErrorType::MissingRightBrace);
        Box::new(Block::from(statements))
    }

    fn checks(&self, token_types: &[TokenType]) -> bool {
        let Some(token) = self.current_token() else {
            return false;
        };
        return token.matches_token(token_types);
    }

    fn if_statement(&mut self) -> DynStmt<'b> {
        self.consume(TokenType::LeftParen, ParserErrorType::MissingLeftParen);
        let condition = self.expression();
        self.consume(TokenType::RightParen, ParserErrorType::MissingRightParen);
        let then_b = self.statement();
        let else_b = if self.match_withs(&[TokenType::Else]) {
            Some(self.statement())
        } else {
            None
        };
        return Box::new(If::new(condition, then_b, else_b));
    }

    fn match_with(&mut self, token_type: TokenType) -> bool {
        let token = match self.current_token() {
            Some(x) => x,
            None => return false,
        };
        if token.match_token(&token_type) {
            self.advance();
            return true;
        }
        return false;
    }

    fn and(&mut self) -> DynExpr<'b> {
        let mut expr = self.equility();
        while self.match_with(TokenType::And) {
            let operator = self.previous_token();
            let right = self.equility();
            expr = Box::new(Logical::new(expr, operator, right));
        }
        expr
    }

    fn while_statement(&mut self) -> DynStmt<'b> {
        self.consume(TokenType::LeftParen, ParserErrorType::MissingLeftParen);
        let condition = self.expression();
        self.consume(TokenType::RightParen, ParserErrorType::MissingRightParen);
        let body = self.statement();
        Box::new(WhileStmt::new(condition, body))
    }

    fn for_statement(&mut self) -> DynStmt<'b> {
        self.consume(TokenType::LeftParen, ParserErrorType::MissingLeftParen);
        let initializer = if self.match_with(TokenType::Semicolon) {
            None
        } else if self.match_with(TokenType::Var) {
            Some(self.var_declaration())
        } else {
            Some(self.expression_statement())
        };
        let condition = if self.check(TokenType::Semicolon) {
            Values::from(true).into()
        } else {
            self.expression()
        };
        self.consume(TokenType::Semicolon, ParserErrorType::MissingSemicolon);
        let increment = if self.check(TokenType::RightParen) {
            None
        } else {
            Some(self.expression())
        };
        self.consume(TokenType::RightParen, ParserErrorType::MissingRightParen);
        let stmt = self.statement();
        let mut output_body = Vec::new();
        if let Some(initializer) = initializer {
            output_body.push(initializer);
        }
        //Setting While loop
        let mut while_body = vec![stmt];
        if let Some(increment) = increment {
            let increment = Expression::new(increment);
            while_body.push(Box::new(increment));
        }
        let while_body = Box::new(Block::from(while_body));
        let while_body = WhileStmt::new(condition, while_body);
        output_body.push(Box::new(while_body));
        return Box::new(Block::from(output_body));
    }

    fn check(&self, token_type: TokenType) -> bool {
        let Some(ty) = self.current_token() else {
            return false;
        };
        ty.match_token(&token_type)
    }

    fn call(&mut self) -> DynExpr<'b> {
        let mut expr = self.primary();
        loop {
            if self.match_with(TokenType::LeftParen) {
                expr = self.finishcall(expr);
            } else {
                break;
            }
        }
        return expr;
    }

    fn finishcall(&mut self, callee: DynExpr<'b>) -> DynExpr<'b> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(self.expression());
                if !self.match_with(TokenType::Comma) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, ParserErrorType::MissingRightParen);
        return Box::new(CallExpr::new(callee, paren, arguments.into()));
    }

    fn recovery(&mut self) {
        use TokenType::*;
        loop {
            if self.checks(&[LeftBrace, While, Var, For]) || self.is_eof() {
                break;
            }
            if self.checks(&[Semicolon]) {
                self.advance();
                break;
            }
            self.advance();
        }
    }

    fn function(&mut self, name: &'static str) -> DynStmt<'b> {
        let name = self.consume(
            TokenType::Identifier,
            ParserErrorType::MissingIdentifier(name),
        );
        self.consume(TokenType::LeftParen, ParserErrorType::MissingLeftParen);
        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                parameters.push(self.consume(
                    TokenType::Identifier,
                    ParserErrorType::MissingIdentifier("function input parameter."),
                ));
                if !self.match_with(TokenType::Comma) {
                    break;
                }
            }
        }
        let parameter = parameters.into_boxed_slice();
        self.consume(TokenType::RightParen, ParserErrorType::MissingRightParen);
        self.consume(TokenType::LeftBrace, ParserErrorType::MissingLeftBrace);
        let body = self.block_statement();
        return Box::new(FunctionDelc::new(name, parameter, body));
    }

    fn return_statement(&mut self) -> DynStmt<'b> {
        let return_expr=self.expression();
        self.consume(TokenType::Semicolon, ParserErrorType::MissingSemicolon);
        Box::new(ReturnStmt::from(return_expr))
    }
}
