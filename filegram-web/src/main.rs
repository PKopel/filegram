mod decode;
mod encode;

use decode::DecodeComponent;
use encode::EncodeComponent;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <div>
            <div>
                <h1>{"Filegram web app"}</h1>
                <a href="https://github.com/PKopel/filegram">{"GitHub repository"}</a>
            </div>
            <EncodeComponent/>
            <DecodeComponent/>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
