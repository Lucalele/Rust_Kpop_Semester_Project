use diesel::prelude::*;

use crate::schema::{companies, labels};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = companies)]
#[diesel(primary_key(company_id))]
pub struct Company {
    pub company_id: i32,
    pub company_name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = companies)]
pub struct NewCompany<'a> {
    pub company_name: &'a str,
}

pub fn insert_company(connection: &mut SqliteConnection, company_name: &str) -> QueryResult<usize> {
    let new_company = NewCompany { company_name };

    diesel::insert_into(companies::table)
        .values(&new_company)
        .on_conflict(companies::company_name)
        .do_nothing()
        .execute(connection)
}

pub fn load_companies(connection: &mut SqliteConnection) -> QueryResult<Vec<Company>> {
    companies::table
        .select(Company::as_select())
        .order(companies::company_id.asc())
        .load(connection)
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = labels)]
#[diesel(primary_key(label_id))]
#[diesel(belongs_to(Company, foreign_key = company_id))]
pub struct Label {
    pub label_id: i32,
    pub label_name: String,
    pub company_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = labels)]
pub struct NewLabel<'a> {
    pub label_name: &'a str,
    pub company_id: i32,
}

pub fn insert_label(
    connection: &mut SqliteConnection,
    label_name: &str,
    company_id: i32,
) -> QueryResult<usize> {
    let new_label = NewLabel {
        label_name,
        company_id,
    };

    diesel::insert_into(labels::table)
        .values(&new_label)
        .on_conflict((labels::label_name, labels::company_id))
        .do_nothing()
        .execute(connection)
}

pub fn load_labels(connection: &mut SqliteConnection) -> QueryResult<Vec<Label>> {
    labels::table
        .select(Label::as_select())
        .order(labels::label_id.asc())
        .load(connection)
}
