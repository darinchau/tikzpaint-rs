//! Responsible for handling terminal text

use crate::figures::*;
use crate::app::*;
use std::rc::Rc;
use std::cell::RefCell;
use yew::prelude::*;

pub struct TerminalTextRenderer {
    v: Vec<CheapString>
}

impl TerminalTextRenderer {
    pub fn new() -> Self {
        TerminalTextRenderer { v: vec![] }
    }

    pub fn push(&mut self, r: CheapString) {
        self.v.push(r);
    }

    pub fn unpack(&self) -> Vec<CheapString> {
        self.v.iter().map(|x| {
            x.clone()
        }).collect::<Vec<CheapString>>()
    }

    pub fn unpack_html(&self) -> Html {
        self.v.iter().map(|x| {
            html!{
                <>
                    // Unwrap the cheap string from x and then clone
                    {x.clone()}
                    <br/>
                </>
            }
        }).collect::<Html>()
    }
}

impl PartialEq for TerminalTextRenderer {
    fn eq(&self, other: &Self) -> bool {
        for (x, y) in self.v.iter().zip(other.v.iter()) {
            if x != y {
                return false;
            }
        }

        true
    }
}