use std::rc::Rc;
use std::fmt::Display;
use std::fmt::Debug;
use std::ops::Deref;


/// An Rc wrapped over a string
pub struct CheapString {
    ptr: Rc<String>,
}

impl Display for CheapString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.ptr)
    }
}

impl Debug for CheapString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.ptr)
    }
}

impl Clone for CheapString {
    fn clone(&self) -> Self {
        CheapString { ptr: Rc::clone(&self.ptr) }
    }
}

impl CheapString {
    pub fn new(s: String) -> Self {
        CheapString { ptr: Rc::new(s) }
    }
}

impl PartialEq for CheapString {
    fn eq(&self, other: &Self) -> bool {
        return *self.ptr == *other.ptr;
    }
}

impl Deref for CheapString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        return &*self.ptr;
    }
}

/// This means string and cheapstring
pub trait StringLike: Debug + Display + Clone + PartialEq {
    fn wrap(&self) -> CheapString {
        CheapString::new(format!("{self}"))
    }
}

impl StringLike for String {}
impl StringLike for &str {}
impl StringLike for CheapString {
    fn wrap(&self) -> CheapString {
        self.clone()
    }
}
