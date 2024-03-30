use std::{collections::{HashMap, VecDeque}, fmt::Debug, io::Write};

use crate::lox_object::Values;

pub struct Environment<'a> {
    values: VecDeque<HashMap<String, Values>>,
    stdout: &'a mut dyn Write,
}

impl Debug for Environment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Environment{{values:{:?}}}", self.values)
    }
}
impl<'a> Environment<'a> {
    pub(super) fn new(stdout: &'a mut dyn Write) -> Self {
        Self {
            values: VecDeque::new(),
            stdout,
        }
    }
    pub fn create_sub_values(&mut self){
        self.values.push_front(HashMap::new());
    }
    pub fn delete_sub_values(&mut self){
        self.values.pop_front();
    }
   pub fn get(&self, name: &str) -> Option<& Values> {
       for x in self.values.iter(){
            let value=x.get(name);
            if value.is_some(){return value;}
       }
       None
    }
   fn get_mut(&mut self,name:&str)->Option<&mut Values>{
        for x in self.values.iter_mut(){
            let value=x.get_mut(name);
            if value.is_some(){return value;}
        }
        None
   }
    pub fn define(&mut self, name: String, value: Values) {
        self.current_block_mut().insert(name,value);
    }
    pub fn redefine(&mut self, name: &str, value: Values) -> Result<(), String> {
        match self.get_mut(name){
            Some(x)=>*x=value,
            None=>return Err(format!("variable {} is not defined",name))
        }
        Ok(())
    }
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
    pub fn writeln(&mut self, output: &str) -> Result<(), std::io::Error> {
        self.stdout.write_all(output.as_bytes())?;
        self.stdout.write_all(b"\n")
    }

    fn current_block(&self) -> &HashMap<String, Values>  {
        self.values.front().unwrap()
    }
    fn current_block_mut(&mut self)->&mut HashMap<String,Values>{
        self.values.front_mut().unwrap()
    }
}
#[macro_export]
macro_rules! write_env {
    ($name:expr,$($t:tt)*) => {
        $name.write(& format!($($t)*))
    };
}
