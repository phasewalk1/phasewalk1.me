#![allow(non_snake_case)]
mod component;
mod statics;
mod style;
use yew::prelude::*;
use component::{Header, RootDirectory, GetInTouch};

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <style>{ style::Sheet() }</style>
            <Header />
            <RootDirectory />
            <GetInTouch />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
