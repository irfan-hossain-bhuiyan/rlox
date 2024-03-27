use std::{error::Error, fmt::Display, mem::take};

use ascii::{AsAsciiStr, AsciiChar, AsciiStr};

use crate::lox_error::Errors;
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    TokenError,
    // End of file
    Eof,
}
impl TokenType {
    fn keyword(st: &str) -> Self {
        match st {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }
}
#[derive(Clone, Debug, Copy)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a AsciiStr,
    line: usize,
    coloum: usize,
}
impl<'a> Token<'a> {
    fn new(token_type: TokenType, lexeme: &'a AsciiStr, line: usize, coloum: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            coloum,
        }
    }
    pub fn err_token()->Self{
        Self{
            token_type:TokenType::TokenError,
            lexeme:unsafe{"".as_ascii_str_unchecked()},
            line:0,
            coloum:0,
        }
    }
    pub fn get_type(&self) -> TokenType {
        self.token_type
    }
    pub fn match_token(&self, token_type: &TokenType) -> bool {
        self.token_type == *token_type
    }
    pub fn matches_token(&self, token_types: &[TokenType]) -> bool {
        for x in token_types.iter() {
            if *x == self.token_type {
                return true;
            }
        }
        false
    }
    pub fn as_str(self) -> &'a str {
        self.lexeme.as_str()
    }

    pub(crate) fn span(&self) -> String {
        format!("{}:{}", self.line, self.coloum)
    }
}
impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme.to_string())
    }
}
#[derive(Debug, Clone, Copy)]
enum TokenizationErrorType {
    UnIdentifiedLetter(char),
    UnFinishedString,
}
impl TokenizationErrorType {
    fn as_str(&self) -> String {
        match self {
           Self::UnIdentifiedLetter(x)=>format!("Character not identified:{}",x),
           Self::UnFinishedString=>format!("You didn't end the string."),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct TokenizationError {
    line: usize,
    coloum: usize,
    error_type: TokenizationErrorType,
}
impl Display for TokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line/coloum:{}:{}\n  Error:{}",
            self.line,
            self.coloum,
            self.error_type.as_str()
        )
    }
}
impl Error for TokenizationError{}
type Tokens<'a> = Vec<Token<'a>>;
pub type TokenizationErrors = Errors<TokenizationError>;
#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    source: &'a AsciiStr,
    tokens: Vec<Token<'a>>,
    line: usize,
    coloum: usize,
    current: usize,
    start: usize,
    errors: Vec<TokenizationError>,
}
type Result<'a>=std::result::Result<Tokens<'a>,TokenizationErrors>;
impl<'a> Scanner<'a> {
    pub fn new(source: &'a AsciiStr) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 0,
            coloum: 0,
            errors: Vec::new(),
        }
    }
    pub fn scan_tokens(mut self) -> Result<'a> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            AsciiStr::from_ascii(b"").unwrap(),
            self.line,
            self.coloum,
        ));
        match self.get_error(){
            Some(x)=>Err(x),
            None=>Ok(self.tokens),
        }
    }
    fn throw_error(&mut self, error_type: TokenizationErrorType) {
        let error = TokenizationError {
            line: self.line,
            coloum: self.coloum,
            error_type,
        };
        self.errors.push(error);
    }
    fn get_error(&mut self)->Option<TokenizationErrors>{
        if self.errors.is_empty(){
            return None;
        }
        Some(take(&mut self.errors).into())
    }
    fn is_at_end(&self) -> bool {
        self.source.len() <= self.current
    }
    fn scan_token(&mut self) {
        let c: char = self.advance().into();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = {
                    if self.match_later('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                };
                self.add_token(token_type)
            }
            '=' => {
                let token_type = if self.match_later('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }

            '<' => {
                let token_type = if self.match_later('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }

            '>' => {
                let token_type = if self.match_later('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                //cahecking for comments
                if self.match_later('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            //Tabs
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.token_line(), // string,literal,number
            '"' => self.token_string(),
            x => {
                if c.is_digit(10) {
                    self.token_digit();
                } else if c.is_ascii_alphabetic() {
                    self.token_identifier();
                } else {
                    self.throw_error(TokenizationErrorType::UnIdentifiedLetter(x));
                }
            }
        }
    }
    fn token_string(&mut self) {
        self.ignore_front();
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.throw_error(TokenizationErrorType::UnFinishedString);
        }
        self.add_token(TokenType::String);
        self.advance();
    }
    fn token_identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        self.add_token(TokenType::keyword(
            self.source[self.start..self.current].as_str(),
        ));
    }

    fn advance(&mut self) -> AsciiChar {
        let ans = self.source[self.current];
        self.current += 1;
        self.coloum += 1;
        ans
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            &self.source[self.start..self.current],
            self.line,
            self.coloum,
        ));
    }
    fn match_later(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != ch {
            return false;
        }
        self.current += 1;
        true
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current].into()
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1].into()
    }
    fn token_digit(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        self.add_token(TokenType::Number);
    }

    fn token_line(&mut self) {
        self.coloum = 0;
        self.line += 1;
    }

    fn ignore_front(&mut self) {
        self.start+=1;//This is to remove the quote from the string.
    }
}
