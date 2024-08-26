use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error as SynError, Fields};

macro_rules! synerror {
    ( $span_ident:ident, $message:literal ) => {
        return SynError::new($span_ident.span(), $message)
            .into_compile_error()
            .into()
    };
}

#[proc_macro_derive(IdParameter)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);

    let fields = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields,
            _ => {
                synerror!(
                    type_name,
                    "cannot derive `IdParameter` for unit or tuple structs"
                )
            }
        },
        _ => {
            synerror!(
                type_name,
                "cannot derive `IdParameter` for non-struct types"
            )
        }
    };

    let first_field = fields.named.into_iter().next();
    if let Some(first_field) = first_field {
        let first_field_name = first_field.ident.unwrap();
        quote! {
            impl IdParameter for #type_name {
                fn new(#first_field_name: usize) -> Self {
                    Self { #first_field_name }
                }

                fn id(&self) -> usize {
                    self.#first_field_name
                }
            }
        }
        .into()
    } else {
        synerror!(
            type_name,
            "cannot derive `IdParameter` for structs with no fields"
        )
    }
}
