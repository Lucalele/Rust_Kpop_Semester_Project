use chrono::NaiveDate;

pub struct Album {
    pub album_id: u32,
    pub title: String,
    pub artist_id: u32,
    pub release_date: NaiveDate,
    pub language: Language,
    pub versions: Vec<String>, //THIS WILL BE COLLAPSED INTO A SINGLE OPTION
}



pub enum Language {
    Hangul,
    Latin,
    Japanese,
    Mandarin,
}


