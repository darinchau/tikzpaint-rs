//! A trait which, when defined, lets us serialize components/structs into strings and the other way round

#![feature(generic_const_exprs)]

use std::{fmt::Write, num::ParseIntError};
use std::any::TypeId;
use std::mem::size_of;

/// The serializable trait allows components/struct to turn from or into string.
/// let s = Self::from_str(self.into_str()); should be identical to Clone
pub trait Serializable where
Self: Sized {
    fn into_str(&self) -> String;
    fn from_str(s: &str) -> Option<Self>;
}

// Implementing Serializable for all the primitives and String
fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn encode_to_hex<const N: usize>(a: &'static str, s: [u8; N]) -> String {
    let mut res = String::from(a);
    for u in s.into_iter() {
        res.push_str(&(format!("{:x}", u)));
    }
    res
}

fn decode_from_hex<const N: usize>(a: &'static str, s: &str) -> Option<[u8; N]> {
    if s.starts_with(a) {
        if let Some(r) = decode_hex(&s[a.len()..]).ok() {
            if r.len() != N {
                return None;
            }

            let y = {
                let mut t = [0_u8; N];
                for i in 0..N {
                    t[i] = r[i];
                }
                t
            };

            return Some(y);
        }
        return None;
    }
    return None;
}

// I am using combinations of macros and generics so I can write simpler macros
macro_rules! seriz {
    {$($t:ty : $id:expr),*} => {
        $ (
            impl Serializable for $t {
                fn from_str(s: &str) -> Option<Self> {
                    if let Some(x) = decode_from_hex($id, s) {
                        return Some(<$t>::from_be_bytes(x));
                    }
                    return None;
                }

                fn into_str(&self) -> String {
                    let y = self.to_be_bytes();
                    encode_to_hex($id, y)
                }
            }
        )*
    }
}

seriz!(
    u128: "q",
    i128: "w",
    u64: "u",
    i64: "i",
    f64: "f",
    i32: "e",
    u32: "r",
    f32: "t",
    u16: "y",
    i16: "o",
    u8: "p",
    i8: "a"
);

impl Serializable for bool {
    fn from_str(s: &str) -> Option<Self> {
        if s == "1" {
            return Some(true);
        }
        else if s == "0" {
            return Some(false);
        }
        return None;
    }

    fn into_str(&self) -> String {
        if *self {String::from("1")} else {String::from("0")}
    }
}

impl Serializable for char {
    fn into_str(&self) -> String {
        String::from(*self)
    }

    fn from_str(s: &str) -> Option<Self> {
        let sa: Vec<char> = s.chars().collect();
        if sa.len() == 1 {
            return Some(sa[0]);
        }
        return None;
    }
}

impl Serializable for String {
    fn from_str(s: &str) -> Option<Self> {
        Some(String::from(s))
    }

    fn into_str(&self) -> String {
        String::from(self)
    }
}
