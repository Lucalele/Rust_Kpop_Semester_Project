use crate::enums::Language;

pub struct Idol{
    pub idol_id: u32,

    //NOT THE SAME AS GROUP GENDER
    pub idol_gender: Gender,

    // if 0, soloist
    pub group_id: Vec<u32>,

    // most idols are not in subunits
    pub subunit_id: Vec<u32>,

    // 0 = indie
    pub company_id: Vec<u32>,

    // 0 = indie or company has no labels
    pub label_id: Vec<u32>,
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

