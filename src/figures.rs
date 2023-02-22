mod base;
mod figureobjects;
mod drawables;
mod utils;

pub use base::coordinates::Coordinates;

pub use base::figureobject::HasCoordinates;
pub use base::figureobject::Drawable;
pub use base::figureobject::Plottable;
pub use base::figureobject::DrawableWrapper;

pub use base::plotoptions::Color;
pub use base::plotoptions::PlotOptions;
pub use base::plotoptions::TikzColor;
pub use base::plotoptions::tikzify_field;

pub use base::figure::Figure;

pub use base::projection::IsProjection;
pub use base::projection::Identity;
pub use base::projection::Matrix;
pub use base::projection::Concat;
pub use base::projection::Projection;

// pub use figureobjects::point::FOPoint;
// pub use drawables::point::Point;

pub use utils::serializable::Serializable;
pub use utils::hashable::Hashable;
pub use utils::dimensionerror::DimensionError;
