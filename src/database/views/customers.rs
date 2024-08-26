use proc_macros::DatabaseEntity;

#[derive(DatabaseEntity)]
#[entity(entity_name = "customers", primary_column = "id")]
pub struct CustomersDatabaseView {
    rows: Vec<CustomersDatabaseViewRow>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct CustomersDatabaseViewRow {
    pub id: i32,
    pub name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}
