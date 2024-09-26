use proc_macros::Relation;

#[derive(Relation)]
#[relation(relation_name = "devices_view", primary_key = "id")]
pub struct DevicesView {
    records: Vec<DevicesViewRecord>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct DevicesViewRecord {
    pub id: i32,
    pub model: String,
    pub owner: Option<String>,
}
