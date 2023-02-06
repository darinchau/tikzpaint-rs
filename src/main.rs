use tikzpaint_rs::app::TopNavBar;
#[allow(unused_imports)]
use yew::prelude::*;

use tikzpaint_rs::app::*;

fn main() {
    yew::Renderer::<TopNavBar>::new().render();
    yew::Renderer::<Canvas>::new().render();
    yew::Renderer::<Button>::new().render();
}

