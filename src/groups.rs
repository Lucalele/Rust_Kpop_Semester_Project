use chrono::NaiveDate;
use diesel::prelude::*;

use crate::schema::{
    group_companies,
    group_labels,
    idol_groups,
    project_group_parents,
    project_groups,
    subunits,
};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_groups)]
#[diesel(primary_key(group_id))]
pub struct IdolGroup {
    pub group_id: i32,
    pub group_name: String,
    pub debut_date: Option<NaiveDate>,
    pub gender: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = idol_groups)]
pub struct NewIdolGroup<'a> {
    pub group_name: &'a str,
    pub debut_date: Option<NaiveDate>,
    pub gender: &'a str,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = subunits)]
#[diesel(primary_key(subunit_id))]
#[diesel(belongs_to(IdolGroup, foreign_key = parent_group_id))]
pub struct Subunit {
    pub subunit_id: i32,
    pub subunit_name: String,

    // A subunit always belongs to exactly one parent group
    pub parent_group_id: i32,

    pub debut_date: Option<NaiveDate>,

    // Restricted by the database to Male, Female, or CoEd
    pub gender: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = subunits)]
pub struct NewSubunit<'a> {
    pub subunit_name: &'a str,
    pub parent_group_id: i32,
    pub debut_date: Option<NaiveDate>,
    pub gender: &'a str,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = project_groups)]
#[diesel(primary_key(project_group_id))]
pub struct ProjectGroup {
    pub project_group_id: i32,
    pub project_group_name: String,
    pub debut_date: Option<NaiveDate>,
    pub gender: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = project_groups)]
pub struct NewProjectGroup<'a> {
    pub project_group_name: &'a str,
    pub debut_date: Option<NaiveDate>,
    pub gender: &'a str,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = group_companies)]
#[diesel(primary_key(group_id, company_id))]
pub struct GroupCompany {
    pub group_id: i32,
    pub company_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = group_labels)]
#[diesel(primary_key(group_id, label_id))]
pub struct GroupLabel {
    pub group_id: i32,
    pub label_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = project_group_parents)]
#[diesel(primary_key(project_group_id, parent_group_id))]
pub struct ProjectGroupParent {
    pub project_group_id: i32,
    pub parent_group_id: i32,
}

pub enum GroupGender {
    Male,
    Female,
    CoEd,
}