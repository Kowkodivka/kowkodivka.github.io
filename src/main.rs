use dioxus::prelude::*;

mod components;
mod content;
mod fonts;

use components::{Footer, Header};

use crate::{components::Section, content::SECTIONS};

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
            div { class: "k-wrap py-32",
                p { class: "k-label", "/ 00 индекс" }
            }

            for info in SECTIONS {
                Section { info, class: "border-t border-rule" }
            }
        }
        Footer {}
    }
}
