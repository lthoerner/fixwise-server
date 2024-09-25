mod derives;

use proc_macro::{self, TokenStream};

macro_rules! synerror {
    ( $span_ident:ident, $message:literal ) => {
        return syn::Error::new($span_ident.span(), $message)
            .into_compile_error()
            .into()
    };
}

pub(crate) use synerror;

#[proc_macro_derive(ProcessEndpoint, attributes(col_format))]
pub fn derive_process_endpoint(input: TokenStream) -> TokenStream {
    derives::api::derive_process_endpoint(input)
}

#[proc_macro_derive(FromRelation, attributes(endpoint))]
pub fn derive_from_relation(input: TokenStream) -> TokenStream {
    derives::api::derive_from_relation(input)
}

#[proc_macro_derive(FromRecord, attributes(endpoint_row))]
pub fn derive_from_record(input: TokenStream) -> TokenStream {
    derives::api::derive_from_record(input)
}

#[proc_macro_derive(ServeEntityJson)]
pub fn derive_serve_entity_json(input: TokenStream) -> TokenStream {
    derives::api::derive_serve_entity_json(input)
}

#[proc_macro_derive(ServeRowJson, attributes(endpoint_row))]
pub fn derive_serve_row_json(input: TokenStream) -> TokenStream {
    derives::api::derive_serve_row_json(input)
}

#[proc_macro_derive(IdParameter)]
pub fn derive_id_parameter(input: TokenStream) -> TokenStream {
    derives::api::derive_id_parameter(input)
}

#[proc_macro_derive(Relation, attributes(relation))]
pub fn derive_relation(input: TokenStream) -> TokenStream {
    derives::database::derive_relation(input)
}

#[proc_macro_derive(GenerateTableData)]
pub fn derive_generate_table_data(input: TokenStream) -> TokenStream {
    derives::database::derive_generate_table_data(input)
}

#[proc_macro_derive(SingleInsert, attributes(defaultable))]
pub fn derive_single_insert(input: TokenStream) -> TokenStream {
    derives::database::derive_single_insert(input)
}

#[proc_macro_derive(BulkInsert)]
pub fn derive_bulk_insert(input: TokenStream) -> TokenStream {
    derives::database::derive_bulk_insert(input)
}

#[proc_macro_derive(IdentifiableRow)]
pub fn derive_identifiable_row(input: TokenStream) -> TokenStream {
    derives::database::derive_identifiable_row(input)
}
