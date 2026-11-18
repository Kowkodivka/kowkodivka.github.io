//! Язык страницы: русский или английский.
//!
//! Выбранный язык лежит в сигнале в контексте приложения. Компонент берёт тексты
//! через `use_texts()` и сам перерисуется, когда язык сменится.
//! Выбор сохраняется в localStorage; при первом визите берётся язык браузера.

use dioxus::prelude::*;

use crate::content::{Texts, texts};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Ru,
    En,
}

impl Lang {
    /// код языка для атрибута lang и localStorage
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

    /// "ru", "ru-RU" → русский, всё остальное → английский
    fn from_code(code: &str) -> Self {
        if code.starts_with("ru") {
            Lang::Ru
        } else {
            Lang::En
        }
    }
}

/// Создаёт язык для всего приложения. Вызывается один раз в App.
/// Язык известен сразу, ещё до первой отрисовки, поэтому страница не мигает русским.
pub fn use_lang_provider() {
    use_context_provider(|| {
        let lang = detect_lang();
        apply_lang(lang);
        Signal::new(lang)
    });
}

pub fn use_lang() -> Signal<Lang> {
    use_context()
}

/// Тексты на текущем языке.
pub fn use_texts() -> &'static Texts {
    let lang = use_lang();
    texts(lang())
}

/// Меняет язык, атрибут lang у <html> и запоминает выбор.
pub fn set_lang(mut lang: Signal<Lang>, next: Lang) {
    lang.set(next);
    apply_lang(next);
}

/// Сохранённый язык, иначе язык браузера, иначе русский.
fn detect_lang() -> Lang {
    let saved = local_storage().and_then(|storage| storage.get_item("lang").ok().flatten());
    let browser = web_sys::window().and_then(|window| window.navigator().language());

    match saved.or(browser) {
        Some(code) => Lang::from_code(&code),
        None => Lang::Ru,
    }
}

fn apply_lang(lang: Lang) {
    let code = lang.code();

    if let Some(root) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
    {
        let _ = root.set_attribute("lang", code);
    }
    // localStorage может быть недоступен (например, приватный режим) — тогда просто не запоминаем
    if let Some(storage) = local_storage() {
        let _ = storage.set_item("lang", code);
    }
}

pub fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}
