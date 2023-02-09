use yew::prelude::*;
use tikzpaint_rs::app::Switch;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Switch>
            {"Hiya!"}
        </Switch>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}


