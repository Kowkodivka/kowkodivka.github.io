use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdContrast};

const TOGGLE_THEME: &str = r#"
const root = document.documentElement;
const systemDark = matchMedia("(prefers-color-scheme: dark)").matches;
const current = root.dataset.theme || (systemDark ? "dark" : "light");
const next = current === "dark" ? "light" : "dark";

const apply = () => {
    root.dataset.theme = next;
    localStorage.setItem("theme", next);
};

if (document.startViewTransition) {
    document.startViewTransition(apply);
} else {
    apply();
}
"#;

#[component]
pub fn ThemeToggle() -> Element {
    rsx! {
        button {
            class: "k-icon-btn",
            r#type: "button",
            aria_label: "сменить тему",
            onclick: |_| {
                document::eval(TOGGLE_THEME);
            },
            Icon { icon: LdContrast, width: 16, height: 16 }
        }
    }
}
