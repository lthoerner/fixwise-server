use deluxe::ExtractAttributes;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

use crate::synerror;

#[derive(ExtractAttributes)]
struct IdParameterAttribute(Ident);

pub fn derive_serve_entity_json(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);

    let Data::Struct(_) = data else {
        synerror!(
            type_name,
            "cannot derive `ServeEntityJson` for non-struct types"
        )
    };

    quote! {
        impl crate::api::ServeEntityJson for #type_name {}
    }
    .into()
}

pub fn derive_serve_row_json(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input);
    let type_name = input.ident.clone();
    let Data::Struct(_) = input.data else {
        synerror!(
            type_name,
            "cannot derive `ServeRowJson` for non-struct types"
        )
    };

    let Ok(IdParameterAttribute(id_param_type)) = deluxe::extract_attributes(&mut input) else {
        synerror!(
            type_name,
            "cannot derive `ServeRowJson` without `#[id_param(...)]` attribute"
        )
    };

    quote! {
        impl crate::api::ServeRowJson<#id_param_type> for #type_name {}
    }
    .into()
}

pub fn derive_id_parameter(input: TokenStream) -> TokenStream {
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
            impl crate::api::IdParameter for #type_name {
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
