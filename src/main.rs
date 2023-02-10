use gloo::console::log;
use yew::prelude::*;
use tikzpaint_rs::app::{Switch, TextField, GetProperty};

#[function_component(App)]
fn app() -> Html {
    let fieldcb = Callback::from(|(_, _): (Event, String)| {

    });

    let switchcb = Callback::from(|x: MouseEvent| {
        let switch = Switch::get_component(x).ok().unwrap();
        log!(format!("{:?}", switch.state))
    });

    html! {
        <>
            <TextField name={"something"} label={"Hiya! Enter stuff: "} cb={fieldcb}></TextField>
            <Switch cb={switchcb}>
                {"Press me"}
            </Switch>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}




