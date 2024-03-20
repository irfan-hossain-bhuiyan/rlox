use std::{
    fmt::{Debug, Display},
    result::Result,
};

use crate::token::{Token, TokenType};
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ExprValue {
    Str(String),
    Boolean(f64),
    Number(f64),
}

impl From<bool> for ExprValue {
    fn from(value: bool) -> Self {
        Self::to_boolean(value)
    }
}
impl Display for ExprValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ExprValue::*;
        match self {
            Str(x) => write!(f, "\"{}\"", x),
            Boolean(0.0) => write!(f, "false"),
            Boolean(_) => write!(f, "true"),
            Number(x) => write!(f, "{x}"),
        }
    }
}
impl ExprValue {
    fn add(self, rhs: Self) -> Result<ExprValue, String> {
        use ExprValue::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => Number(x + y),
            (Number(x), Boolean(y)) => Number(x + y),
            (Boolean(x), Number(y)) => Number(x + y),
            (Boolean(x), Boolean(y)) => Number(x + y),
            (Str(x), Str(y)) => Str(x + &y),
            (s, r) => return Err(format!("Can't add {} and {}", s, r)),
        };
        Ok(ans)
    }
    fn sub(self, rhs: Self) -> Result<Self, String> {
        use ExprValue::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => Number(x - y),
            (Number(x), Boolean(y)) => Number(x - y),
            (Boolean(x), Number(y)) => Number(x - y),
            (Boolean(x), Boolean(y)) => Number(x - y),
            (s, r) => return Err(format!("Can't sub {} and {}", s, r)),
        };
        Ok(ans)
    }
    fn mul(self, rhs: Self) -> Result<Self, String> {
        use ExprValue::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => Number(x * y),
            (Number(x), Boolean(y)) => Number(x * y),
            (Boolean(x), Number(y)) => Number(x * y),
            (Boolean(x), Boolean(y)) => Number(x * y),
            (s, r) => return Err(format!("Can't mul {} and {}", s, r)),
        };
        Ok(ans)
    }
    fn div(self, rhs: Self) -> Result<Self, String> {
        use ExprValue::*;
        let ans = match (self, rhs) {
            (_, Number(0.0)) => {
                return Err("Division by zero".to_owned());
            }
            (Number(x), Number(y)) => Number(x / y),
            (s, r) => return Err(format!("Can't divide {} and {}", s, r)),
        };
        Ok(ans)
    }
    fn eq(&self, rhs: &Self) -> ExprValue {
        use ExprValue::*;
        match (self, rhs) {
            (Number(x), Number(y)) => x == y,
            (Boolean(x), Boolean(y)) => x == y,
            (Str(x), Str(y)) => x == y,
            _ => false,
        }
        .into()
    }
    fn neq(&self, rhs: &Self) -> ExprValue {
        use ExprValue::*;
        match (self, rhs) {
            (Number(x), Number(y)) => x != y,
            (Boolean(x), Boolean(y)) => x != y,
            (Str(x), Str(y)) => x != y,
            _ => true,
        }
        .into()
    }
    fn greater(&self, rhs: &Self) -> Result<Self, String> {
        use ExprValue::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => x > y,
            (Boolean(x), Boolean(y)) => x > y,
            (Str(x), Str(y)) => x > y,
            (s, r) => return Err(format!("Can't compare {} and {}", s, r)),
        };
        Ok(ans.into())
    }
    fn greater_equal(&self, rhs: &Self) -> Result<Self, String> {
        use ExprValue::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => x >= y,
            (Boolean(x), Boolean(y)) => x >= y,
            (Str(x), Str(y)) => x >= y,
            (s, r) => return Err(format!("Can't compare {} and {}", s, r)),
        };
        Ok(ans.into())
    }
    fn less_equal(&self, rhs: &Self) -> Result<Self, String> {
        use ExprValue::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => x <= y,
            (Boolean(x), Boolean(y)) => x <= y,
            (Str(x), Str(y)) => x <= y,
            (s, r) => return Err(format!("Can't compare {} and {}", s, r)),
        };
        Ok(ans.into())
    }
    fn less(&self, rhs: &Self) -> Result<Self, String> {
        use ExprValue::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => x < y,
            (Boolean(x), Boolean(y)) => x < y,
            (Str(x), Str(y)) => x < y,
            (s, r) => return Err(format!("Can't compare {} and {}", s, r)),
        };
        Ok(ans.into())
    }
    fn is_truthy(&self) -> ExprValue {
        use ExprValue::{Boolean, Number, Str};
        match self {
            Boolean(x) | Number(x) => *x != 0.0,
            Str(x) => !x.is_empty(),
        }
        .into()
    }

    fn negative(&self) -> Result<ExprValue, String> {
        use ExprValue::*;
        let ans = match self {
            Number(x) => Number(-x),
            s => return Err(format!("Can't be negative of {}", s)),
        };
        Ok(ans)
    }
    fn not(&self) -> Result<ExprValue, String> {
        use ExprValue::*;
        if let Boolean(x) = self {
            let boolean_value = match x {
                0.0 => 1.0,
                _ => 0.0,
            };
            Ok(Boolean(boolean_value))
        } else {
            Err(format!("It only work for boolean type.Not for {}", self))
        }
    }

    fn to_boolean<T: Into<f64>>(x: T) -> ExprValue {
        let x = x.into();
        if x == 0.0 {
            Self::Boolean(0.0)
        } else {
            Self::Boolean(1.0)
        }
    }
}
pub trait Expr: Debug + Display {
    fn evaluate(&self) -> Result<ExprValue, String>;
}
#[derive(Debug)]
pub struct BinaryOp<'a> {
    left: Box<dyn Expr + 'a>,
    operator: Token<'a>,
    right: Box<dyn Expr + 'a>,
}

