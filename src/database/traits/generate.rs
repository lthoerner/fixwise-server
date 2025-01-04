use std::collections::HashSet;

use super::write::{WriteRecord, WriteRelation};
use crate::database::loading_bar::LoadingBar;
use crate::database::tables::generators::*;

/// A trait that allows a database table to be randomly generated.
///
/// This is used for generating arbitrary quantities of synthetic data to test the application.
pub trait GenerateTable: WriteRelation<Record: GenerateRecord> {
    /// Randomly generate the database table with a given number of records.
    ///
    /// Some record types (those with foreign key columns) can only be generated if a set of
    /// existing tables are provided. This means that, when generating multiple database tables,
    /// they must be generated in the correct order such that each will have access to its
    /// dependency tables.
    fn generate(
        count: usize,
        dependencies: <Self::Record as GenerateRecord>::Dependencies<'_>,
    ) -> Self {
        let mut records = Vec::new();
        let mut existing_ids = HashSet::new();
        let mut loading_bar = LoadingBar::new(count);
        for _ in 0..count {
            loading_bar.update();
            records.push(<Self::Record as GenerateRecord>::generate(
                &records,
                &mut existing_ids,
                dependencies,
            ))
        }

        Self::with_records(records)
    }
}

/// A trait that allows a database record to be randomly generated.
///
/// This is used for generating arbitrary quantities of synthetic data to test the application.
pub trait GenerateRecord: WriteRecord + Sized {
    /// The primary identifier type for this record.
    ///
    /// Usually this will be an [`i32`] (signed integers are used for database compatibility, even
    /// though negative values are not expected), but if needed it can be any type that can be put
    /// in a [`HashSet`] to ensure that duplicate records are not generated.
    type Identifier: Copy;
    /// The existing tables which must be provided in order for records of this type to be
    /// generated.
    ///
    /// This should be in the form of a tuple of [`Relation`] types.
    ///
    /// It is mandatory to utilize this feature for any record type with one or more foreign key
    /// columns to ensure referential integrity when the records are inserted into the database.
    type Dependencies<'a>: Copy;

    /// Randomly generate a single record of synthetic data.
    ///
    /// This is usually implemented using a mix of basic RNG and the [`fake`] crate, which can
    /// generate more complex data such as names, phone numbers, email/street addresses, etc. The
    /// implementation must return a record with a unique ID. Any foreign key column must only use
    /// IDs found within its respective dependency table.
    fn generate(
        existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self;
}

/// A trait that allows a database table to be generated from values known at compile-time.
///
/// This is mostly useful for small tables that have a fixed set of data for whom randomly-generated
/// data would not make sense, such as [`tables::device_categories::DeviceCategoriesTable`].
pub trait GenerateStaticTable: WriteRelation<Record: GenerateStaticRecord> {
    /// The items that are to be inserted into the database table.
    ///
    /// This is a string array because [`GenerateStaticTable`] is only implemented for simple tables
    /// with ID-string pairs, using the [`GenerateStaticRecord`] trait to convert the strings to
    /// database entries.
    const ITEMS: &[&str];

    /// Generate the table from static data, usually so it can be inserted into the database.
    ///
    /// This is only called `generate` for semantic consistency with the [`GenerateTable`] trait
    /// which uses actual random data generation.
    fn generate() -> Self {
        let mut existing_ids = HashSet::new();
        let records = Self::ITEMS
            .iter()
            .map(|item| Self::Record::new(generate_unique_i32(0, &mut existing_ids), *item))
            .collect();

        Self::with_records(records)
    }
}

/// A helper trait that allows database records to be generated using a string.
///
/// This trait should only be implemented for record types with simple ID-string pairs.
pub trait GenerateStaticRecord {
    /// Turn a string into a database record.
    ///
    /// This method should only be used for [`GenerateStaticTable::generate`].
    fn new(id: i32, display_name: impl Into<String>) -> Self;
}
