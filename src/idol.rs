use diesel::prelude::*;

use crate::schema::{
    idol_companies,
    idol_group_memberships,
    idol_labels,
    idol_names,
    idol_project_group_memberships,
    idol_subunit_memberships,
    idols,
};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idols)]
#[diesel(primary_key(idol_id))]
pub struct Idol {
    pub idol_id: i32,

    // NOT the same as group gender
    pub idol_gender: String,

    // If true, they can have solo albums
    pub is_soloist: bool,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = idol_names)]
#[diesel(primary_key(idol_name_id))]
#[diesel(belongs_to(Idol, foreign_key = idol_id))]
pub struct IdolName {
    // Unique ID for this specific name
    pub idol_name_id: i32,

    // Connects every name back to the same idol
    pub idol_id: i32,

    // Exact name used to identify the idol
    pub name: String,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_group_memberships)]
#[diesel(primary_key(idol_id, group_id))]
pub struct IdolGroupMembership {
    pub idol_id: i32,
    pub group_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_subunit_memberships)]
#[diesel(primary_key(idol_id, subunit_id))]
pub struct IdolSubunitMembership {
    pub idol_id: i32,
    pub subunit_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_project_group_memberships)]
#[diesel(primary_key(idol_id, project_group_id))]
pub struct IdolProjectGroupMembership {
    pub idol_id: i32,
    pub project_group_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_companies)]
#[diesel(primary_key(idol_id, company_id))]
pub struct IdolCompany {
    pub idol_id: i32,
    pub company_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_labels)]
#[diesel(primary_key(idol_id, label_id))]
pub struct IdolLabel {
    pub idol_id: i32,
    pub label_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = idols)]
pub struct NewIdol<'a> {
    pub idol_gender: &'a str,
    pub is_soloist: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = idol_names)]
pub struct NewIdolName<'a> {
    pub idol_id: i32,
    pub name: &'a str,
}