impl<'a> BinaryOp<'a> {
    pub fn new(left: Box<dyn Expr + 'a>, operator: Token<'a>, right: Box<dyn Expr + 'a>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
#[derive(Debug)]
pub struct Grouping<'a> {
    expression: Box<dyn Expr + 'a>,
}

impl<'a> Grouping<'a> {
    pub fn new(expression: Box<dyn Expr + 'a>) -> Self {
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
    right: Box<dyn Expr + 'a>,
}

impl<'a> Unary<'a> {
    pub fn new(operator: Token<'a>, right: Box<dyn Expr + 'a>) -> Self {
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

impl Expr for BinaryOp<'_> {
    fn evaluate(&self) -> Result<ExprValue, String> {
        let left = self.left.evaluate()?;
        let right = self.right.evaluate()?;
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
            _=>Err("mismatched type sin binary operation.".into()),
        }

    }
}
impl Expr for Grouping<'_> {
    fn evaluate(&self) -> Result<ExprValue, String> {
        self.expression.evaluate()
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
impl Expr for Literal<'_> {
    fn evaluate(&self) -> Result<ExprValue, String> {
        use TokenType::{False, Number, String, True};
        let ans=match self.token_type() {
            String => ExprValue::Str(self.to_string()),
            Number => ExprValue::Number(self.token.as_str().parse().unwrap()),
            True => ExprValue::Boolean(1.0),
            False => ExprValue::Boolean(0.0),
            _ => return Err("Unexpected value,wanted boolean,number or string".into()),
        };
        Ok(ans)
    }
}
impl Display for Unary<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! {f,"({}{})",self.operator.to_string(),self.right.to_string()}
    }
}
impl Expr for Unary<'_> {
    fn evaluate(&self) -> Result<ExprValue, String> {
        let right = self.right.evaluate()?;
        use TokenType::*;
        match self.operator.get_type() {
            Minus => right.negative(),
            Bang => right.is_truthy().not(),
            _ =>Err("Other operator is not allowed".into()),
        }
    }
}
