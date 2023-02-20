//! PlotOptions is a struct that holds info (such as color, thickness etc) of a displayable/drawable

use std::fmt::{Display};
use crate::figures::Serializable;

/// An OptionField allows default options on specification. This forces us to handle defaults when parsing options
#[derive(Clone)]
pub enum OptionField<T> where
T: DisplayOption + Clone + Serializable {
    Custom(T),
    Default,
    None
}

impl<T: DisplayOption + Clone + Serializable> Serializable for OptionField<T> {
    fn from_str(s: &str) -> Option<Self> {
        if !s.starts_with("opt") {
            return None;
        }

        let trail = &s[3..];
        if trail == "d" {
            return Some(OptionField::Default);
        }
        else if trail == "n" {
            return Some(OptionField::None);
        }
        else if let Some(t) = T::from_str(trail) {
            return Some(OptionField::Custom((t)));
        }

        return None;
    }

    fn into_str(&self) -> String {
        match self {
            OptionField::Default => String::from("optd"),
            OptionField::None => String::from("optn"),
            OptionField::Custom(t) => format!("opt{}", t.into_str())
        }
    }
}

/// The DisplayOption trait specifies the formatting that we should display the types under different contexts
pub trait DisplayOption {
    /// The standard implementation of conversion to string
    fn to_str(&self) -> String;
    /// Converts the type into a tikz display inside the square bracket
    fn to_tikz(&self) -> String;
}

impl<T: Display> DisplayOption for T {
    fn to_str(&self) -> String {
        format!("{}", self)
    }

    fn to_tikz(&self) -> String {
        format!("{}", self)
    }
}


#[derive(Clone)]
pub struct Color(u8, u8, u8);

impl DisplayOption for Color {
    fn to_str(&self) -> String {
        let Color(r, g, b) = *self;
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    fn to_tikz(&self) -> String {
        let Color(r, g, b) = *self;
        format!("{{rgb,255:red,{};green,{};blue,{}}}", r, g, b)
    }
}

impl Serializable for Color {
    fn into_str(&self) -> String {
        self.to_str()
    }

    fn from_str(s: &str) -> Option<Self> {
        if !s.starts_with("#") || s.chars().count() != 7 {
            return None;
        }

        let r = s[1..3].parse::<u8>().ok()?;
        let g = s[3..5].parse::<u8>().ok()?;
        let b = s[5..7].parse::<u8>().ok()?;
        return Some(Color(r, g, b));
    }
}

/// Colors enum containing all colors available in Tikz
#[derive(Clone)]
pub enum TikzColor {
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    Black,
    Gray,
    Darkgray,
    Lightgray,
    Brown,
    Lime,
    Olive,
    Orange,
    Pink,
    Purple,
    Teal,
    Violet,
    White
}

// Paste the Python version of the dict and use
// for k, v in f.items():
//     r = int(v[1:3], 16)
//     g = int(v[3:5], 16)
//     b = int(v[5:7], 16)
//     print(f"TikzColor::{k.title()} => Color({r}, {g}, {b}),")

impl TikzColor {
    /// Convert a TikzColor enum to RGB color scheme
    pub fn to_color(&self) -> Color {
        match self {
            TikzColor::Red => Color(238, 0, 0),
            TikzColor::Green => Color(0, 238, 0),
            TikzColor::Blue => Color(0, 0, 238),
            TikzColor::Cyan => Color(0, 238, 238),
            TikzColor::Magenta => Color(238, 0, 238),
            TikzColor::Yellow => Color(238, 238, 0),
            TikzColor::Black => Color(0, 0, 0),
            TikzColor::Gray => Color(136, 136, 136),
            TikzColor::Darkgray => Color(68, 68, 68),
            TikzColor::Lightgray => Color(187, 187, 187),
            TikzColor::Brown => Color(150, 75, 0),
            TikzColor::Lime => Color(191, 255, 0),
            TikzColor::Olive => Color(128, 128, 0),
            TikzColor::Orange => Color(255, 165, 0),
            TikzColor::Pink => Color(255, 105, 180),
            TikzColor::Purple => Color(179, 0, 179),
            TikzColor::Teal => Color(0, 154, 154),
            TikzColor::Violet => Color(238, 130, 238),
            TikzColor::White => Color(238, 238, 238),
        }
    }
}

#[derive(Clone)]
pub struct PlotOptions{
    pub fill_color: OptionField<Color>,
    pub thickness: OptionField<u64>
}

impl PlotOptions {
    pub fn new() -> PlotOptions {
        PlotOptions {
            fill_color: OptionField::Default,
            thickness: OptionField::Default,
        }
    }
}

impl Serializable for PlotOptions {
    fn into_str(&self) -> String {
        let mut s = String::from("po");
        s.push_str(&self.fill_color.into_str());
        s.push_str("; ");
        s.push_str(&self.thickness.into_str());
        s.push_str("; ");
        return s;
    }

    fn from_str(s: &str) -> Option<Self> {
        if !s.starts_with("po") {
            return None
        }

        let mut tokens = (&s[2..]).split("; ");
        let fill_color = OptionField::<Color>::from_str(tokens.next()?)?;
        let thickness = OptionField::<u64>::from_str(tokens.next()?)?;
        return Some(Self {fill_color, thickness});
    }
}

/// A helper function to convert a Plot Option into string
pub fn tikzify_field<T>(s: &mut String, field: &OptionField<T>, field_name: &str) where
T: DisplayOption + Clone + Serializable {
    if let OptionField::Custom(t) = field {
        s.push_str(field_name);
        s.push_str("=");
        s.push_str(&t.to_tikz())
    }
}
