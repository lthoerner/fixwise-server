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

#[proc_macro_derive(FromRelation, attributes(resource))]
pub fn derive_from_relation(input: TokenStream) -> TokenStream {
    derives::api::derive_from_relation(input)
}

#[proc_macro_derive(FromRecord, attributes(resource_record))]
pub fn derive_from_record(input: TokenStream) -> TokenStream {
    derives::api::derive_from_record(input)
}

#[proc_macro_derive(ServeResourceJson)]
pub fn derive_serve_resource_json(input: TokenStream) -> TokenStream {
    derives::api::derive_serve_resource_json(input)
}

#[proc_macro_derive(ServeRecordJson, attributes(resource_record))]
pub fn derive_serve_record_json(input: TokenStream) -> TokenStream {
    derives::api::derive_serve_record_json(input)
}

#[proc_macro_derive(IdParameter)]
pub fn derive_id_parameter(input: TokenStream) -> TokenStream {
    derives::api::derive_id_parameter(input)
}

#[proc_macro_derive(GenerateTable)]
pub fn derive_generate_table(input: TokenStream) -> TokenStream {
    derives::database::derive_generate_table(input)
}
