use std::error::Error;
pub fn emit_error(error:&dyn Error){
    eprintln!("{}",error);
}
pub fn emit_errors(errors:&[impl Error]){
    for x in errors{
        eprintln!("{}",x);
    }
}
