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
            <h1>{"Filegram image app"}</h1>
            <EncodeComponent/>
            <DecodeComponent/>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
