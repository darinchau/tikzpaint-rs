use std::sync::atomic::{AtomicUsize, Ordering};
use std::rc::Rc;
use std::fmt::Display;
use std::fmt::Debug;

use yew::prelude::*;
use wasm_bindgen::prelude::*;

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

/// Returns a Ok(String) if the conversion is successful, and returns the inside value
/// Otherwise return the raw JsValue according to Debug in wasm
pub fn jsvalue_to_string(jsvalue: JsValue) -> Result<String, String> {
    // I hate to do this but the only way I can think is to print it out
    // Say it with me: Rust wasm still has skimpy documentation

    let x = format!("{:?}", jsvalue);
    if x.starts_with("JsValue(") {
        // Trim JsValue away, also trim ending bracket
        return Ok(String::from(&x[8..x.len()-1]));
    }
    else{
        return Err(x);
    }
}