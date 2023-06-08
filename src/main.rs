use self::statics::guts::*;
use self::statics::refs::*;
use yew::prelude::*;

pub(self) mod statics {
    type StaticStr = &'static str;
    pub(super) mod refs {
        use super::StaticStr;
        pub(crate) static MIRROR: StaticStr =
            "https://mirror.xyz/0x497437B2aC4B2D408138bE772233E3F2dF8b7064";
        pub(crate) static GITHUB: StaticStr = "https://github.com/phasewalk1";
        pub(crate) static PERSY: StaticStr = "https://phasewalk1.github.io";
        pub(crate) static TWITTER: StaticStr = "https://twitter.com/phasewalk1";
        pub(crate) static MAILTO: StaticStr = "mailto:kat@phasewalk1.net";
    }

    pub(super) mod guts {
        use super::StaticStr;
        pub(crate) static HEADER_TEXT: StaticStr = "phasewalk";
        pub(crate) static SUBHEADER_TEXT: StaticStr = "Pure mathematics, Dark software.";
    }
}

#[function_component]
fn Header() -> Html {
    html! {
        <div>
            <h1>{ HEADER_TEXT }</h1>
            <h3>
                <q>{ SUBHEADER_TEXT }</q>
            </h3>
        </div>
    }
}

#[function_component]
fn MirrorHref() -> Html {
    html! {
        <a href={MIRROR}>
            { "mirror" }
        </a>
    }
}

#[function_component]
fn GithubRef() -> Html {
    html! {
        <a href={GITHUB}>
            { "github" }
        </a>
    }
}

#[function_component]
fn PersySiteRef() -> Html {
    html! {
        <a href={PERSY}>
            { "site" }
        </a>
    }
}

#[function_component]
fn RootDirectory() -> Html {
    html! {
        <ul>
            <li> <GithubRef /> </li>
            <li> <MirrorHref /> </li>
        </ul>
    }
}

#[function_component]
fn ContactRefs() -> Html {
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
fn GetInTouch() -> Html {
    html! {
        <div>
            <h4>{"Get in touch"}</h4>
            <ContactRefs />
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Header />
            <RootDirectory />
            <GetInTouch />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
