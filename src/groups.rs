use crate::enums::Language;

pub struct Idol {
    pub idol_id: u32,

    // NOT the same as group gender
    pub idol_gender: Gender,

    // Groups the idol is/was in
    pub group_ids: Vec<u32>,

    // Subunits the idol is/was in
    pub subunit_ids: Vec<u32>,

    // Project groups/collabs the idol is/was in
    pub project_group_ids: Vec<u32>,

    // If true, they can have solo albums
    pub is_soloist: bool,

    // Optional, mostly for filtering
    pub company_ids: Vec<u32>,
    pub label_ids: Vec<u32>,
}

pub struct IdolName {
    pub idol_name_id: u32,
    pub idol_id: u32,
    pub display_name: String,
    pub language: Language,
    pub name_type: NameType,
}

pub struct NameAlias {
    pub idol_alias_id: u32,
    pub idol_id: u32,
    pub alias: String,
}

pub enum Gender {
    Male,
    Female,
}

pub enum NameType {
    StageName,
    BirthName,
    EnglishName,
    JapaneseName,
    ChineseName,
    Nickname,
}