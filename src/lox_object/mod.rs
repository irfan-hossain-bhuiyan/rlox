use core::fmt;
use std::{
    fmt::Display,
    result::Result,
};

use crate::interpreter::environment::Environment;
#[derive(Debug,Clone)]
pub enum Object{
    Value(Values),
    Var{name:String},
}
impl Object{
    pub fn into_value(&self,env:&Environment)->Result<Values,String>{
        let ans=match self{
            Self::Value(x)=>x,
            Self::Var { name }=>match env.get(name){
                Some(x)=>x,
                None=>return Err("Variable not found.".to_owned())
            },
        };
        Ok(ans.clone())
    }
}
impl Display for Object{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Value(x)=>Display::fmt(x, f),
            Object::Var { name }=>Display::fmt(name, f),
        }
    }
}
impl From<Values> for Object{
    fn from(value: Values) -> Self {
        Self::Value(value)
    }
}
#[derive(Debug, Clone,)]
pub enum Values {
    Str(String),
    Boolean(f64),
    Number(f64),
    Null
}
impl From<bool> for Values {
    fn from(value: bool) -> Self {
        Self::to_boolean(value)
    }
}
impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Values::*;
        match self {
            Str(x) => Display::fmt(x,f),
            Boolean(0.0) => write!(f, "false"),
            Boolean(_) => write!(f, "true"),
            Number(x) => write!(f, "{x}"),
            Null=>write!(f,"Null"),
        }
    }
}
impl Values {
    pub fn add(self, rhs: Self) -> Result<Values, String> {
        use Values::*;
        let ans = match (self, rhs) {
            (Number(x), Number(y)) => Number(x + y),
            (Number(x),Str(y))=>Str(x.to_string()+&y),
            (Number(x), Boolean(y)) => Number(x + y),
            (Boolean(x), Number(y)) => Number(x + y),
            (Boolean(x), Boolean(y)) => Number(x + y),
            (Str(x), Str(y)) => Str(x + &y),
            (Str(x),Number(y))=>Str(x+&y.to_string()),
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
    pub fn eq(&self, rhs: &Self) -> Values {
        use Values::*;
        match (self, rhs) {
            (Number(x), Number(y)) => x == y,
            (Boolean(x), Boolean(y)) => x == y,
            (Str(x), Str(y)) => x == y,
            (Null,Null)=>true,
            _ => false,
        }
        .into()
    }
    pub fn neq(&self, rhs: &Self) -> Values {
        use Values::*;
        match (self, rhs) {
            (Number(x), Number(y)) => x != y,
            (Boolean(x), Boolean(y)) => x != y,
            (Str(x), Str(y)) => x != y,
            (Null,Null)=>false,
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
    pub fn is_truthy(&self) -> Values {
        use Values::{Boolean, Number, Str,Null};
        match self {
            Boolean(x) | Number(x) => *x != 0.0,
            Str(x) => !x.is_empty(),
            Null=>false,
        }
        .into()
    }

    pub fn negative(&self) -> Result<Values, String> {
        use Values::*;
        let ans = match self {
            Number(x) => Number(-x),
            s => return Err(format!("Can't be negative of {}", s)),
        };
        Ok(ans)
    }
    pub fn not(&self) -> Result<Values, String> {
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

    pub fn to_boolean<T: Into<f64>>(x: T) -> Values {
        let x = x.into();
        if x == 0.0 {
            Self::Boolean(0.0)
        } else {
            Self::Boolean(1.0)
        }
    }
}

