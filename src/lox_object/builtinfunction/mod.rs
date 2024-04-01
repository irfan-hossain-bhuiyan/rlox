use std::time::Instant;

use super::LoxCallable;
#[derive(Debug)]
pub struct PrintFunc;
impl<'a> LoxCallable<'a> for PrintFunc {
    fn call(
        &self,
        env: &mut crate::interpreter::environment::Environment,
        args: &[super::Values],
    ) -> Result<super::Values<'a>, Box<dyn std::error::Error>> {
        for x in args{
            env.write(&x.to_string())?;
            env.write(" ")?;
        }
        env.writeln("")?;
        Ok(super::Values::Null)
    }
    fn arity(&self,_args_num:usize) -> bool {
        return true;
    }
}
#[derive(Debug)]
pub struct ClockFunc(Instant);
impl ClockFunc{
    pub fn new()->Self{
        Self(Instant::now())
    }
}

impl<'a> LoxCallable<'a> for ClockFunc {
    fn call(
        &self,
        env: &mut crate::interpreter::environment::Environment,
        _args: &[super::Values],
    ) -> Result<super::Values<'a>, Box<dyn std::error::Error>> {
        env.writeln(format!("Time passed {} seconds",self.0.elapsed().as_secs_f64()).as_str())?;
        Ok(super::Values::Null)
    }
    fn arity(&self,args_num:usize) -> bool {
        args_num==0
    }
}
