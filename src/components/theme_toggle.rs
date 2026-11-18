use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdContrast;
use wasm_bindgen::prelude::*;

use crate::i18n::{local_storage, use_texts};

// В web-sys startViewTransition пока помечен нестабильным
// document.startViewTransition(update)
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document, js_name = startViewTransition)]
    fn start_view_transition(update: &JsValue);
}

#[component]
pub fn ThemeToggle() -> Element {
    let texts = use_texts();

    rsx! {
        button {
            class: "k-icon-btn",
            r#type: "button",
            aria_label: texts.header.theme_button,
            onclick: |_| toggle_theme(),
            Icon { icon: LdContrast, width: 16, height: 16 }
        }
    }
}

fn toggle_theme() {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Some(root) = document.document_element() else {
        return;
    };

    let next = match root.get_attribute("data-theme").as_deref() {
        Some("dark") => "light",
        _ => "dark",
    };

    let apply = move || {
        let _ = root.set_attribute("data-theme", next);
        if let Some(storage) = local_storage() {
            let _ = storage.set_item("theme", next);
        }
    };

    let supported = js_sys::Reflect::has(&document, &"startViewTransition".into()).unwrap_or(false);
    if supported {
        start_view_transition(&Closure::once_into_js(apply));
    } else {
        apply();
    }
}
