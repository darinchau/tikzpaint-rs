mod base;
mod figureobjects;
mod drawables;
mod utils;

pub use base::coordinates::Coordinates;

// Using the wrap design pattern (invented by me??) :))
pub use base::figureobject::Plottable;
pub use base::figureobject::PlottableObject;

pub use base::figureobject::IsFigureObject;
pub use base::figureobject::FigureObject;
pub use base::figureobject::WrappableAsFigureObject;

pub use base::figureobject::Drawable;
pub use base::figureobject::DrawableObject;
pub use base::figureobject::WrappableAsDrawable;

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
pub use base::projection::WrappableAsProjection;

pub use figureobjects::point::FOPoint;
pub use drawables::point::Point;

pub use utils::serializable::Serializable;
pub use utils::hashable::Hashable;
pub use utils::dimensionerror::DimensionError;
