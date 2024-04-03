use std::{
    error::Error,
    fmt::{Debug, Display},
    rc::Rc,
    result::Result,
};
pub mod builtinfunction;

use crate::{
    ast::statement::{self, DynStmt, FunctionDelc, Stmt},
    interpreter::environment::Environment,
    token::Token,
};
//#[derive(Debug,Clone)]
//pub enum Object<'a>{
//    Value(Values<'_><'a>),
//    Var{name:String},
//}
//impl<'a> Object<'a>{
//    pub fn into_value(&self,env:&Environment<'a>)->Result<Values<'_><'a>,String>{
//        let ans=match self{
//            Self::Value(x)=>x,
//            Self::Var { name }=>match env.get(name){
//                Some(x)=>x,
//                None=>return Err("Variable not found.".to_owned())
//            },
//        };
//        Ok(ans.clone())
//    }
//}
//impl Display for Object<'_>{
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        match self {
//            Object::Value(x)=>Display::fmt(x, f),
//            Object::Var { name }=>Display::fmt(name, f),
//        }
//    }
//}
//impl<'a> From<Values<'_><'a>> for Object<'a>{
//    fn from(value: Values<'_><'a>) -> Self {
//        Self::Value(value)
//    }
//}
pub trait LoxCallable<'a>: Debug {
    fn call(
        &self,
        env: &mut Environment<'a>,
        args: &[Values<'a>],
    ) -> Result<Values<'a>, Box<dyn Error>>;
    fn arity(&self, input_number: usize) -> bool;
}
#[derive(Debug, Clone)]
pub enum Values<'a> {
    Str(String),
    Boolean(f64),
    Number(f64),
    Fn(Rc<dyn LoxCallable<'a> + 'a>),
    Null,
}
impl From<bool> for Values<'_> {
    fn from(value: bool) -> Self {
        Self::to_boolean(value)
    }
}
impl Display for Values<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Values::*;
        match self {
            Str(x) => Display::fmt(x, f),
            Boolean(0.0) => write!(f, "false"),
            Boolean(_) => write!(f, "true"),
            Number(x) => write!(f, "{x}"),
            Null => write!(f, "Null"),
            Fn(_) => write!(f, "A function"),
        }
    }
}
#[derive(Debug)]
pub struct LoxFunction<'a> {
    name: Token<'a>,
    paren: Box<[Token<'a>]>,
    body: DynStmt<'a>,
}
impl<'a> LoxFunction<'a> {
    pub fn new(name: Token<'a>, paren: Box<[Token<'a>]>, body: DynStmt<'a>) -> Self {
        Self { name, paren, body }
    }

    fn set_arguments(&self, env: &mut Environment<'a>, args: &[Values<'a>]) {
        for (paran, args) in self.paren.iter().zip(args) {
            env.define(paran.to_string(), args.clone());
        }
    }

    pub fn name(&self) -> &'a str {
        self.name.as_str()
    }
}
impl<'a> LoxCallable<'a> for LoxFunction<'a> {
    fn call(
        &self,
        env: &mut Environment<'a>,
        args: &[Values<'a>],
    ) -> Result<Values<'a>, Box<dyn Error>> {
        env.create_sub_values();
        self.set_arguments(env, args);
        let return_value = self.body.execute(env)?;
        env.delete_sub_values();
        if let Some(return_value) = return_value {
            return Ok(return_value);
        }
        return Ok(Values::Null);
    }
    fn arity(&self, input_number: usize) -> bool {
        input_number == self.paren.len()
    }
}
impl Values<'_> {
    pub fn add(self, rhs: Self) -> Result<Self, String> {
        use Values::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => Number(x + y),
            (Number(x), Str(y)) => Str(x.to_string() + &y),
            (Number(x), Boolean(y)) => Number(x + y),
            (Boolean(x), Number(y)) => Number(x + y),
            (Boolean(x), Boolean(y)) => Number(x + y),
            (Str(x), Str(y)) => Str(x + &y),
            (Str(x), Number(y)) => Str(x + &y.to_string()),
            (s, r) => return Err(format!("Can't add {} and {}", s, r)),
        };
        Ok(ans)
    }
    pub fn sub(self, rhs: Self) -> Result<Self, String> {
        use Values::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => Number(x - y),
            (Number(x), Boolean(y)) => Number(x - y),
            (Boolean(x), Number(y)) => Number(x - y),
            (Boolean(x), Boolean(y)) => Number(x - y),
            (s, r) => return Err(format!("Can't sub {} and {}", s, r)),
        };
        Ok(ans)
    }
    pub fn mul(self, rhs: Self) -> Result<Self, String> {
        use Values::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => Number(x * y),
            (Number(x), Boolean(y)) => Number(x * y),
            (Boolean(x), Number(y)) => Number(x * y),
            (Boolean(x), Boolean(y)) => Number(x * y),
            (s, r) => return Err(format!("Can't mul {} and {}", s, r)),
        };
        Ok(ans)
    }
    pub fn div(self, rhs: Self) -> Result<Self, String> {
        use Values::*;
        let ans = match (self, rhs) {
            (_, Number(0.0)) => {
                return Err("Division by zero".to_owned());
            }
            (Number(x), Number(y)) => Number(x / y),
            (s, r) => return Err(format!("Can't divide {} and {}", s, r)),
        };
        Ok(ans)
    }
    pub fn eq(&self, rhs: &Self) -> Self {
        use Values::*;
        match (self, rhs) {
            (Number(x), Number(y)) => x == y,
            (Boolean(x), Boolean(y)) => x == y,
            (Str(x), Str(y)) => x == y,
            (Null, Null) => true,
            _ => false,
        }
        .into()
    }
    pub fn neq(&self, rhs: &Self) -> Self {
        use Values::*;
        match (self, rhs) {
            (Number(x), Number(y)) => x != y,
            (Boolean(x), Boolean(y)) => x != y,
            (Str(x), Str(y)) => x != y,
            (Null, Null) => false,
            _ => true,
        }
        .into()
    }
    pub fn greater(&self, rhs: &Self) -> Result<Self, String> {
        use Values::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => x > y,
            (Boolean(x), Boolean(y)) => x > y,
            (Str(x), Str(y)) => x > y,
            (s, r) => return Err(format!("Can't compare {} and {}", s, r)),
        };
        Ok(ans.into())
    }
    pub fn greater_equal(&self, rhs: &Self) -> Result<Self, String> {
        use Values::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => x >= y,
            (Boolean(x), Boolean(y)) => x >= y,
            (Str(x), Str(y)) => x >= y,
            (s, r) => return Err(format!("Can't compare {} and {}", s, r)),
        };
        Ok(ans.into())
    }
    pub fn less_equal(&self, rhs: &Self) -> Result<Self, String> {
        use Values::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => x <= y,
            (Boolean(x), Boolean(y)) => x <= y,
            (Str(x), Str(y)) => x <= y,
            (s, r) => return Err(format!("Can't compare {} and {}", s, r)),
        };
        Ok(ans.into())
    }
    pub fn less(&self, rhs: &Self) -> Result<Self, String> {
        use Values::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => x < y,
            (Boolean(x), Boolean(y)) => x < y,
            (Str(x), Str(y)) => x < y,
            (s, r) => return Err(format!("Can't compare {} and {}", s, r)),
        };
        Ok(ans.into())
    }
    pub fn is_truthy(&self) -> bool {
        use Values::{Boolean, Fn, Null, Number, Str};
        match self {
            Boolean(x) | Number(x) => *x != 0.0,
            Str(x) => !x.is_empty(),
            Null => false,
            Fn(_) => true,
        }
    }
    ///Cast a lox value to lox boolean value
    pub fn cast_to_boolean(&self) -> Self {
        self.is_truthy().into()
    }

    pub fn negative(&self) -> Result<Self, String> {
        use Values::*;
        let ans = match self {
            Number(x) => Number(-x),
            s => return Err(format!("Can't be negative of {}", s)),
        };
        Ok(ans)
    }
    pub fn not(&self) -> Result<Self, String> {
        use Values::*;
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

    pub fn to_boolean<T: Into<f64>>(x: T) -> Self {
        let x = x.into();
        if x == 0.0 {
            Self::Boolean(0.0)
        } else {
            Self::Boolean(1.0)
        }
    }
}
