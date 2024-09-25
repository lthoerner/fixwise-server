use proc_macros::Relation;

#[derive(Relation)]
#[relation(
    relation_name = "vendors_view",
    primary_key = "id",
    foreign_key_name = "PLACEHOLDER"
)]
pub struct VendorsView {
    records: Vec<VendorsViewRecord>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct VendorsViewRecord {
    pub id: i32,
    pub display_name: String,
}
