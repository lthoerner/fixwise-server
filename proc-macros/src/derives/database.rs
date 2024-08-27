use deluxe::ExtractAttributes;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

use crate::synerror;

#[derive(ExtractAttributes)]
#[deluxe(attributes(entity))]
struct DatabaseEntityAttributes {
    schema_name: Option<String>,
    entity_name: String,
    primary_column: String,
}

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
        impl crate::database::DatabaseEntity for #type_name {
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

        impl crate::database::DatabaseRow for #row_type_name {
            type Entity = #type_name;
        }
    }
    .into()
}

pub fn derive_single_insert(input: TokenStream) -> TokenStream {
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
                    "cannot derive `SingleInsert` for unit or tuple structs"
                )
            }
        },
        _ => {
            synerror!(
                type_name,
                "cannot derive `SingleInsert` for non-struct types"
            )
        }
    };

    let mut column_names = Vec::new();
    let mut column_idents = Vec::new();
    for field in fields.named.into_iter() {
        let column_name = field.ident.unwrap();
        column_idents.push(quote!(row.#column_name));
        column_names.push(column_name.to_string());
    }

    quote! {
        impl crate::database::SingleInsert for #type_name {
            const COLUMN_NAMES: &[&str] = &[#(#column_names),*];

            fn push_column_bindings(
                mut builder: crate::database::Separated<crate::database::Postgres, &str>,
                row: Self,
            ) {
                #(
                    builder.push_bind(#column_idents);
                )*
            }
        }
    }
    .into()
}

pub fn derive_bulk_insert(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);
    let Data::Struct(_) = data else {
        synerror!(type_name, "cannot derive `BulkInsert` for non-struct types")
    };

    quote! {
        impl crate::database::BulkInsert for #type_name {}
    }
    .into()
}

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
            impl crate::database::tables::IdentifiableRow for #type_name {
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
