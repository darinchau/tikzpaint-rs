//! Nav Bar on top

use yew::prelude::*;

pub struct TopNavBar {
    state: TopNavBarMsg,
}

#[derive(PartialEq, Clone, Copy)]
pub enum TopNavBarMsg {
    Home,
    News,
    Contact,
    About
}

struct TopNavBarButton {
    id: usize,
    message: TopNavBarMsg,
    text: &'static str,
}

impl Component for TopNavBar {
    type Message = TopNavBarMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        TopNavBar {
            state: TopNavBarMsg::Home
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state = msg;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let buttons = vec![
            TopNavBarButton{
                id: 1,
                message: TopNavBarMsg::Home,
                text: "Home",
            },
            TopNavBarButton{
                id: 2,
                message: TopNavBarMsg::News,
                text: "News",
            },
            TopNavBarButton{
                id: 3,
                message: TopNavBarMsg::Contact,
                text: "Contact",
            },
            TopNavBarButton{
                id: 4,
                message: TopNavBarMsg::About,
                text: "About",
            }
        ];

        let link = ctx.link();

        let html_buttons = buttons.iter().map(|x| {
            let active = x.message == self.state;
            let button_class = if active {classes!("active")} else {classes!()};
            html! {
                <button class={button_class} onclick={link.callback(|_| {
                    TopNavBarMsg::Home
                })}>{"Some text"}</button>
            }
        }).collect::<Html>();

        html! {
            <div class="topnav" id="myTopnav">
                { html_buttons }
            </div>
        }
    }
}
