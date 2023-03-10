#![allow(unused)]
//! Base module for Tikzpaint-rs. We try to structure the app such that
//! the "figures" module can also work as a CLI tool for Tikz figure generation

pub mod figures;
pub mod app;
pub mod renderer;
use yew::prelude::*;
use app::{CanvasManager, initialize_app};
use yew::suspense::{Suspension, SuspensionResult};

#[derive(Debug)]
struct User {
    name: String,
}

fn on_init() {
    initialize_app()
}

fn on_init_complete<F: FnOnce()>(_fn: F) {
    todo!()  // implementation omitted.
}

#[hook]
fn use_on_init() -> SuspensionResult<()> {
    let (s, handle) = Suspension::new();
    on_init_complete(move || {handle.resume();});
    todo!()
}

#[function_component(Content)]
fn content() -> HtmlResult {
    let _ = use_on_init()?;

    Ok(html!{
        <>
            <CanvasManager header_height={60} side_bar_width={190} terminal_height={150} figure_dims={2} debug={true}/>
        </>
    })
}

/// This component should be injected on the base app.
#[function_component(App)]
pub fn app() -> Html {
    let fallback = html! {
        <div>
            {"Loading..."}
        </div>
    };

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}
