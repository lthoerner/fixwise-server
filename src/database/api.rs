use std::fmt::Debug;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use super::views::ColumnFormatting;

#[derive(Debug, Clone, Serialize)]
pub(super) struct CellValue<T: Debug + Clone + Serialize + DatabaseFormat> {
    pub(super) base: T,
    formatted: Option<String>,
}

impl<T: Debug + Clone + Serialize + DatabaseFormat> CellValue<T> {
    pub(super) fn new(base: T, formatting: Option<&ColumnFormatting>) -> Self {
        let formatted = match formatting {
            Some(formatting) => base.format(formatting),
            None => None,
        };

        Self { base, formatted }
    }
}

pub(super) trait DatabaseFormat {
    fn format(&self, column_formatting: &ColumnFormatting) -> Option<String>;
}

impl DatabaseFormat for i32 {
    fn format(&self, column_formatting: &ColumnFormatting) -> Option<String> {
        let mut formatted = self.to_string();
        if let Some(pad_length) = column_formatting.pad_length {
            formatted = format!("{:0>width$}", formatted, width = pad_length);
        }

        Some(format!(
            "{}{formatted}{}",
            column_formatting.prefix.as_ref().unwrap_or(&String::new()),
            column_formatting.suffix.as_ref().unwrap_or(&String::new())
        ))
    }
}

impl DatabaseFormat for Decimal {
    fn format(&self, column_formatting: &ColumnFormatting) -> Option<String> {
        Some(format!(
            "{}{self}{}",
            column_formatting.prefix.as_ref().unwrap_or(&String::new()),
            column_formatting.suffix.as_ref().unwrap_or(&String::new())
        ))
    }
}

impl DatabaseFormat for String {
    fn format(&self, column_formatting: &ColumnFormatting) -> Option<String> {
        Some(format!(
            "{}{self}{}",
            column_formatting.prefix.as_ref().unwrap_or(&String::new()),
            column_formatting.suffix.as_ref().unwrap_or(&String::new())
        ))
    }
}

impl DatabaseFormat for NaiveDateTime {
    fn format(&self, column_formatting: &ColumnFormatting) -> Option<String> {
        let formatted = self.format("%m/%d/%Y %H:%M");
        Some(format!(
            "{}{formatted}{}",
            column_formatting.prefix.as_ref().unwrap_or(&String::new()),
            column_formatting.suffix.as_ref().unwrap_or(&String::new()),
        ))
    }
}
