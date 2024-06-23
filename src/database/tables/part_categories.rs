use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct PartCategoriesDatabaseTable {
    rows: Vec<PartCategoriesDatabaseTableRow>,
}

impl DatabaseEntity for PartCategoriesDatabaseTable {
    type Row = PartCategoriesDatabaseTableRow;
    const ENTITY_NAME: &'static str = "part_categories";
    const PRIMARY_COLUMN_NAME: &'static str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn rows(self) -> Vec<Self::Row> {
        self.rows
    }

    fn borrow_rows(&self) -> &[Self::Row] {
        &self.rows
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct PartCategoriesDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl IdentifiableRow for PartCategoriesDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}
