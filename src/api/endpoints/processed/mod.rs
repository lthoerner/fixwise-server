pub mod customers;
pub mod device_models;
pub mod devices;
pub mod invoices;
pub mod parts;
pub mod products;
pub mod services;
pub mod tickets;
pub mod vendors;

#[allow(unused_imports)]
pub use {
    customers::{CustomersResource, CustomersResourceRecord},
    device_models::{DeviceModelsResource, DeviceModelsResourceRecord},
    devices::{DevicesResource, DevicesResourceRecord},
    invoices::{InvoicesResource, InvoicesResourceRecord},
    parts::{PartsResource, PartsResourceRecord},
    products::{ProductsResource, ProductsResourceRecord},
    services::{ServicesResource, ServicesResourceRecord},
    tickets::{TicketsResource, TicketsResourceRecord},
    vendors::{VendorsResource, VendorsResourceRecord},
};
