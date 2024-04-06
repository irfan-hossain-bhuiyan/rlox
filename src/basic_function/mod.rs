use std::{fmt::Display, io};

pub fn to_string(x: Result<impl Display, impl Display>) -> String {
    match x {
        Ok(x) => x.to_string(),
        Err(x) => format!("Error:{}", x),
    }
}
pub fn print(x: Result<impl Display, impl Display>) {
    println!("{}", to_string(x));
}
pub fn repl(mut f: impl FnMut(&str)) {
    loop {
    let mut x: String = String::new();
         if let Err(x)=io::stdin().read_line(&mut x){
             eprintln!("{}",x);
             continue;
         }
         if x.as_str()=="q"{return;}
         f(&x);
    }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

/// A reference-counted, mutable memory location.
#[derive(Debug)]
pub struct RcRef<T> {
    inner: Rc<RefCell<T>>,
}
impl<T:Default> Default for RcRef<T>{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> RcRef<T> {
    /// Creates a new `RcRef` wrapping a `RefCell` containing `value`.
    pub fn new(value: T) -> Self {
        RcRef {
            inner: Rc::new(RefCell::new(value)),
        }
    }

    /// Immutably borrows the wrapped value.
    pub fn brw(&self) -> std::cell::Ref<'_, T> {
        self.inner.borrow()
    }

    /// Mutably borrows the wrapped value.
    pub fn brw_mut(&self) -> std::cell::RefMut<'_, T> {
        self.inner.borrow_mut()
    }

    /// Returns a reference to the underlying `Rc<RefCell<T>>`.
    pub fn inner(&self) -> &Rc<RefCell<T>> {
        &self.inner
    }

    /// Returns a clone of the underlying `Rc<RefCell<T>>`.
    pub fn clone_inner(&self) -> Rc<RefCell<T>> {
        self.inner.clone()
    }
}

impl<T> Clone for RcRef<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        RcRef {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for RcRef<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for RcRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::get_mut(&mut self.inner).unwrap()
    }
}
