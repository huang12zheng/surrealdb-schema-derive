use proc_macro2::TokenStream;
use quote::quote;
use syn::{FieldsNamed, GenericArgument, Ident, PathArguments, Type};

pub(crate) fn gen_into_value(struct_ident: &Ident, fields: &FieldsNamed) -> TokenStream {
    let field_conversions: Vec<TokenStream> = fields
        .named
        .iter()
        .map(|field| {
            let field_ident = field.ident.clone().unwrap();
            let field_ref =
                if maybe_extract_optional(field).is_some() || maybe_extract_vec(field).is_some() {
                    quote! {IntoValue::into(self.#field_ident)}
                } else {
                    quote! {self.#field_ident.into()}
                };
            quote! {
                (stringify!(#field_ident).into(),#field_ref)
            }
        })
        .collect();

    quote! {
        impl Into<surrealdb_obj_derive::surrealdb::sql::Value> for #struct_ident {
            fn into(self) -> surrealdb_obj_derive::surrealdb::sql::Value {
                surrealdb_obj_derive::surrealdb::sql::Value::Object(
                    surrealdb_obj_derive::surrealdb::sql::Object(std::collections::BTreeMap::from([
                        #(#field_conversions),*
                    ]))
                )
            }
        }

        impl CompressionWith for #struct_ident {
            fn compression_with(value: surrealdb_obj_derive::surrealdb::sql::Value) -> Self {
                deserialize::<#struct_ident>(serialize(value))
            }
        }
    }
}

fn maybe_extract_optional(field: &syn::Field) -> Option<Type> {
    if let Type::Path(ref path_type) = field.ty {
        if let Some(first) = path_type.path.segments.first() {
            if first.ident == "Option" {
                if let PathArguments::AngleBracketed(angle_bracketed) = &first.arguments {
                    let first_arg = angle_bracketed.args.first();
                    if let Some(GenericArgument::Type(inner_type)) = first_arg {
                        Some(inner_type.clone())
                    } else {
                        panic!("Invalid option: {:?}", path_type);
                    }
                } else {
                    panic!("Invalid option: {:?}", path_type);
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn maybe_extract_vec(field: &syn::Field) -> Option<Type> {
    if let Type::Path(ref path_type) = field.ty {
        if let Some(first) = path_type.path.segments.first() {
            if first.ident == "Vec" {
                if let PathArguments::AngleBracketed(angle_bracketed) = &first.arguments {
                    let first_arg = angle_bracketed.args.first();
                    if let Some(GenericArgument::Type(inner_type)) = first_arg {
                        Some(inner_type.clone())
                    } else {
                        panic!("Invalid Vec: {:?}", path_type);
                    }
                } else {
                    panic!("Invalid Vec: {:?}", path_type);
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
