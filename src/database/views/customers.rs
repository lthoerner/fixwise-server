use serde::Serialize;

use proc_macros::Relation;

#[derive(Relation, Serialize)]
#[relation(relation_name = "customers", primary_key = "id")]
pub struct CustomersView {
    records: Vec<CustomersViewRecord>,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct CustomersViewRecord {
    pub id: i32,
    pub name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}
