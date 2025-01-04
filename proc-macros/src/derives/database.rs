use deluxe::ExtractAttributes;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
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

#[derive(ExtractAttributes)]
#[deluxe(attributes(auto_primary_key))]
struct AutoPrimaryKeyAttribute;

#[derive(ExtractAttributes)]
#[deluxe(attributes(manual_primary_key))]
struct ManualPrimaryKeyAttribute;

#[derive(Clone)]
enum PrimaryKeyAttribute {
    Auto,
    Manual,
    None,
}

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
        impl crate::database::traits::read::Relation for #type_name {
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

        impl crate::database::traits::read::Record for #record_type_name {
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
        impl crate::database::traits::write::Table for #type_name {}
        impl crate::database::traits::write::TableRecord for #record_type_name {}
    }
    .into()
}

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
        impl crate::database::traits::write::SingleInsert for #type_name {
            const COLUMN_NAMES: &[&str] = &[#(#column_names),*];

            fn push_column_bindings(
                mut builder: sqlx::query_builder::Separated<crate::database::Postgres, &str>,
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
        impl crate::database::traits::write::BulkInsert for #type_name {}
    }
    .into()
}

pub fn derive_create_and_update(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);

    let create_params_type_name = Ident::new(
        &format!("{}CreateQueryParameters", type_name),
        type_name.span(),
    );

    let update_params_type_name = Ident::new(
        &format!("{}UpdateQueryParameters", type_name),
        type_name.span(),
    );

    let fields = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields,
            _ => {
                synerror!(
                    type_name,
                    "cannot derive `CreateAndUpdate` for unit or tuple structs"
                )
            }
        },
        _ => {
            synerror!(
                type_name,
                "cannot derive `CreateAndUpdate` for non-struct types"
            )
        }
    };

    let mut fields_with_primary_key_flags = Vec::new();
    for mut field in fields.named {
        let auto_primary_key =
            deluxe::extract_attributes::<_, AutoPrimaryKeyAttribute>(&mut field).is_ok();
        let manual_primary_key =
            deluxe::extract_attributes::<_, ManualPrimaryKeyAttribute>(&mut field).is_ok();

        let primary_key_flag = match (auto_primary_key, manual_primary_key) {
                (true, true) => synerror!(type_name, "cannot use both `#[auto_primary_key]` and `#[manual_primary_key]` on a single column"),
                (true, false) => PrimaryKeyAttribute::Auto,
                (false, true) => PrimaryKeyAttribute::Manual,
                (false, false) => PrimaryKeyAttribute::None,
            };

        fields_with_primary_key_flags.push((field, primary_key_flag))
    }

    let create_query_parameter_fields: Vec<TokenStream2> = fields_with_primary_key_flags
        .clone()
        .into_iter()
        .filter_map(|(f, pk)| match pk {
            PrimaryKeyAttribute::Auto => None,
            _ => {
                // * This needs to be done instead of just using `quote!(#f)` because otherwise, any
                // * additional attributes on the field would be included in the output
                let field_name = f.ident.unwrap();
                let field_type = f.ty;
                Some(quote!(pub #field_name: #field_type))
            }
        })
        .collect();

    let update_query_parameter_fields: Vec<TokenStream2> = fields_with_primary_key_flags
        .into_iter()
        .map(|(f, pk)| {
            let field_type = f.ty.clone();
            let (new_field_name, new_field_type) = match pk {
                PrimaryKeyAttribute::None => {
                    let field_name_string = f
                        .ident
                        .clone()
                        .unwrap()
                        .to_string()
                        .trim_start_matches("r#")
                        .to_owned();

                    (
                        Ident::new(&format!("new_{}", field_name_string), type_name.span()),
                        quote!(Option<#field_type>),
                    )
                }
                _ => (f.ident.unwrap(), quote!(#field_type)),
            };

            quote!(pub #new_field_name: #new_field_type)
        })
        .collect();

    quote! {
        pub struct #create_params_type_name {
            #(
                #create_query_parameter_fields
            ),*
        }

        pub struct #update_params_type_name {
            #(
                #update_query_parameter_fields
            ),*
        }
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
