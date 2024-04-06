use std::cell::RefMut;

use rlox::basic_function::RcRef;

type AryRef=RcRef<Box<[i32]>>;
fn first_mut_ptr(array:&AryRef) -> RefMut<'_, i32>   {
    RefMut::map(array.brw_mut(),|x|&mut x[0])
}
fn main(){
    print!("Running rcRef_test.rs");
    let owned=AryRef::new(vec![1,2,3,4,5,6,7,8].into_boxed_slice());
    let clone1=owned.clone();
    let clone2=owned.clone();
    *first_mut_ptr(&clone1)=10;
    println!("{:?}",clone2.brw());
}
