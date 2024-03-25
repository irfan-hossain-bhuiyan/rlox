use std::{collections::HashMap, fmt::Debug, io::Write};

use crate::lox_object::Values;

pub struct Environment<'a>{
    values:HashMap<String,Values>,
    stdout:&'a mut dyn Write,    
}

impl Debug for Environment<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Environment{{values:{:?}}}",self.values)
    }
}
impl<'a> Environment<'a>{
    pub(super) fn new(stdout:&'a mut dyn Write)->Self{
        Self{
            values:HashMap::new(),
            stdout,
        }
    }
    pub fn get(&self,name:&str)->Option<&Values>{
        self.values.get(name)
    }
    pub fn define(&mut self,name:String,value:Values){
        self.values.insert(name, value);
    }
    pub fn redefine(&mut self,name:&str,value:Values)->Result<(),String>{
        match self.values.get_mut(name){
            Some(x)=>{*x=value;Ok(())},
            None=>Err("Variable not found".to_string()),
        }
    }
    pub fn contains(&self,name:&str)->bool{
        self.values.contains_key(name)
    }
    pub fn writeln(&mut self, output:&str) -> Result<(), std::io::Error> {
        writeln!(self.stdout,"{}",output)
    }
}
#[macro_export]
macro_rules! write_env {
    ($name:expr,$($t:tt)*) => {
        $name.write(& format!($($t)*))
    };
}
