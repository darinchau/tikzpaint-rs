use std::sync::atomic::{AtomicUsize, Ordering};
use std::rc::Rc;
use std::fmt::Display;
use std::fmt::Debug;

pub fn is_true(x: Option<bool>) -> bool {
    x.is_some() && x.unwrap()
}

pub fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

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
