use chrono::NaiveDate;

pub struct IdolGroup {
    pub group_id: i32,
    pub group_name: String,
    pub debut_date: NaiveDate,
    pub gender: GroupGender,
}

pub struct Subunit {
    pub subunit_id: i32,
    pub subunit_name: String,

    // A subunit always belongs to exactly one parent group
    pub parent_group_id: i32,

    pub debut_date: NaiveDate,

    // Same enum as GroupGender
    pub gender: GroupGender,
}

pub struct ProjectGroup {
    pub project_group_id: i32,
    pub project_group_name: String,
    pub debut_date: NaiveDate,
    pub gender: GroupGender,
}

pub struct GroupCompany {
    // Group associated with the company
    pub group_id: i32,

    // Company associated with the group
    pub company_id: i32,
}

pub struct GroupLabel {
    // Group associated with the label
    pub group_id: i32,

    // Label associated with the group
    pub label_id: i32,
}

pub struct ProjectGroupParent {
    // Project group
    pub project_group_id: i32,

    // Parent group connected to the project group
    pub parent_group_id: i32,
}

pub enum GroupGender {
    Male,
    Female,
    CoEd,
}