use dioxus::prelude::*;

mod components;
mod content;
mod fonts;
mod grid;
mod sections;

use crate::{
    components::{Footer, Header, Section},
    content::SECTIONS,
    sections::Hero,
};

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
            Hero {}
            for info in SECTIONS {
                Section { info, class: "border-t border-rule" }
            }
        }
        Footer {}
    }
}
