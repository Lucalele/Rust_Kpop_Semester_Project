pub struct Album {
    pub album_id: u32,
    pub title: String,
    pub artist_id: u32,
    pub release_date: NaiveDate,
    pub language: Language,
    pub versions: Vec<String>,
}

pub struct Group{
    pub group_id: u32,
    pub group_name: String,
    pub debut_date: NaiveDate,
    pub gender: GroupGender, //set male to 0, female to 1, co-ed to 2
    pub comapany_name: Vec<String>,
    pub label_name: Vec<String>,
}

pub struct Idol{
    pub idol_id: u32,
    pub idol_gender: Gender, //0 for male, 1 for female
}

pub struct IdolName {
    pub idol_name_id: u32,
    pub idol_id: u32,
    pub display_name: String,
    pub language: Language,
    pub name_type: NameType,
}

pub struct NameAlias {
    pub idol_name_id: u32,
    pub alias: String,
}

pub enum GroupGender {
    Male,
    Female,
    CoEd,
}

pub enum Gender {
    Male,
    Female,
}

pub enum Language {
    Hangul,
    Latin,
    Japanese,
    Mandarin,
}

