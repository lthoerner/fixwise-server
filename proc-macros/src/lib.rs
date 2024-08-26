use deluxe::ExtractAttributes;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error as SynError, Fields, Ident};

macro_rules! synerror {
    ( $span_ident:ident, $message:literal ) => {
        return SynError::new($span_ident.span(), $message)
            .into_compile_error()
            .into()
    };
}

#[derive(ExtractAttributes)]
#[deluxe(attributes(entity))]
struct DatabaseEntityAttributes {
    schema_name: Option<String>,
    entity_name: String,
    primary_column: String,
}

#[proc_macro_derive(DatabaseEntity, attributes(entity))]
pub fn derive_database_entity(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input);
    let type_name = input.ident.clone();
    let Data::Struct(_) = input.data else {
        synerror!(
            type_name,
            "cannot derive `DatabaseEntity` for non-struct types"
        )
    };

    let row_type_name = Ident::new(&format!("{}Row", type_name), type_name.span());
    let Ok(DatabaseEntityAttributes {
        schema_name,
        entity_name,
        primary_column,
    }) = deluxe::extract_attributes(&mut input)
    else {
        synerror!(
            type_name,
            "cannot derive `DatabaseEntity` without `#[entity(...)]` attribute"
        )
    };

    let optional_schema_definition = schema_name.map(|schema_name| {
        quote! {
            const SCHEMA_NAME: &str = #schema_name;
        }
    });

    quote! {
        impl DatabaseEntity for #type_name {
            type Row = #row_type_name;
            #optional_schema_definition
            const ENTITY_NAME: &str = #entity_name;
            const PRIMARY_COLUMN_NAME: &str = #primary_column;

            fn with_rows(rows: Vec<Self::Row>) -> Self {
                Self { rows }
            }

            fn take_rows(self) -> Vec<Self::Row> {
                self.rows
            }

            fn rows(&self) -> &[Self::Row] {
                &self.rows
            }
        }
    }
    .into()
}

#[proc_macro_derive(IdParameter)]
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

#[proc_macro_derive(IdentifiableRow)]
pub fn derive_identifiable_row(input: TokenStream) -> TokenStream {
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
                    "cannot derive `IdentifiableRow` for unit or tuple structs"
                )
            }
        },
        _ => {
            synerror!(
                type_name,
                "cannot derive `IdentifiableRow` for non-struct types"
            )
        }
    };

    let first_field = fields.named.into_iter().next();
    if let Some(first_field) = first_field {
        let first_field_name = first_field.ident.unwrap();
        quote! {
            impl IdentifiableRow for #type_name {
                fn id(&self) -> i32 {
                    self.#first_field_name
                }
            }
        }
        .into()
    } else {
        synerror!(
            type_name,
            "cannot derive `IdentifiableRow` for structs with no fields"
        )
    }
}
