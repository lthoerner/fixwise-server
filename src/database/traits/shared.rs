use rand::{thread_rng, Rng};
use sqlx::postgres::PgRow;

/// A trait that allows table and view types to interoperate with and be queried from the database.
///
/// This does not implement any insertion or deletion methods because "relations" can be views,
/// which are read-only. For inserting items to tables, see the [`SingleInsert`] and [`BulkInsert`]
/// traits. For deleting items from tables, see the [`Table`] trait.
///
/// This trait does not do a lot on its own but it, along with [`Record`], provides the
/// functionality which allows almost all of the other database traits to be auto-implemented or
/// conveniently derived.
pub trait Relation: Sized {
    /// The record type which this relation contains a collection of.
    ///
    /// This type and the [`Record::Relation`] type are directly interreferential to allow
    /// "upcasting" and "downcasting," mostly for auto-implementations in other traits.
    type Record: Record<Relation = Self>;

    /// The name of the schema in which this relation exists in the database.
    ///
    /// This defaults to "main" but can be changed in case a relation lives in a different schema.
    /// The main alternate schema which would be used here is "persistent" for items which are not
    /// deleted each time the application is run. This will be unnecessary once Fixwise is no longer
    /// in early development/testing.
    const SCHEMA_NAME: &str = "main";
    /// The name of the relation in the database.
    ///
    /// It is recommended that all [`Relation`] types should have an identical name to the one they
    /// have in the database (with different case conventions, of course), but this is not assumed
    /// in order to be slightly less restrictive.
    const RELATION_NAME: &str;
    /// The primary column of this relation in the database.
    ///
    /// This is used directly in the SQL for querying the relation, so it should be in the format
    /// expected by SQL. For most relations, this will be a standalone column name, but for junction
    /// tables, it will be multiple column names written as a parenthesized, comma-separated list,
    /// such as `"(column_a, column_b, column_c)"`.
    const PRIMARY_KEY: &str;

    /// Create the relation from a collection of records.
    // TODO: Take `Into<Vec<Self::Record>>` here
    fn with_records(records: Vec<Self::Record>) -> Self;
    /// Convert the relation into a collection of records.
    fn take_records(self) -> Vec<Self::Record>;
    /// Borrow the relation's records.
    fn records(&self) -> &[Self::Record];

    /// Pick a random record from the relation.
    ///
    /// This is used mostly for randomly generating foreign keys, but can be used elsewhere if
    /// needed.
    fn pick_random(&self) -> Self::Record {
        let records = self.records();
        records[thread_rng().gen_range(0..records.len())].clone()
    }
}

/// A trait that allows table/view record types to interoperate with and be queried from the
/// database.
///
/// This does not implement any insertion methods because "relations" can be views, which are
/// read-only. For inserting items to tables, see the [`SingleInsert`] and [`BulkInsert`] traits.
///
/// This trait mostly exists for use with insertion traits, but also acts as a passthrough to allow
/// items to be queried using the record type instead of the relation type when convenient.
pub trait Record: for<'a> sqlx::FromRow<'a, PgRow> + Send + Unpin + Clone {
    /// The relation type which contains a collection of this record type.
    ///
    /// This type and the [`Relation::Record`] type are directly interreferential to allow
    /// "upcasting" and "downcasting," mostly for auto-implementations in other traits.
    type Relation: Relation<Record = Self>;
}
