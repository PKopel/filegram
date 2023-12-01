mod decode;
mod encode;
mod utils;

use decode::DecodeComponent;
use encode::EncodeComponent;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <header>
        <h1>{"Filegram web app"}</h1>
        </header>
        <div class="main">
            <EncodeComponent/>
            <DecodeComponent/>
        </div>
        <footer>
            <p>{"Author: Paweł Kopel"}</p>
            <p><a href="https://github.com/PKopel/filegram">{"GitHub repository"}</a></p>
        </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
