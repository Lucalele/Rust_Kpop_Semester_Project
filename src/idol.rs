use diesel::prelude::*;

use crate::schema::{
    idol_companies, idol_group_memberships, idol_labels, idol_names,
    idol_project_group_memberships, idol_subunit_memberships, idols,
};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idols)]
#[diesel(primary_key(idol_id))]
pub struct Idol {
    pub idol_id: i32,

    // NOT the same as group gender
    pub idol_gender: String,

    // If true, they can have solo albums
    pub is_soloist: Option<bool>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = idols)]
pub struct NewIdol<'a> {
    pub idol_gender: &'a str,
    pub is_soloist: Option<bool>,
}

pub fn insert_idol(
    connection: &mut SqliteConnection,
    idol_gender: &str,
    is_soloist: Option<bool>,
) -> QueryResult<usize> {
    let new_idol = NewIdol {
        idol_gender,
        is_soloist,
    };

    diesel::insert_into(idols::table)
        .values(&new_idol)
        .execute(connection)
}

pub fn load_idols(connection: &mut SqliteConnection) -> QueryResult<Vec<Idol>> {
    idols::table
        .select(Idol::as_select())
        .order(idols::idol_id.asc())
        .load(connection)
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

//possibly unused
#[derive(Debug, Insertable)]
#[diesel(table_name = idol_names)]
pub struct NewIdolName<'a> {
    pub idol_id: i32,
    pub name: &'a str,
}

pub fn insert_idol_name(
    connection: &mut SqliteConnection,
    idol_id_value: i32,
    idol_name_value: &str,
) -> QueryResult<usize> {
    diesel::insert_into(idol_names::table)
        .values((
            idol_names::idol_id.eq(idol_id_value),
            idol_names::name.eq(idol_name_value),
        ))
        .execute(connection)
}

pub fn load_idol_names(connection: &mut SqliteConnection) -> QueryResult<Vec<IdolName>> {
    idol_names::table
        .select(IdolName::as_select())
        .order(idol_names::idol_name_id.asc())
        .load(connection)
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_group_memberships)]
#[diesel(primary_key(idol_id, group_id))]
pub struct IdolGroupMembership {
    pub idol_id: i32,
    pub group_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = idol_group_memberships)]
pub struct NewIdolGroupMembership {
    pub idol_id: i32,
    pub group_id: i32,
}

pub fn insert_idol_group_membership(
    connection: &mut SqliteConnection,
    idol_id: i32,
    group_id: i32,
) -> QueryResult<usize> {
    let membership = NewIdolGroupMembership { idol_id, group_id };

    diesel::insert_into(idol_group_memberships::table)
        .values(&membership)
        .execute(connection)
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_subunit_memberships)]
#[diesel(primary_key(idol_id, subunit_id))]
pub struct IdolSubunitMembership {
    pub idol_id: i32,
    pub subunit_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = idol_subunit_memberships)]
pub struct NewIdolSubunitMembership {
    pub idol_id: i32,
    pub subunit_id: i32,
}

pub fn insert_idol_subunit_membership(
    connection: &mut SqliteConnection,
    idol_id: i32,
    subunit_id: i32,
) -> QueryResult<usize> {
    let membership = NewIdolSubunitMembership {
        idol_id,
        subunit_id,
    };

    diesel::insert_into(idol_subunit_memberships::table)
        .values(&membership)
        .execute(connection)
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_project_group_memberships)]
#[diesel(primary_key(idol_id, project_group_id))]
pub struct IdolProjectGroupMembership {
    pub idol_id: i32,
    pub project_group_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = idol_project_group_memberships)]
pub struct NewIdolProjectGroupMembership {
    pub idol_id: i32,
    pub project_group_id: i32,
}

pub fn insert_idol_project_group_membership(
    connection: &mut SqliteConnection,
    idol_id: i32,
    project_group_id: i32,
) -> QueryResult<usize> {
    let membership = NewIdolProjectGroupMembership {
        idol_id,
        project_group_id,
    };

    diesel::insert_into(idol_project_group_memberships::table)
        .values(&membership)
        .execute(connection)
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_companies)]
#[diesel(primary_key(idol_id, company_id))]
pub struct IdolCompany {
    pub idol_id: i32,
    pub company_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = idol_companies)]
pub struct NewIdolCompany {
    pub idol_id: i32,
    pub company_id: i32,
}

pub fn insert_idol_company(
    connection: &mut SqliteConnection,
    idol_id: i32,
    company_id: i32,
) -> QueryResult<usize> {
    let relationship = NewIdolCompany {
        idol_id,
        company_id,
    };

    diesel::insert_into(idol_companies::table)
        .values(&relationship)
        .execute(connection)
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = idol_labels)]
#[diesel(primary_key(idol_id, label_id))]
pub struct IdolLabel {
    pub idol_id: i32,
    pub label_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = idol_labels)]
pub struct NewIdolLabel {
    pub idol_id: i32,
    pub label_id: i32,
}

pub fn insert_idol_label(
    connection: &mut SqliteConnection,
    idol_id: i32,
    label_id: i32,
) -> QueryResult<usize> {
    let relationship = NewIdolLabel { idol_id, label_id };

    diesel::insert_into(idol_labels::table)
        .values(&relationship)
        .execute(connection)
}
