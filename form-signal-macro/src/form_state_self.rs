use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Ident};

pub fn make_self(ident: &Ident, generics: &Generics, struct_name: &Ident) -> TokenStream {
    quote! {
        impl #generics Default for #struct_name #generics {
            fn default() -> Self {
                let inner: #ident = Default::default();
                Self::new(inner)
            }
        }

        impl #generics #struct_name #generics {
            pub fn new(val: #ident) -> Self {
                Self::from(val)
            }
        }
    }
}
