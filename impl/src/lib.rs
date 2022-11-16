//! This crate implements the macro for `surrealdb_obj_derive` and should not be used directly.

mod codegen;
mod errors;
pub mod runtime;

use codegen::*;
use derive_builder::Builder;
pub use errors::*;
use proc_macro2::TokenStream;
use quote::quote;
pub use runtime::surreal_value_primitives::*;
pub use runtime::surreal_value_primitives::{SurrealOption, SurrealValue};
use surrealdb::{self, Datastore, Session};
use syn::{parse2, spanned::Spanned, Data, DeriveInput, Fields, FieldsNamed, Ident};

#[derive(Clone)]
pub struct DefineTableContext {
    pub namespace: String,
    pub database: String,
}

#[derive(Builder)]
pub struct DefineTableArgs<'a> {
    pub datastore: &'a Datastore,
    pub session: &'a Session,
    #[builder(setter(into, strip_option), default)]
    pub context: Option<DefineTableContext>,
}

pub trait SurrealDbObject:
    TryFrom<SurrealValue, Error = SurrealDbSchemaDeriveQueryError> + Into<SurrealValue>
{
    fn get_table_name() -> String;
}

#[doc(hidden)]
pub fn derive_surreal_db_object(_item: TokenStream) -> Result<TokenStream, syn::Error> {
    let (struct_ident, fields) = extract_derive_struct(_item)?;
    gen_surreal_db_object(&struct_ident, &fields)
}

fn gen_surreal_db_object(
    struct_ident: &Ident,
    fields: &FieldsNamed,
) -> Result<TokenStream, syn::Error> {
    let impl_into_surreal_value = object_conversion::gen_into_value(&struct_ident, &fields);
    let impl_display_object = object_display::gen_display_object(&struct_ident);
    return Ok(TokenStream::from(quote! {
        #impl_into_surreal_value
        #impl_display_object

        impl SurrealDbObject for #struct_ident {

            fn get_table_name() -> String {
                stringify!(#struct_ident).into()
            }
        }


    }));
}

fn extract_derive_struct(struct_stream: TokenStream) -> Result<(Ident, FieldsNamed), syn::Error> {
    let top_level_error_span = struct_stream.span();
    let input: DeriveInput = parse2(struct_stream)?;
    let ident = input.ident;
    let fields = (if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(named_fields) = data_struct.fields {
            Ok(named_fields)
        } else {
            Err(syn::Error::new(
                top_level_error_span,
                "Must use named fields",
            ))
        }
    } else {
        Err(syn::Error::new(
            top_level_error_span,
            "Only structs are supported for surrealdb deriving",
        ))
    })?;
    Ok((ident, fields))
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn derives_simple_struct() {
        assert_debug_snapshot!(derive_surreal_db_object(quote! {
            struct MyStruct {
                name: String,
                count: u8,
                value: isize,
            }
        })
        .unwrap());
        assert_debug_snapshot!(derive_surreal_db_object(quote! {
            struct RustStruct {
                name: Option<String>,
                generics: RustGenerics,
            }
        })
        .unwrap());
    }
}
