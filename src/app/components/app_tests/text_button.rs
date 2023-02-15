use gloo::console::log;
use yew::prelude::*;
use crate::app::{Switch, TextField, GetProperty, Button, ButtonType};

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| String::new());
    let onchange_state = state.clone();
    let onchange = Callback::from(move |(_, x): (Event, String)| {
        onchange_state.set(x);
    });

    let buttoncb = Callback::from(move |x: MouseEvent| {
        log!(format!("{}", &*state))
    });

    html! {
        <>
            <TextField name={"something"} label={"Hiya! Enter stuff: "} onchange={onchange}></TextField>
            <Button button_type={ButtonType::Submit} name={"submit"} cb={buttoncb}>
                {"Press me"}
            </Button>
        </>
    }
}