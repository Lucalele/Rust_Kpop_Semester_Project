use chrono::NaiveDate;


pub struct IdolGroup{
    pub group_id: u32,
    pub group_name: String, 
    pub debut_date: NaiveDate,
    pub gender: GroupGender, 
    pub company_ids: Vec<u32>,
    pub label_ids: Vec<u32>,
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

pub struct ProjectGroup {
    pub project_group_id: u32,
    pub project_group_name: String,
    pub parent_group_ids: Vec<u32>,
    pub debut_date: NaiveDate,
    pub gender: GroupGender,
}

pub enum GroupGender {
    Male,
    Female,
    CoEd,
}
