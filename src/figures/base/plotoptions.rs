//! PlotOptions is a struct that holds info (such as color, thickness etc) of a displayable/drawable

pub struct Color(u8, u8, u8);

impl Color {
    fn to_str(&self) -> String {
        let Color(r, g, b) = *self;
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
}


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

    pub fn to_str(&self) -> String {
        self.to_color().to_str()
    }
}

pub struct PlotOptions{
    color: TikzColor,
    thickness: f64
}

impl PlotOptions {
    pub fn new() -> PlotOptions {
        PlotOptions { 
            color: TikzColor::Black, 
            thickness: 1., 
        }
    }

    pub fn tikzify(&self) -> String {
        format!("color={}, line width={}pt", self.color.to_str(), self.thickness)
    }
}

impl Clone for PlotOptions {
    fn clone(&self) -> Self {
        PlotOptions { 
            color: self.color.clone(),
            thickness: self.thickness 
        }
    }
}
