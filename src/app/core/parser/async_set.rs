//! This module contains all predefined functions i.e. patterns
use std::sync::Mutex;
use std::sync::Arc;

use crate::figures::*;

// I don't know whats the "correct" way to go about this
// I want to export a multithreaded drainable hashmap to check patterns using multiple workers

#[derive(Clone)]
/// This is a data structure that supports multiprocess loopings
pub struct AsyncSet<T> where
T: Sized + Clone {
    data: Arc<Mutex<(Vec<T>, usize)>>,
    looping: bool
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AsyncSetError {
    PushWhileLooping,
    MultipleStart,
    NoMoreValues,
    GetWhileNotLooping
}

impl<T: Clone> AsyncSet<T> {
    pub fn new() -> Self {
        return Self {
            data: Arc::new(Mutex::new((vec![], 0))),
            looping: false
        }
    }

    /// To add some data into the set. This ought to be called in the main thread.
    /// This guarantees that we do not
    pub fn push(&mut self, x: T) -> Result<(), AsyncSetError> {
        if self.looping {
            return Err(AsyncSetError::PushWhileLooping);
        }

        (*self.data.lock().unwrap()).0.push(x);
        Ok(())
    }

    /// Indicates that we starts the looping. This ought to be called in the main thread.
    pub fn start_loop(&mut self) -> Result<(), AsyncSetError> {
        if self.looping {
            return Err(AsyncSetError::MultipleStart);
        }

        let mut x = self.data.lock().unwrap();
        let (_, mut idx) = *x;
        idx += 1;
        self.looping = true;
        Ok(())
    }

    /// Gets a value in thread for processing. This will clone T in the process
    pub fn get(&self) -> Result<T, AsyncSetError> {
        if !self.looping {
            return Err(AsyncSetError::GetWhileNotLooping);
        }
        let (data, mut idx) = &*self.data.lock().unwrap();
        if idx >= data.len() {
            return Err(AsyncSetError::NoMoreValues);
        }
        let t = data[idx].clone();
        self.data.lock().unwrap().1 += 1;
        return Ok(t);
    }

    // Just in case we want to finish the loop, maybe add more values, and loop again some time later
    pub fn finish(&mut self) {
        let mut x = self.data.lock().unwrap();
        let (_, mut idx) = *x;
        self.looping = false;
    }
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::sync::mpsc;
    use super::*;

    #[test]
    fn test_1() {
        let num_values = 30;

        let mut set = AsyncSet::new();
        let mut v = Arc::new(Mutex::new(vec![false; num_values]));

        for i in 0..num_values {
            set.push(i);
        }

        set.start_loop();

        let mut handles = vec![];

        for i in 0..10 {
            let vect = v.clone();
            let set1 = set.clone();
            let handle = thread::spawn(move || {
                match set1.get() {
                    Ok(x) => {
                        (vect.lock().unwrap())[x] = true;
                    },
                    Err(e) => {
                        if e == AsyncSetError::NoMoreValues {
                            return;
                        }
                        panic!("{:?}", e);
                    }
                }

            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        set.finish();

        let vect = &*v.lock().unwrap();

        for i in 0..num_values {
            assert_eq!(vect[i], true);
        }
    }
}