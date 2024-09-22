pub mod processed;
pub mod raw;
pub mod utils;

use std::fmt::Debug;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::database::shared_models::TicketStatus;

#[derive(Serialize)]
struct TagOption {
    name: &'static str,
    color: CssColor,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum CssColor {
    Preset {
        name: &'static str,
        opacity: f32,
    },
    #[allow(dead_code)]
    Rgba {
        r: u8,
        g: u8,
        b: u8,
        a: f32,
    },
}

enum ColumnFormat {
    Id,
    Currency,
    Date,
    Tag,
    None,
}

#[derive(Serialize)]
struct FrontendColumnMetadata {
    data_type: FrontendDataType,
    display: FrontendColumnDisplay,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum FrontendColumnDisplay {
    Text {
        name: &'static str,
        trimmable: bool,
    },
    Tag {
        name: &'static str,
        options: &'static [TagOption],
    },
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum FrontendDataType {
    Integer,
    Decimal,
    String,
    Timestamp,
    Tag,
}

trait ViewFormat {
    fn format(&self, column_formatting: &ColumnFormat) -> Option<String>;
}

#[derive(Debug, Clone, Serialize)]
struct ViewCell<T: Debug + Clone + Serialize + ViewFormat> {
    value: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatted: Option<String>,
}

impl<T: Debug + Clone + Serialize + ViewFormat> ViewCell<T> {
    fn new(value: T, formatting: &ColumnFormat) -> Self {
        let formatted = value.format(formatting);

        Self { value, formatted }
    }
}

impl ViewFormat for i32 {
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

// ? Should there be a trait or some way to write this in a less-specific way?
impl ViewFormat for TicketStatus {
    fn format(&self, column_formatting: &ColumnFormat) -> Option<String> {
        Some(match column_formatting {
            ColumnFormat::Tag => match self {
                TicketStatus::New => "New".to_string(),
                TicketStatus::WaitingForParts => "Waiting for Parts".to_string(),
                TicketStatus::WaitingForCustomer => "Waiting for Customer".to_string(),
                TicketStatus::InRepair => "In Repair".to_string(),
                TicketStatus::ReadyForPickup => "Ready for Pickup".to_string(),
                TicketStatus::Closed => "Closed".to_string(),
            },
            _ => panic!("Invalid formatting specifier for ApiTag"),
        })
    }
}

impl<T: ViewFormat> ViewFormat for Option<T> {
    fn format(&self, column_formatting: &ColumnFormat) -> Option<String> {
        match self {
            Some(value) => value.format(column_formatting),
            None => Some("N/A".to_owned()),
        }
    }
}
