pub struct Idol {
    pub idol_id: i32,

    // NOT the same as group gender
    pub idol_gender: String,

    // Groups the idol is/was in
    pub group_ids: Vec<i32>,

    // Subunits the idol is/was in
    pub subunit_ids: Vec<i32>,

    // Project groups/collabs the idol is/was in
    pub project_group_ids: Vec<i32>,

    // If true, they can have solo albums
    pub is_soloist: bool,

    // Optional, mostly for filtering
    pub company_ids: Vec<i32>,

    // Optional, mostly for filtering
    pub label_ids: Vec<i32>,
}

pub struct IdolName {
    // Unique ID for this specific name
    pub idol_name_id: i32,

    // Connects every name back to the same idol
    pub idol_id: i32,

    // Exact name used to identify the idol
    pub name: String,
}