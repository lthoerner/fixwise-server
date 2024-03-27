use std::iter;
use std::sync::OnceLock;

use axum::Json;
use serde::{Deserialize, Serialize};

static CONFIG: OnceLock<DatabaseConfig> = OnceLock::new();

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    tables: Vec<TableConfig>,
    views: Vec<ViewConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TableConfig {
    name: String,
    primary_column: ColumnSchemaConfig,
    required_columns: Vec<ColumnSchemaConfig>,
    optional_columns: Vec<ColumnSchemaConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColumnSchemaConfig {
    name: String,
    data_type: String,
}

#[derive(Debug, Deserialize)]
struct ViewConfig {
    table: String,
    columns: Vec<ColumnViewConfig>,
}

#[derive(Debug, Deserialize)]
struct ColumnViewConfig {
    name: String,
    display_name: String,
    trimmable: bool,
    formatting: Option<ColumnFormattingConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnFormattingConfig {
    prefix: Option<String>,
    suffix: Option<String>,
    pad_length: Option<u32>,
}

#[derive(Serialize)]
pub struct FrontendTableView(Vec<FrontendColumnView>);

#[derive(Serialize)]
pub struct FrontendColumnView {
    name: String,
    data_type: String,
    display_name: String,
    trimmable: bool,
    formatting: Option<ColumnFormattingConfig>,
}

pub fn get_frontend_view(table_name: &str) -> Json<FrontendTableView> {
    let config = CONFIG.get().unwrap();
    let table = config.tables.iter().find(|t| t.name == table_name).unwrap();
    let view = config.views.iter().find(|v| v.table == table_name).unwrap();

    // TODO: Redundant work - maybe store the full set of columns elsewhere for reuse
    let table_columns = iter::once(&table.primary_column)
        .chain(&table.required_columns)
        .chain(&table.optional_columns)
        .collect::<Vec<&ColumnSchemaConfig>>();

    let column_views = view
        .columns
        .iter()
        .map(|col| FrontendColumnView {
            name: col.name.clone(),
            data_type: table_columns
                .iter()
                .find(|c| c.name == col.name)
                .unwrap()
                .data_type
                .clone(),
            display_name: col.display_name.clone(),
            trimmable: col.trimmable,
            formatting: col.formatting.clone(),
        })
        .collect::<Vec<FrontendColumnView>>();

    Json(FrontendTableView(column_views))
}

pub fn create_setup_script() -> String {
    let config: DatabaseConfig = toml::from_str(include_str!("../database/config.toml")).unwrap();

    let mut script = String::new();

    for table in &config.tables {
        script.push_str(&format!("DROP TABLE IF EXISTS {} CASCADE;\n", table.name));
    }

    for view in &config.views {
        script.push_str(&format!(
            "DROP VIEW IF EXISTS {}_view CASCADE;\n",
            view.table
        ));
    }

    for table in &config.tables {
        let TableConfig {
            name: _,
            primary_column,
            required_columns,
            optional_columns,
        } = table;

        let primary_column_declaration = generate_column_declaration(primary_column, true, true);

        let required_column_declarations = required_columns
            .iter()
            .map(|col| generate_column_declaration(col, false, true))
            .collect::<Vec<String>>();

        let optional_column_declarations = optional_columns
            .iter()
            .map(|col| generate_column_declaration(col, false, false))
            .collect::<Vec<String>>();

        let column_declarations = iter::once(primary_column_declaration)
            .chain(required_column_declarations)
            .chain(optional_column_declarations)
            .collect::<Vec<String>>();

        script.push_str(&format!(
            "\nCREATE TABLE {} (\n{}\n);\n",
            table.name,
            column_declarations.join(",\n")
        ));
    }

    for view in &config.views {
        script.push_str(&generate_view_declaration(view));
    }

    fn generate_column_declaration(
        column: &ColumnSchemaConfig,
        primary_column: bool,
        required: bool,
    ) -> String {
        format!(
            "    {:<16}{}{}",
            column.name,
            map_type(&column.data_type, primary_column),
            if primary_column {
                " PRIMARY KEY"
            } else if required {
                " NOT NULL"
            } else {
                ""
            }
        )
        .to_owned()
    }

    fn map_type(type_name: &str, primary_column: bool) -> String {
        if primary_column && type_name == "integer" {
            return "serial".to_owned();
        }

        match type_name {
            "integer" => "integer",
            "decimal" => "numeric(1000, 2)",
            "string" => "text",
            _ => panic!("Unknown type in schema config"),
        }
        .to_owned()
    }

    fn generate_view_declaration(view: &ViewConfig) -> String {
        let ViewConfig { table, columns } = view;

        let column_names = columns
            .iter()
            .map(|col| col.name.clone())
            .collect::<Vec<String>>();

        let first_line = format!("CREATE VIEW {}_view AS", table);
        let second_line = format!("    SELECT {} FROM {};", column_names.join(", "), table);

        format!("\n{}\n{}\n", first_line, second_line)
    }

    CONFIG.get_or_init(|| config);

    script
}
