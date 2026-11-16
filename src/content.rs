pub const NICK: &str = "Kowkodivka";

#[derive(Clone, Copy, PartialEq)]
pub struct SectionInfo {
    pub id: &'static str,
    pub number: &'static str,
    pub label: &'static str,
    pub title: &'static str,
    pub accent: &'static str,
    pub title_end: &'static str,
}

pub const SECTIONS: [SectionInfo; 0] = [];
