use tikzpaint_rs::App;
use tikzpaint_rs::app::initialize_app;

fn main() {
    initialize_app();
    yew::Renderer::<App>::new().render();
}