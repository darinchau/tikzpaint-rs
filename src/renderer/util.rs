//! Defines Tikz and SVG property field

pub trait IsTikzPropertyField {
    fn to_tikz(&self) -> (String, Option<String>);
}

pub trait IsSVGPropertyField {
    fn to_svg(&self) -> String;
}


#[derive(Clone, Copy, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl IsSVGPropertyField for Color {
    fn to_svg(&self) -> String {
        let Color(r, g, b) = self;
        return format!("rgb({r},{g},{b})");
    }
}

impl IsTikzPropertyField for Color {
    /// This function returns two strings - the first one being the one in the commands,
    /// the second one being any side effects on the preamble - for example using patterns
    /// and defining colors
    fn to_tikz(&self) -> (String, Option<String>) {
        let Color(r, g, b) = self;
        let st = format!("{{rgb,255:red,{r};green,{g};blue,{b}}}");
        return (st, None)
    }
}

macro_rules! propertify {
    ($($t:ty), *) => {
        $ (
            impl IsSVGPropertyField for $t {
                fn to_svg(&self) -> String {
                    self.to_string()
                }
            }

            impl IsTikzPropertyField for $t {
                fn to_tikz(&self) -> (String, Option<String>) {
                    (self.to_string(), None)
                }
            }
        )*
    };
}

propertify!(f64);
