//! This submodule defines the variable types and handles the parsing

use std::fmt::Debug;
use std::any::TypeId;
use std::any::Any;
use crate::core::calc::*;

macro_rules! variable_types {
    ($($x:ident, $t:ty), *) => {
        #[derive(Clone, Copy, PartialEq)]
        /// A variable is like a macro - it is something that contains a single ASTNode
        pub enum VariableType {
            $(
                $x,
            ) *
        }

        impl Debug for VariableType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $ (
                        VariableType::$x => write!(f, stringify!($x)),
                    ) *
                }
            }
        }

        #[derive(Clone, PartialEq)]
        pub enum VariablePayload {
            $(
                $x($t),
            ) *
        }

        impl Debug for VariablePayload {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $ (
                        VariablePayload::$x(y) => write!(f, "Var({:?})", y),
                    ) *
                }
            }
        }

        // impl VariablePayload {
        //     /// Attempts to turn a variable payload into the underlying desired type. Returns an error if the types does not match
        //     pub fn parse<T: Clone + 'static>(&self) -> Result<T, &'static str> {
        //         let t_type = TypeId::of::<T>();
        //         match self {
        //             $ (
        //                 VariablePayload::$x(y) => {
        //                     if t_type == TypeId::of::<$t>() {
        //                         let y_any = (&y as &dyn Any).downcast_ref::<T>().unwrap();
        //                         return Ok(y_any.to_owned());
        //                     }

        //                     Err("Types does not match")
        //                 }
        //             ) *
        //         }
        //     }
        // }
    };
}

impl PartialEq<f64> for VariablePayload {
    fn eq(&self, other: &f64) -> bool {
        if let VariablePayload::Number(x) = self {
            return eq(x, other);
        }
        return false;
    }
}

variable_types!{
    Number, f64,
    NumberTuple, Vec<f64>
}

/// Note: This is very convenient but also dangerous. Use only when you are 3000% sure the types match
impl From<&VariablePayload> for f64 {
    fn from(value: &VariablePayload) -> Self {
        match value {
            VariablePayload::Number(x) => *x,
            _ => panic!("Types does not match! Told you not to use implicit conversion :D")
        }
    }
}

/// Note: This is very convenient but also dangerous. Use only when you are 3000% sure the types match
impl From<&VariablePayload> for Vec<f64> {
    fn from(value: &VariablePayload) -> Self {
        match value {
            VariablePayload::NumberTuple(x) => x.clone(),
            _ => panic!("Types does not match! Told you not to use implicit conversion :D")
        }
    }
}