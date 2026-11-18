use super::{FooterText, HeaderText, HeroText, SectionInfo, Texts};

pub const TEXTS: Texts = Texts {
    page_title: "Kowkodivka",
    nick: "Kowkodivka",

    header: HeaderText {
        clock_label: "UTC+7",
        nav_label: "Sections",
        theme_button: "Change theme",
        lang_button: "Переключить на русский",
    },

    hero: HeroText {
        index_label: "Index",
        roles: &["Student", "(Not a) Programmer", "(Not a) Physicist"],
        first_name: "Yuriy",
        last_name: "Khoshin",
        intro: "“Three quarks for Muster Mark!” — James Joyce",
        primary_action: "Get in touch",
        secondary_action: "Projects",
        photo_alt: "avatar",
        coordinates: "56°29′ N, 84°57′ E",
        city: "Tomsk",
        wordmark: "Kowkodivka",
    },

    sections: [
        SectionInfo {
            id: "about",
            number: "01",
            label: "about",
            title: "...",
            accent: "...",
            title_end: "...",
        },
        SectionInfo {
            id: "stack",
            number: "02",
            label: "stack",
            title: "Table of ",
            accent: "elements",
            title_end: "",
        },
        SectionInfo {
            id: "projects",
            number: "03",
            label: "projects",
            title: "One ",
            accent: "in progress",
            title_end: ", the rest are ahead",
        },
        SectionInfo {
            id: "setup",
            number: "04",
            label: "setup",
            title: "Equipment ",
            accent: "datasheet",
            title_end: "",
        },
        SectionInfo {
            id: "music",
            number: "05",
            label: "music",
            title: "On ",
            accent: "repeat",
            title_end: "",
        },
        SectionInfo {
            id: "games",
            number: "06",
            label: "games",
            title: "Now ",
            accent: "playing",
            title_end: "",
        },
        SectionInfo {
            id: "contact",
            number: "07",
            label: "contact",
            title: "...",
            accent: "...",
            title_end: "",
        },
    ],

    footer: FooterText {
        to_top: "back to top",
    },
};
