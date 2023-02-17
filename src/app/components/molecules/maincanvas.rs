//! Implementation of the main canvas part of the app

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Serializable, Button, ButtonType};

pub enum ClickType {
    Point,
}

pub struct MainCanvasMessage {
    x: usize,
    y: usize,
    click: ClickType,
}

#[derive(Properties, PartialEq)]
pub struct MainCanvasProps {

}

/// A canvas is essentially a wrapper of a TikzPaint figure plus rendering properties
pub struct MainCanvas {

}