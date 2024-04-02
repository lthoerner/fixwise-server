use std::collections::{HashMap, HashSet};

pub mod customers;
pub mod inventory;
pub mod tickets;

pub trait IdentifiableRow {
    fn id(&self) -> i32;
}

pub trait Generate {
    fn generate<'a>(
        existing: &mut HashSet<i32>,
        dependencies: &'a HashMap<&'static str, &'a [impl IdentifiableRow]>,
    ) -> Self;
}
