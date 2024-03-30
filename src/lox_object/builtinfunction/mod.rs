use std::time::Instant;

use super::LoxFunc;
#[derive(Debug)]
pub struct PrintFunc;
impl LoxFunc for PrintFunc {
    fn call(
        &self,
        env: &mut crate::interpreter::environment::Environment,
        args: &[super::Values],
    ) -> Result<super::Values, Box<dyn std::error::Error>> {
        env.writeln(&args[0].to_string())?;
        Ok(super::Values::Null)
    }
    fn arity(&self) -> usize {
        return 1;
    }
}
#[derive(Debug)]
pub struct ClockFunc(Instant);
impl ClockFunc{
    pub fn new()->Self{
        Self(Instant::now())
    }
}

impl LoxFunc for ClockFunc {
    fn call(
        &self,
        env: &mut crate::interpreter::environment::Environment,
        _args: &[super::Values],
    ) -> Result<super::Values, Box<dyn std::error::Error>> {
        env.writeln(format!("Time passed {} seconds",self.0.elapsed().as_secs_f64()).as_str())?;
        Ok(super::Values::Null)
    }
    fn arity(&self) -> usize {
        return 0;
    }
}
