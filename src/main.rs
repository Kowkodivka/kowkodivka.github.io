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

const FAVICON: Asset = asset!("/assets/favicon.ico");
const STYLES: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        fonts::Fonts {}
        document::Title { "Kowkodivka" }
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
