use std::collections::HashMap;

use crate::lox_object::Values;

#[derive(Debug,Default)]
pub struct Environment{
    values:HashMap<String,Values>,
}
impl Environment{
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
        self.contains(name)
    }
}
