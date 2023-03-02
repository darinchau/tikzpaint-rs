mod base;
mod figureobjects;
mod drawables;
mod utils;
mod renderer;

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
pub use utils::dimension_error::DimensionError;
pub use utils::cheap_string::CheapString;
pub use utils::cheap_string::StringLike;

pub use renderer::svg_renderer::*;
pub use renderer::tikz_renderer::*;
pub use renderer::util::*;
