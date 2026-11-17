use super::{FooterText, HeaderText, HeroText, SectionInfo, Texts};

pub const TEXTS: Texts = Texts {
    page_title: "Кошкодiвка",
    nick: "Kowkodivka",

    header: HeaderText {
        clock_label: "UTC+7",
        nav_label: "Разделы",
        lang_button: "Switch to English",
    },

    hero: HeroText {
        index_label: "Индекс",
        roles: &["Студент", "Программист", "Физик"],
        first_name: "Юрий",
        last_name: "Хошин",
        intro: "«Три кварка для мистера Марка!» — Джеймс Джойс",
        primary_action: "Написать",
        secondary_action: "Проекты",
        photo_alt: "аватар",
        coordinates: "56°29′ N, 84°57′ E",
        city: "Томск",
        wordmark: "Kowkodivka",
    },

    sections: [
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
    ],

    footer: FooterText {
        to_top: "наверх"
    },
};
