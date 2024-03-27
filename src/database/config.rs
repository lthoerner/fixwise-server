use std::iter;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

pub(super) static CONFIG: OnceLock<DatabaseConfig> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub(super) struct DatabaseConfig {
    pub(super) tables: Vec<TableConfig>,
    pub(super) views: Vec<TableViewConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct TableConfig {
    pub(super) name: String,
    pub(super) primary_column: ColumnSchemaConfig,
    pub(super) required_columns: Vec<ColumnSchemaConfig>,
    pub(super) optional_columns: Vec<ColumnSchemaConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ColumnSchemaConfig {
    pub(super) name: String,
    pub(super) data_type: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct TableViewConfig {
    pub(super) table: String,
    pub(super) columns: Vec<ColumnViewConfig>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ColumnViewConfig {
    pub(super) name: String,
    pub(super) display_name: String,
    pub(super) trimmable: bool,
    pub(super) formatting: Option<ColumnFormattingConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ColumnFormattingConfig {
    prefix: Option<String>,
    suffix: Option<String>,
    pad_length: Option<u32>,
}

pub fn create_setup_script() -> String {
    let config: DatabaseConfig =
        toml::from_str(include_str!("../../database/config.toml")).unwrap();

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

    fn generate_view_declaration(view: &TableViewConfig) -> String {
        let TableViewConfig { table, columns } = view;

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
