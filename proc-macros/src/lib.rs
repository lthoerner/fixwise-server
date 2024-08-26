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

#[proc_macro_derive(IdParameter)]
pub fn derive_id_parameter(input: TokenStream) -> TokenStream {
    derives::api::derive_id_parameter(input)
}

#[proc_macro_derive(DatabaseEntity, attributes(entity))]
pub fn derive_database_entity(input: TokenStream) -> TokenStream {
    derives::database::derive_database_entity(input)
}

#[proc_macro_derive(BulkInsert, attributes(entity))]
pub fn derive_bulk_insert(input: TokenStream) -> TokenStream {
    derives::database::derive_bulk_insert(input)
}

#[proc_macro_derive(IdentifiableRow)]
pub fn derive_identifiable_row(input: TokenStream) -> TokenStream {
    derives::database::derive_identifiable_row(input)
}
