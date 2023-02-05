mod base;
mod figureobjects;

pub use base::coordinates::Coordinates;

pub use base::figureobject::FigureObject;
pub use base::figureobject::Drawable;
pub use base::figureobject::Plot;

pub use base::plotoptions::Color;
pub use base::plotoptions::PlotOptions;
pub use base::plotoptions::TikzColor;

pub use base::figure::Figure;

pub use base::projection::Projection;
pub use base::projection::Identity;
pub use base::projection::Matrix;
pub use base::projection::Concat;
