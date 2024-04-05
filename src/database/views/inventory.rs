use rust_decimal::Decimal;

use crate::database::DatabaseEntity;

pub struct InventoryDatabaseView {
    rows: Vec<InventoryDatabaseViewRow>,
}

impl DatabaseEntity for InventoryDatabaseView {
    type Row = InventoryDatabaseViewRow;
    const ENTITY_NAME: &'static str = "inventory_view";
    const PRIMARY_COLUMN_NAME: &'static str = "sku";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn rows(self) -> Vec<Self::Row> {
        self.rows
    }
}

#[derive(sqlx::FromRow)]
pub struct InventoryDatabaseViewRow {
    pub sku: i32,
    pub name: String,
    pub count: i32,
    pub cost: Decimal,
    pub price: Decimal,
}
