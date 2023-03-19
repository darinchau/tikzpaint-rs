//! A scoped vector is a wrapper around a shallow-copied vector with various helper methods
//! If this vector is closed, then an iter loops through the whole thing
//! but if this is not closed, only loop through the new elements
use std::sync::{Arc, Mutex};
use std::fmt::Debug;

pub struct ScopedVec<T: Copy + Debug> {
    ptr: Arc<Mutex<ScopedVecInner<T>>>
}

impl<T: Copy + Debug> ScopedVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: Arc::new(Mutex::new(ScopedVecInner::new()))
        }
    }

    pub fn push(&mut self, t: T) {
        let mut inner = self.ptr.lock().unwrap();
        inner.push(t);
    }

    pub fn close(&mut self) {
        let mut inner = self.ptr.lock().unwrap();
        inner.close();
    }

    pub fn iter(&self) -> ScopedVecIterator<T> {
        let mut inner = self.ptr.lock().unwrap();
        inner.iter()
    }

    pub fn shallow_copy(&self) -> Self {
        Self {
            ptr: Arc::clone(&self.ptr)
        }
    }
}

#[derive(Debug)]
struct ScopedVecInner<T: Copy + Debug> {
    obj: Vec<T>,
    new: Vec<T>,
    closed: bool,
}

impl<T: Copy + Debug> ScopedVecInner<T> {
    pub fn new() -> Self {
        Self {
            obj: vec![],
            new: vec![],
            closed: false,
        }
    }

    /// Pushes a value into the vector. Panics if the vector is closed
    pub fn push(&mut self, t: T) {
        if self.closed {
            panic!("Attempt to push a value to a closed vector.")
        }

        self.new.push(t);
    }

    pub fn close(&mut self) {
        if self.closed {
            return;
        }

        self.obj.append(&mut self.new);
        self.closed = true;
    }

    pub fn iter(&mut self) -> ScopedVecIterator<T> {
        let mut v = vec![];
        if !self.closed {
            if self.obj.len() > 0 {
                v.push(self.obj[self.obj.len() - 1]);
            }

            for x in self.new.iter() {
                v.push(*x);
            }

            self.obj.append(&mut self.new);
        }
        else {
            for x in self.obj.iter() {
                v.push(*x);
            }
        }

        ScopedVecIterator::new(v)
    }
}

pub struct ScopedVecIterator<T: Copy + Debug> {
    data: Vec<T>,
    idx: usize,
}

impl<T: Copy + Debug> ScopedVecIterator<T> {
    fn new(x: Vec<T>) -> Self {
        Self {
            data: x,
            idx: 0
        }
    }
}

impl<T: Copy + Debug> Iterator for ScopedVecIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        if self.idx > self.data.len() {
            return None;
        }

        return Some(self.data[self.idx - 1]);
    }
}
