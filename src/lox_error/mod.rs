use std::error::Error;
#[derive(Default,Debug)]
struct ComptimeErrors{
    errors:Vec<Box<dyn Error>>,
}
impl ComptimeErrors{
    fn add(&mut self,error:Box<dyn Error>){
        self.errors.push(error);
    }
    fn emit_error(&self){
        for x in self.errors.iter(){
            eprintln!("{}",x);
        }
    }
}
pub fn emit_runtime_error(error:&dyn Error){
    eprintln!("{}",error);
}
