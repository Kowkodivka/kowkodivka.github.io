mod en;
mod ru;

use crate::i18n::Lang;

pub const CLOCK_OFFSET_HOURS: i64 = 7;

pub fn texts(lang: Lang) -> &'static Texts {
    match lang {
        Lang::Ru => &ru::TEXTS,
        Lang::En => &en::TEXTS,
    }
}

pub struct Texts {
    pub page_title: &'static str,
    pub nick: &'static str,
    pub header: HeaderText,
    pub hero: HeroText,
    pub sections: [SectionInfo; 7],
    pub footer: FooterText,
}

pub struct HeaderText {
    pub clock_label: &'static str,
    pub nav_label: &'static str,
    pub theme_button: &'static str,
    pub lang_button: &'static str,
}

pub struct HeroText {
    pub index_label: &'static str,
    pub roles: &'static [&'static str],
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub intro: &'static str,
    pub primary_action: &'static str,
    pub secondary_action: &'static str,
    pub photo_alt: &'static str,
    pub coordinates: &'static str,
    pub city: &'static str,
    pub wordmark: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SectionInfo {
    pub id: &'static str,
    pub number: &'static str,
    pub label: &'static str,
    pub title: &'static str,
    pub accent: &'static str,
    pub title_end: &'static str,
}

pub struct FooterText {
    pub to_top: &'static str,
}
