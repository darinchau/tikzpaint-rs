use gloo::console::log;
use yew::prelude::*;
use tikzpaint_rs::app::{Switch, GetProperty};

#[function_component(App)]
fn app() -> Html {
    let click = Callback::from(|x: MouseEvent| {
        if let Some(switch) = Switch::get_component(x).ok() {
            log!(format!("From main: {:?}", switch.state));
        }
        else {
            log!("Got something that is not a switch na")
        }
    });
    html! {
        <Switch id={"test"} cb={click}>
            {"hiya"}
        </Switch>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}



