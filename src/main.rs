use dioxus::prelude::*;

mod components;
mod content;
mod grid;
mod i18n;
mod sections;

use components::{Footer, Header, Section};
use i18n::use_texts;
use sections::Hero;

const STYLES: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    i18n::use_lang_provider();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: STYLES }
        Page {}
    }
}

#[component]
fn Page() -> Element {
    let texts = use_texts();

    rsx! {
        document::Title { "{texts.page_title}" }
        Header {}
        main { id: "top",
            Hero {}

            for info in texts.sections {
                Section { info, class: "border-t border-rule" }
            }
        }
        Footer {}
    }
}
