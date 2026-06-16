pub struct Album {
    pub album_id: u32,
    pub title: String,
    pub artist: String,
    pub release_date: NaiveDate,
    pub language: String,
    pub versions: Vec<String>,
}

pub struct Group{
    pub group_id: u32,
    pub group_name: String,
    pub debut_date: NaiveDate,
    pub gender: u8, //set male to 0, female to 1, co-ed to 2
    pub comapany_name: Vec<String>,
    pub label_name: Vec<String>,
}

pub struct Idol{
    pub idol_id: u32,
    pub idol_gender: u8, //0 for male, 1 for female
}

pub struct IdolName{
    pub idol_id: u32,
    pub name: String,
    pub language: u8, // 0 = Hangul, 1 = Latin, 2 = Japanese, 3 = Mandarin
}





