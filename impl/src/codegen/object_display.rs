use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn gen_display_object(struct_ident: &Ident) -> TokenStream {
    quote! {
        impl core::fmt::Display for  #struct_ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", Into::<SurrealValue>::into(self.clone()).0)
            }
        }
    }
}
