use chrono::NaiveDate;
use diesel::prelude::*;

use crate::schema::{
    group_companies, group_labels, idol_groups, project_group_parents, project_groups, subunits,
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

pub fn insert_group(
    connection: &mut SqliteConnection,
    group_name: &str,
    debut_date: Option<NaiveDate>,
    gender: &str,
) -> QueryResult<usize> {
    let new_group = NewIdolGroup {
        group_name,
        debut_date,
        gender,
    };

    diesel::insert_into(idol_groups::table)
        .values(&new_group)
        .execute(connection)
}

pub fn load_groups(connection: &mut SqliteConnection) -> QueryResult<Vec<IdolGroup>> {
    idol_groups::table
        .select(IdolGroup::as_select())
        .order(idol_groups::group_id.asc())
        .load(connection)
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

pub fn insert_subunit(
    connection: &mut SqliteConnection,
    subunit_name: &str,
    parent_group_id: i32,
    debut_date: Option<NaiveDate>,
    gender: &str,
) -> QueryResult<usize> {
    let new_subunit = NewSubunit {
        subunit_name,
        parent_group_id,
        debut_date,
        gender,
    };

    diesel::insert_into(subunits::table)
        .values(&new_subunit)
        .execute(connection)
}

pub fn load_subunits(connection: &mut SqliteConnection) -> QueryResult<Vec<Subunit>> {
    subunits::table
        .select(Subunit::as_select())
        .order(subunits::subunit_id.asc())
        .load(connection)
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

pub fn insert_project_group(
    connection: &mut SqliteConnection,
    project_group_name: &str,
    debut_date: Option<NaiveDate>,
    gender: &str,
) -> QueryResult<usize> {
    let new_project_group = NewProjectGroup {
        project_group_name,
        debut_date,
        gender,
    };

    diesel::insert_into(project_groups::table)
        .values(&new_project_group)
        .execute(connection)
}

pub fn load_project_groups(connection: &mut SqliteConnection) -> QueryResult<Vec<ProjectGroup>> {
    project_groups::table
        .select(ProjectGroup::as_select())
        .order(project_groups::project_group_id.asc())
        .load(connection)
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = group_companies)]
#[diesel(primary_key(group_id, company_id))]
pub struct GroupCompany {
    pub group_id: i32,
    pub company_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = group_companies)]
pub struct NewGroupCompany {
    pub group_id: i32,
    pub company_id: i32,
}

pub fn insert_group_company(
    connection: &mut SqliteConnection,
    group_id: i32,
    company_id: i32,
) -> QueryResult<usize> {
    let relationship = NewGroupCompany {
        group_id,
        company_id,
    };

    diesel::insert_into(group_companies::table)
        .values(&relationship)
        .execute(connection)
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = group_labels)]
#[diesel(primary_key(group_id, label_id))]
pub struct GroupLabel {
    pub group_id: i32,
    pub label_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = group_labels)]
pub struct NewGroupLabel {
    pub group_id: i32,
    pub label_id: i32,
}

pub fn insert_group_label(
    connection: &mut SqliteConnection,
    group_id: i32,
    label_id: i32,
) -> QueryResult<usize> {
    let relationship = NewGroupLabel { group_id, label_id };

    diesel::insert_into(group_labels::table)
        .values(&relationship)
        .execute(connection)
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = project_group_parents)]
#[diesel(primary_key(project_group_id, parent_group_id))]
pub struct ProjectGroupParent {
    pub project_group_id: i32,
    pub parent_group_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = project_group_parents)]
pub struct NewProjectGroupParent {
    pub project_group_id: i32,
    pub parent_group_id: i32,
}

pub fn insert_project_group_parent(
    connection: &mut SqliteConnection,
    project_group_id: i32,
    parent_group_id: i32,
) -> QueryResult<usize> {
    let relationship = NewProjectGroupParent {
        project_group_id,
        parent_group_id,
    };

    diesel::insert_into(project_group_parents::table)
        .values(&relationship)
        .execute(connection)
}

pub enum GroupGender {
    Male,
    Female,
    CoEd,
}
