use dioxus::prelude::*;

mod components;
mod content;
mod fonts;

use components::{Footer, Header};

const STYLES: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Kowkodivka" }
        fonts::Fonts {}
        document::Stylesheet { href: STYLES }

        Header {}
        main { id: "top",
            div { class: "h-svh flex items-center justify-center",
                p { "Hello, World!" }
            }
        }
        Footer {}
    }
}
