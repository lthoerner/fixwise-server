use deluxe::ExtractAttributes;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

use crate::synerror;

#[derive(ExtractAttributes)]
#[deluxe(attributes(relation))]
struct RelationAttributes {
    schema_name: Option<String>,
    relation_name: String,
    primary_key: String,
}

#[derive(ExtractAttributes)]
#[deluxe(attributes(defaultable))]
struct DefaultableRecordAttribute;

pub fn derive_relation(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input);
    let type_name = input.ident.clone();
    let record_type_name = Ident::new(&format!("{}Record", type_name), type_name.span());

    let Data::Struct(_) = input.data else {
        synerror!(type_name, "cannot derive `Relation` for non-struct types")
    };

    let Ok(RelationAttributes {
        schema_name,
        relation_name,
        primary_key,
    }) = deluxe::extract_attributes(&mut input)
    else {
        synerror!(
            type_name,
            "cannot derive `Relation` without `#[relation(...)]` attribute"
        )
    };

    let optional_schema_definition = schema_name.map(|schema_name| {
        quote! {
            const SCHEMA_NAME: &str = #schema_name;
        }
    });

    quote! {
        impl crate::database::Relation for #type_name {
            type Record = #record_type_name;
            #optional_schema_definition
            const RELATION_NAME: &str = #relation_name;
            const PRIMARY_KEY: &str = #primary_key;

            fn with_records(records: Vec<Self::Record>) -> Self {
                Self { records }
            }

            fn take_records(self) -> Vec<Self::Record> {
                self.records
            }

            fn records(&self) -> &[Self::Record] {
                &self.records
            }
        }

        impl crate::database::Record for #record_type_name {
            type Relation = #type_name;
        }
    }
    .into()
}

pub fn derive_table(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);

    let record_type_name = Ident::new(&format!("{}Record", type_name), type_name.span());

    let Data::Struct(_) = data else {
        synerror!(type_name, "cannot derive `Table` for non-struct types")
    };

    quote! {
        impl crate::database::Table for #type_name {}
        impl crate::database::TableRecord for #record_type_name {}
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
            let defaultable_attribute: Option<DefaultableRecordAttribute> =
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
                    match record.#column_ident {
                        Some(column_value) => { builder.push_bind(column_value); },
                        None => { builder.push("DEFAULT"); },
                    }
                }
            }
            false => quote!(builder.push_bind(record.#column_ident);),
        };

        column_names.push(column_name);
        binding_statements.push(binding_or_default);
    }

    quote! {
        impl crate::database::SingleInsert for #type_name {
            const COLUMN_NAMES: &[&str] = &[#(#column_names),*];

            fn push_column_bindings(
                mut builder: crate::database::Separated<crate::database::Postgres, &str>,
                record: Self,
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

pub fn derive_identifiable_record(input: TokenStream) -> TokenStream {
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
                    "cannot derive `IdentifiableRecord` for unit or tuple structs"
                )
            }
        },
        _ => {
            synerror!(
                type_name,
                "cannot derive `IdentifiableRecord` for non-struct types"
            )
        }
    };

    let first_field = fields.named.into_iter().next();
    if let Some(first_field) = first_field {
        let first_field_name = first_field.ident.unwrap();
        quote! {
            impl crate::database::tables::IdentifiableRecord for #type_name {
                fn id(&self) -> i32 {
                    self.#first_field_name
                }
            }
        }
        .into()
    } else {
        synerror!(
            type_name,
            "cannot derive `IdentifiableRecord` for structs with no fields"
        )
    }
}
