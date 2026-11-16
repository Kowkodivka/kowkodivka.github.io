use dioxus::prelude::*;

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
            svg {
                width: "16",
                height: "16",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.7",
                circle { cx: "12", cy: "12", r: "8" }
                path { d: "M12 4a8 8 0 0 0 0 16z", fill: "currentColor" }
            }
        }
    }
}
