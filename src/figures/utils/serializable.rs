//! A trait which, when defined, lets us serialize components/structs into strings and the other way round

#![feature(generic_const_exprs)]

use std::{fmt::Write, num::ParseIntError};
use std::any::TypeId;
use std::mem::size_of;

/// The serializable trait allows components/struct to turn from or into string.
/// let s = Self::from_str(self.into_str()); should be identical to Clone
pub trait Serializable {
    fn into_str(&self) -> String;
    fn from_str(s: &str) -> Option<Self> where Self: Sized;
}

const ENCODING: [char; 256] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï', 'Ð', 'Ñ', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', 'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß', 'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï', 'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷', 'ø', 'ù', 'ú', 'û', 'ü', 'ý', 'þ', 'ÿ', 'Ā', 'ā', 'Ă', 'ă', 'Ą', 'ą', 'Ć', 'ć', 'Ĉ', 'ĉ', 'Ċ', 'ċ', 'Č', 'č', 'Ď', 'ď', 'Đ', 'đ', 'Ē', 'ē', 'Ĕ', 'ĕ', 'Ė', 'ė', 'Ę', 'ę', 'Ě', 'ě', 'Ĝ', 'ĝ', 'Ğ', 'ğ', 'Ġ', 'ġ', 'Ģ', 'ģ', 'Ĥ', 'ĥ', 'Ħ', 'ħ', 'Ĩ', 'ĩ', 'Ī', 'ī', 'Ĭ', 'ĭ', 'Į', 'į', 'İ', 'ı', 'Ĳ', 'ĳ', 'Ĵ', 'ĵ', 'Ķ', 'ķ', 'ĸ', 'Ĺ', 'ĺ', 'Ļ', 'ļ', 'Ľ', 'ľ', 'Ŀ', 'ŀ', 'Ł', 'ł', 'Ń', 'ń', 'Ņ', 'ņ', 'Ň', 'ň', 'ŉ', 'Ŋ', 'ŋ', 'Ō', 'ō', 'Ŏ', 'ŏ', 'Ő', 'ő', 'Œ', 'œ', 'Ŕ', 'ŕ', 'Ŗ', 'ŗ', 'Ř', 'ř', 'Ś', 'ś', 'Ŝ', 'ŝ', 'Ş', 'ş', 'Š', 'š', 'Ţ', 'ţ', 'Ť', 'ť', 'Ŧ', 'ŧ', 'Ũ', 'ũ', 'Ū', 'ū', 'Ŭ', 'ŭ', 'Ů', 'ů', 'Ű', 'ű', 'Ų', 'ų', 'Ŵ', 'ŵ', 'Ŷ', 'ŷ', 'Ÿ', 'Ź', 'ź', 'Ż', 'ż', 'Ž', 'ž', 'ſ', 'ƀ', 'Ɓ'];

fn encode_u8(s: u8) -> char {
    return ENCODING[s as usize];
}

fn decode_u8(s: char) -> Option<u8> {
    let h = unsafe {
        *(&s as *const char as *const u32)
    };
    let k = match h {
        _ if (h < 58) => h-48,
        _ if (h < 97) => h-55,
        _ if (h < 123) => h-61,
        _ => h - 130
    };
    return u8::try_from(k).ok();
}

// Implementing Serializable for all the primitives and String
fn decode_hex(s: &str) -> Result<Vec<u8>, &'static str> {
    let mut res = vec![];
    for c in s.chars() {
        if let Some(x) = decode_u8(c) {
            res.push(x);
        }
        else {
            return Err("Failed to parse string as base 256 encoded string.");
        }
    }
    return Ok(res);
}

fn encode_to_hex<const N: usize>(a: &'static str, s: [u8; N]) -> String {
    let mut res = String::from(a);
    for i in s {
        res.push(encode_u8(i));
    }
    return res;
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
                    s.parse::<$t>().ok()
                }

                fn into_str(&self) -> String {
                    format!("{}", self)
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
