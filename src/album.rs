use chrono::NaiveDate;



pub struct Album {
    pub album_id: i32,
    pub title: String,
    pub artist_id: i32,
    pub release_date: NaiveDate,
    pub language: String,
    pub versions: Vec<String>, //THIS WILL BE COLLAPSED INTO A SINGLE OPTION
}






