use dioxus::prelude::*;

mod components;
mod fonts;

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

        main {
        }
    }
}
