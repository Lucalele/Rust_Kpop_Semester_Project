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