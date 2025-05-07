use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

use crate::synerror;

pub fn derive_generate_table(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);
    let Data::Struct(_) = data else {
        synerror!(
            type_name,
            "cannot derive `GenerateTable` for non-struct types"
        )
    };

    quote! {
        impl crate::database::traits::generate::GenerateTable for #type_name {}
    }
    .into()
}
