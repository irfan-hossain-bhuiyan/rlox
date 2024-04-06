fn fibbo(n:u32)->u32{
    if n==0 || n==1{return n;}
    fibbo(n-1)+fibbo(n-2)
}
fn main(){
    println!("{}",fibbo(40));
}
