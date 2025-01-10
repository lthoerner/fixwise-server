pub mod customers;
pub mod device_models;
pub mod devices;
pub mod invoices;
pub mod items;
pub mod parts;
pub mod products;
pub mod services;
pub mod tickets;
pub mod vendors;

#[allow(unused_imports)]
pub use {
    customers::{CustomersView, CustomersViewRecord},
    device_models::{DeviceModelsView, DeviceModelsViewRecord},
    devices::{DevicesView, DevicesViewRecord},
    invoices::{InvoicesView, InvoicesViewRecord},
    items::{ItemsView, ItemsViewRecord},
    parts::{PartsView, PartsViewRecord},
    products::{ProductsView, ProductsViewRecord},
    services::{ServicesView, ServicesViewRecord},
    tickets::{TicketsView, TicketsViewRecord},
    vendors::{VendorsView, VendorsViewRecord},
};
