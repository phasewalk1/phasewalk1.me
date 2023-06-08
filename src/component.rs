#![allow(non_snake_case)]
use yew::prelude::*;
use super::statics::refs::*;
use super::statics::guts::*;

#[function_component]
pub(super) fn Header() -> Html {
    html! {
        <div>
            <h1 class="large-text">{ HEADER_TEXT }</h1>
            <h3>
                <q class="medium-text">{ SUBHEADER_TEXT }</q>
            </h3>
        </div>
    }
}

#[function_component]
pub(super) fn MirrorHref() -> Html {
    html! {
        <a href={MIRROR}>
            { "mirror" }
        </a>
    }
}

#[function_component]
pub(super) fn GithubRef() -> Html {
    html! {
        <a href={GITHUB}>
            { "github" }
        </a>
    }
}

#[function_component]
pub(super) fn PersySiteRef() -> Html {
    html! {
        <a href={PERSY}>
            { "site" }
        </a>
    }
}

#[function_component]
pub(super) fn RootDirectory() -> Html {
    html! {
        <ul class="list">
            <li> <GithubRef /> </li>
            <li> <MirrorHref /> </li>
            <li> <PersySiteRef /> </li>
        </ul>
    }
}

#[function_component]
pub(super) fn ContactRefs() -> Html {
    html! {
        <div>
            <a href={ TWITTER }>
                {"twitter"}
            </a>
            <br/>
            <a href={ MAILTO }>
                {"mailto:kat@phasewalk.net"}
            </a>
        </div>
    }
}

#[function_component]
pub(super) fn GetInTouch() -> Html {
    html! {
        <div class="subhead">
            <h4>{"Get in touch"}</h4>
            <ContactRefs />
        </div>
    }
}

