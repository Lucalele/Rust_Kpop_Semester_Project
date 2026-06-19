pub struct Album {
    pub album_id: u32,
    pub title: String,
    pub artist_id: u32,
    pub release_date: NaiveDate,
    pub language: Language,
    pub versions: Vec<String>, //THIS WILL BE COLLAPSED INTO A SINGLE OPTION
}

pub struct Group{
    pub group_id: u32,
    pub group_name: String,
    pub debut_date: NaiveDate,
    pub gender: GroupGender, 
    pub company_ids: Vec<u32>,
    pub label_ids: Vec<u32>,
}

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

pub struct Label {
    pub label_id: u32,
    pub label_name: String,
}

pub struct Company{
    pub company_id: u32,
    pub company_name: String,
}

pub struct Subunit {
    pub subunit_id: u32,
    pub subunit_name: String,

    // A subunit always belongs to exactly one parent group
    pub parent_group_id: u32,

    pub debut_date: NaiveDate,

    // Same enum as GroupGender
    pub gender: GroupGender,
}
