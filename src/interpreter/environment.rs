use std::{
    collections::HashMap,
    fmt::Debug,
    io::Write,
    mem,
    rc::Rc,
};

use crate::{
    basic_function::RcRef,
    lox_object::{
        builtinfunction::{ClockFunc, PrintFunc},
        Values,
    },
};

#[derive(Default, Debug, Clone)]
pub struct Scopedata<'a> {
    values: HashMap<String, Values<'a>>,
    parent: Option<Scope<'a>>,
}
#[derive(Debug, Clone, Default)]
pub struct Scope<'a>(RcRef<Scopedata<'a>>);

impl<'a> Scope<'a> {
    fn create_sub_values(&mut self) {
        self.0.brw_mut().parent = Some(mem::take(self));
    }
    fn delete_sub_values(&mut self) {
        *self = self.0.clone().brw().parent.clone().unwrap();
    }
    pub fn get(&self, key: &str) -> Option<Values<'a>> {
        let brw = self.0.brw();
        match brw.values.get(key) {
            Some(x) => Some(x.clone()),
            None => match &brw.parent {
                Some(x) => x.get(key),
                None => None,
            },
        }
    }

    pub fn define(&mut self, key: String, value: Values<'a>) {
        self.0.brw_mut().values.insert(key, value);
    }
    pub fn redefine(&mut self, key: &str, value: Values<'a>) -> Result<(), String> {
        let mut brw_mut=self.0.brw_mut();
        if  let Some(x)=brw_mut.values.get_mut(key){
            *x=value;
        }
        else if let Some(x)=brw_mut.parent.as_mut(){
            x.redefine(key,value)?;
        }
        else{return Err(format!("Variable {} not defined",key));}
        Ok(())
    }
    pub fn contain(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
}
pub struct Environment<'a> {
    scope: Scope<'a>,
    stdout: &'a mut dyn Write,
}
impl Debug for Environment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Environment{{values:{:?}}}", self.scope)
    }
}
impl<'a> Environment<'a> {
    pub fn global_env(stdout: &'a mut dyn Write) -> Self {
        let mut output = Self {
            scope: Scope::default(),
            stdout,
        };
        output.include_globals();
        output
    }
    pub fn create_sub_values(&mut self) {
        self.scope.create_sub_values();
    }
    pub fn delete_sub_values(&mut self) {
        self.scope.delete_sub_values();
    }

    pub fn define(&mut self, name: String, value: Values<'a>) {
        self.scope.define(name, value);
    }
    pub fn redefine(&mut self, name: &str, value: Values<'a>) -> Result<(), String> {
        self.scope.redefine(name, value)
    }
    pub fn get_current(&self) -> Scope<'a> {
        self.scope.clone()
    }
    pub fn write(&mut self, output: &str) -> Result<(), std::io::Error> {
        self.stdout.write_all(output.as_bytes())?;
        Ok(())
    }
    pub fn writeln(&mut self, output: &str) -> Result<(), std::io::Error> {
        self.stdout.write_all(output.as_bytes())?;
        self.stdout.write_all(b"\n")?;
        Ok(())
    }

    fn include_globals(&mut self) {
        self.create_sub_values();
        self.define("print".to_owned(), Values::<'a>::Fn(Rc::new(PrintFunc)));
        let clock_timer = ClockFunc::new();
        self.define("clock".to_owned(), Values::Fn(Rc::new(clock_timer)));
    }

    pub fn get(&self, key: &str) -> Option<Values<'a>> {
        self.scope.get(key)
    }

    pub fn set_scope(&mut self, values: Scope<'a>) {
        self.scope = values;
    }
}
