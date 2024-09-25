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
    primary_key: String,
    foreign_key_name: String,
    dependent_tables: Option<Vec<Ident>>,
}

#[derive(ExtractAttributes)]
#[deluxe(attributes(defaultable))]
struct DefaultableRowAttribute;

pub fn derive_database_entity(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input);
    let type_name = input.ident.clone();
    let row_type_name = Ident::new(&format!("{}Row", type_name), type_name.span());

    let Data::Struct(_) = input.data else {
        synerror!(
            type_name,
            "cannot derive `DatabaseEntity` for non-struct types"
        )
    };

    let Ok(DatabaseEntityAttributes {
        schema_name,
        entity_name,
        primary_key,
        foreign_key_name,
        dependent_tables,
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

    let dependent_tables_definition = match dependent_tables {
        Some(dependent_tables) => quote! {
            const DEPENDENT_TABLES: &[&str] = &[
                #(
                    const_format::formatcp!(
                        "{}.{}",
                        <#dependent_tables as crate::database::DatabaseEntity>::SCHEMA_NAME,
                        <#dependent_tables as crate::database::DatabaseEntity>::ENTITY_NAME
                    ),
                )*
            ];
        },
        None => quote!(
            const DEPENDENT_TABLES: &[&str] = &[];
        ),
    };

    quote! {
        impl crate::database::DatabaseEntity for #type_name {
            type Row = #row_type_name;
            #optional_schema_definition
            const ENTITY_NAME: &str = #entity_name;
            const PRIMARY_KEY: &str = #primary_key;
            const FOREIGN_KEY_NAME: &str = #foreign_key_name;
            #dependent_tables_definition

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

pub fn derive_generate_table_data(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);
    let Data::Struct(_) = data else {
        synerror!(
            type_name,
            "cannot derive `GenerateTableData` for non-struct types"
        )
    };

    quote! {
        impl crate::database::GenerateTableData for #type_name {}
    }
    .into()
}

pub fn derive_single_insert(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);

    let Data::Struct(data_struct) = data else {
        synerror!(
            type_name,
            "cannot derive `SingleInsert` for non-struct types"
        )
    };

    let fields: Vec<(String, Ident, bool)> = {
        let Fields::Named(_) = &data_struct.fields else {
            synerror!(
                type_name,
                "cannot derive `SingleInsert` for unit or tuple structs"
            )
        };

        let mut defaultable_fields: Vec<(String, Ident, bool)> = Vec::new();
        for mut field in data_struct.fields.into_iter() {
            let field_ident = field.ident.clone().unwrap();
            // TODO: Use the #[sqlx(rename = "<name>")] attribute
            let field_name = field_ident
                .clone()
                .to_string()
                .trim_start_matches("r#")
                .to_owned();
            let defaultable_attribute: Option<DefaultableRowAttribute> =
                deluxe::extract_attributes(&mut field).ok();

            defaultable_fields.push((field_name, field_ident, defaultable_attribute.is_some()));
        }

        defaultable_fields
    };

    let mut column_names = Vec::new();
    let mut binding_statements = Vec::new();
    for (column_name, column_ident, defaultable) in fields {
        let binding_or_default = match defaultable {
            true => {
                quote! {
                    match row.#column_ident {
                        Some(column_value) => { builder.push_bind(column_value); },
                        None => { builder.push("DEFAULT"); },
                    }
                }
            }
            false => quote!(builder.push_bind(row.#column_ident);),
        };

        column_names.push(column_name);
        binding_statements.push(binding_or_default);
    }

    quote! {
        impl crate::database::SingleInsert for #type_name {
            const COLUMN_NAMES: &[&str] = &[#(#column_names),*];

            fn push_column_bindings(
                mut builder: crate::database::Separated<crate::database::Postgres, &str>,
                row: Self,
            ) {
                #(
                    #binding_statements
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
