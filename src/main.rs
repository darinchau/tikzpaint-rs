use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <p>{"This is some test text"}</p>
            <h3>{"in bold"}</h3>
            <table border="1">
            <tbody>
            <tr>
            <td>{"This is a table"}</td>
            <td>{"that has no reason"}</td>
            <td>{"to be there"}</td>
            </tr>
            <tr>
            <td>{"why"}</td>
            <td>{"are you"}</td>
            <td>{"reading this?"}</td>
            </tr>
            <tr>
            <td>{"Lorem ipsum"}</td>
            <td>{"beep boop"}</td>
            <td>{"test text."}</td>
            </tr>
            </tbody>
            </table>
            <p>{"Desmond BB!"}</p>
            <p><img src="https://cse.hkust.edu.hk/admin/people/faculty/photos/desmond.jpg" alt="Desmond BB" width="206" height="274" /></p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}