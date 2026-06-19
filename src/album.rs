pub struct Album {
    pub album_id: u32,
    pub title: String,
    pub artist_id: u32,
    pub release_date: NaiveDate,
    pub language: Language,
    pub versions: Vec<String>, //THIS WILL BE COLLAPSED INTO A SINGLE OPTION
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




pub enum Language {
    Hangul,
    Latin,
    Japanese,
    Mandarin,
}


