
pub struct Idol {
    pub idol_id: i32,

    // NOT the same as group gender
    pub idol_gender: String,

    // If true, they can have solo albums
    pub is_soloist: bool,
}

pub struct IdolName {
    // Unique ID for this specific name
    pub idol_name_id: i32,

    // Connects every name back to the same idol
    pub idol_id: i32,

    // Exact name used to identify the idol
    pub name: String,
}

pub struct IdolGroup {
    // Idol connected to the group
    pub idol_id: i32,

    // Group the idol is/was in
    pub group_id: i32,
}

pub struct IdolSubunit {
    // Idol connected to the subunit
    pub idol_id: i32,

    // Subunit the idol is/was in
    pub subunit_id: i32,
}

pub struct IdolProjectGroup {
    // Idol connected to the project group or collaboration
    pub idol_id: i32,

    // Project group/collaboration the idol is/was in
    pub project_group_id: i32,
}

pub struct IdolCompany {
    // Idol connected to the company
    pub idol_id: i32,

    // Company associated with the idol
    pub company_id: i32,
}

pub struct IdolLabel {
    // Idol connected to the label
    pub idol_id: i32,

    // Label associated with the idol
    pub label_id: i32,
}

