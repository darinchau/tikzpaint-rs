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

pub use base::drawable::Drawable;
pub use base::drawable::DrawableObject;
pub use base::drawable::WrappableAsDrawable;

pub use base::figure::Figure;

pub use base::projection::IsProjection;
pub use base::projection::Identity;
pub use base::projection::Matrix;
pub use base::projection::Concat;
pub use base::projection::Projection;
pub use base::projection::WrappableAsProjection;

pub use figureobjects::point::FOPoint;
pub use drawables::point::Point;

pub use utils::dimension_error::DimensionError;
pub use utils::cheap_string::CheapString;
pub use utils::cheap_string::StringLike;
pub use utils::coord_transform::CoordTransform;
