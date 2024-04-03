pub mod customers;
pub mod inventory;
pub mod tickets;

use std::fmt::Debug;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

enum ColumnFormat {
    Id,
    Currency,
    Date,
    None,
}

trait ViewFormat {
    fn format(&self, column_formatting: &ColumnFormat) -> Option<String>;
}

#[derive(Debug, Clone, Serialize)]
struct ViewCell<T: Debug + Clone + Serialize + ViewFormat> {
    base: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatted: Option<String>,
}

impl<T: Debug + Clone + Serialize + ViewFormat> ViewCell<T> {
    fn new(base: T, formatting: &ColumnFormat) -> Self {
        let formatted = base.format(formatting);

        Self { base, formatted }
    }
}

impl ViewFormat for u32 {
    fn format(&self, column_formatting: &ColumnFormat) -> Option<String> {
        match column_formatting {
            ColumnFormat::None => None,
            ColumnFormat::Id => Some(format!("#{:0>10}", self)),
            _ => panic!("Invalid formatting specifier for u32"),
        }
    }
}

impl ViewFormat for Decimal {
    fn format(&self, column_formatting: &ColumnFormat) -> Option<String> {
        match column_formatting {
            ColumnFormat::None => None,
            ColumnFormat::Currency => Some(format!("${self}")),
            _ => panic!("Invalid formatting specifier for Decimal"),
        }
    }
}

impl ViewFormat for String {
    fn format(&self, column_formatting: &ColumnFormat) -> Option<String> {
        match column_formatting {
            ColumnFormat::None => None,
            _ => panic!("Invalid formatting specifier for String"),
        }
    }
}

impl ViewFormat for NaiveDateTime {
    fn format(&self, column_formatting: &ColumnFormat) -> Option<String> {
        Some(match column_formatting {
            ColumnFormat::Date => self.format("%m/%d/%Y %H:%M").to_string(),
            _ => panic!("Invalid formatting specifier for NaiveDateTime"),
        })
    }
}

impl<T: ViewFormat> ViewFormat for Option<T> {
    fn format(&self, column_formatting: &ColumnFormat) -> Option<String> {
        match self {
            Some(value) => value.format(column_formatting),
            None => None,
        }
    }
}
