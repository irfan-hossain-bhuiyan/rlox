struct R<'a>{
    sor:&'a str,
    v:Vec<&'a str>
}
impl<'a> R<'a>{
    fn new(r:&'a str)->Self{
        R{sor:r,v:vec![r]}
    }
    fn r(self)->Vec<&'a str>{
        self.v
    }
}
fn main() {
    let x=String::from("It is working");
    let o=R::new(&x);
    let v=o.r();
    println!("{:?}",v);
}
