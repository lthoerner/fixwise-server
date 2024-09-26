use proc_macros::Relation;

#[derive(Relation)]
#[relation(relation_name = "customers", primary_key = "id")]
pub struct CustomersView {
    records: Vec<CustomersViewRecord>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct CustomersViewRecord {
    pub id: i32,
    pub name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}
