use std::{error::Error, fmt::{Debug, Display}};
pub fn emit_error(error:&dyn Error){
    eprintln!("{}",error);
}
#[derive(Debug,Clone)]
pub struct Errors<Err:Error>(Box<[Err]>);
impl<Err:Error> Default for Errors<Err>{
    fn default() -> Self {
        Vec::new().into()
    }
}   
impl<Err:Error> From<Vec<Err>> for Errors<Err>{
    fn from(value: Vec<Err>) -> Self {
        Errors(value.into_boxed_slice())
    }
}
impl<Err:Error> Display for Errors<Err>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.0.iter(){
            write!(f,"{}\n",x)?
        }
        Ok(())
    }
}

impl<Err:Error> Error for Errors<Err>{}
