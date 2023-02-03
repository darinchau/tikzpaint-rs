mod base;
mod utils;
pub mod shapes;

pub use utils::coordinates::Coordinates;

pub use base::displayable::Displayable;
pub use base::plotoptions::Color;
pub use base::plotoptions::PlotOptions;
pub use base::plotoptions::TikzColor;

pub use shapes::point::Point;