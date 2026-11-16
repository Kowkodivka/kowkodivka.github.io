pub const NICK: &str = "Kowkodivka";

pub struct HeroText {
    pub role: &'static str,
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub intro: &'static str,
    pub primary_action: &'static str,
    pub secondary_action: &'static str,
    pub coordinates: &'static str,
    pub city: &'static str,
    pub wordmark: &'static str,
}

pub const HERO: HeroText = HeroText {
    role: "full-stack ламер * ТГУ",
    first_name: "Юрий",
    last_name: "Хошин",
    intro: "я еблан",
    primary_action: "написать",
    secondary_action: "проекты",
    coordinates: "56.48° N * 84.95° E",
    city: "ТОМСК",
    wordmark: "KOWKODIVKA",
};

pub const CLOCK_LABEL: &str = "UTC+7";
pub const CLOCK_OFFSET_HOURS: i64 = 7;

#[derive(Clone, Copy, PartialEq)]
pub struct SectionInfo {
    pub id: &'static str,
    pub number: &'static str,
    pub label: &'static str,
    pub title: &'static str,
    pub accent: &'static str,
    pub title_end: &'static str,
}

pub const SECTIONS: [SectionInfo; 7] = [
    SectionInfo {
        id: "about",
        number: "01",
        label: "о себе",
        title: "...",
        accent: "...",
        title_end: "...",
    },
    SectionInfo {
        id: "stack",
        number: "02",
        label: "стек",
        title: "Таблица ",
        accent: "элементов",
        title_end: "",
    },
    SectionInfo {
        id: "projects",
        number: "03",
        label: "проекты",
        title: "Один ",
        accent: "в работе",
        title_end: ", остальные — впереди",
    },
    SectionInfo {
        id: "setup",
        number: "04",
        label: "сетап",
        title: "Паспорт ",
        accent: "оборудования",
        title_end: "",
    },
    SectionInfo {
        id: "music",
        number: "05",
        label: "музыка",
        title: "На ",
        accent: "повторе",
        title_end: "",
    },
    SectionInfo {
        id: "games",
        number: "06",
        label: "игры",
        title: "Сейчас ",
        accent: "прохожу",
        title_end: "",
    },
    SectionInfo {
        id: "contact",
        number: "07",
        label: "связь",
        title: "...",
        accent: "...",
        title_end: "",
    },
];
