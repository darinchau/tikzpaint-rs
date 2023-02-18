//! A trait which, when defined, lets us serialize components/structs into strings and the other way round
use std::{fmt::Write, num::ParseIntError};

/// The serializable trait allows components/struct to turn from or into string.
/// let s = Self::from_str(self.into_str()); should be identical to Clone
pub trait Serializable where
Self: Sized {
    fn into_str(&self) -> String;
    fn from_str(s: &str) -> Option<Self>;
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

impl Serializable for f64 {
    /// Serializes a float into hex array
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::figures::Serializable;
    /// let x = 5.1;
    /// let hex_array = x.into_str();
    /// assert_eq!(hex_array, "f4014666666666666");
    ///
    /// ```
    ///
    /// # The classic example where 0.1 + 0.2 != 0.3
    /// ```
    /// use tikzpaint_rs::figures::Serializable;
    /// let x = (0.1 + 0.2).into_str();
    /// let y = (0.3).into_str();
    /// assert!(x != y);
    /// ```
    fn from_str(s: &str) -> Option<Self> {
        if s.starts_with('f') {
            if let Some(r) = decode_hex(&s[1..]).ok() {
                if r.len() != 8 {
                    return None;
                }

                let y = {
                    let mut t = [0_u8; 8];
                    for i in 0..8 {
                        t[i] = r[i];
                    }
                    t
                };

                Some(f64::from_be_bytes(y))
            }
            else {
                return None;
            }
        }
        else {
            None
        }
    }

    fn into_str(&self) -> String {
        let y = self.to_be_bytes();
        format!("f{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}", y[0], y[1], y[2], y[3], y[4], y[5], y[6], y[7])
    }
}
