use dioxus::prelude::*;

use crate::content::{Texts, texts};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Ru,
    En,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::Ru => "ru",
            Lang::En => "en",
        }
    }

    pub fn other(self) -> Self {
        match self {
            Lang::Ru => Lang::En,
            Lang::En => Lang::Ru,
        }
    }

    fn from_code(code: &str) -> Self {
        if code.starts_with("ru") {
            Lang::Ru
        } else {
            Lang::En
        }
    }
}

const DETECT_LANG: &str = r#"
return localStorage.getItem("lang") || navigator.language || "ru";
"#;

pub fn use_lang_provider() {
    let lang = use_context_provider(|| Signal::new(Lang::Ru));

    use_effect(move || {
        spawn(async move {
            if let Ok(code) = document::eval(DETECT_LANG).join::<String>().await {
                set_lang(lang, Lang::from_code(&code));
            }
        });
    });
}

pub fn use_lang() -> Signal<Lang> {
    use_context()
}

pub fn use_texts() -> &'static Texts {
    let lang = use_lang();
    texts(lang())
}

pub fn set_lang(mut lang: Signal<Lang>, next: Lang) {
    lang.set(next);
    let code = next.code();
    document::eval(&format!(
        r#"document.documentElement.lang = "{code}"; localStorage.setItem("lang", "{code}");"#
    ));
}
