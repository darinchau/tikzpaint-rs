use std::rc::Rc;
use std::fmt::Display;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;


/// An Rc wrapped over a string
pub struct CheapString {
    ptr: Rc<String>,
}

pub struct ThreadSafeCheapString {
    ptr: Arc<String>,
}

impl Display for CheapString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.ptr)
    }
}

impl Display for ThreadSafeCheapString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.ptr)
    }
}

impl Debug for CheapString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.ptr)
    }
}

impl Debug for ThreadSafeCheapString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.ptr)
    }
}

impl Clone for CheapString {
    fn clone(&self) -> Self {
        CheapString { ptr: Rc::clone(&self.ptr) }
    }
}

impl Clone for ThreadSafeCheapString {
    fn clone(&self) -> Self {
        ThreadSafeCheapString { ptr: Arc::clone(&self.ptr) }
    }
}


impl CheapString {
    pub fn new(s: String) -> Self {
        CheapString { ptr: Rc::new(s) }
    }
}

impl ThreadSafeCheapString {
    pub fn new(s: String) -> Self {
        ThreadSafeCheapString { ptr: Arc::new(s) }
    }
}

impl PartialEq for CheapString {
    fn eq(&self, other: &Self) -> bool {
        return *self.ptr == *other.ptr;
    }
}

impl PartialEq for ThreadSafeCheapString {
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

impl Deref for ThreadSafeCheapString {
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

    fn wrap_thread_safe(&self) -> ThreadSafeCheapString {
        ThreadSafeCheapString::new(format!("{self}"))
    }
}

impl StringLike for String {}

impl StringLike for &str {}

impl From<CheapString> for String {
    fn from(value: CheapString) -> Self {
        (&*value.ptr).clone()
    }
}

impl StringLike for CheapString {
    fn wrap(&self) -> CheapString {
        self.clone()
    }
}

impl StringLike for ThreadSafeCheapString {
    fn wrap_thread_safe(&self) -> ThreadSafeCheapString {
        self.clone()
    }
}
