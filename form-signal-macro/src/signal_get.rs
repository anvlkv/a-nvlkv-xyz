use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Fields, Generics, Ident, LitInt};

use super::{is_iterable, is_nested};

pub fn make_signal_get(
    fields: &Fields,
    ident: &Ident,
    generics: &Generics,
    struct_name: &Ident,
    (trait_path, getter, try_getter): (TokenStream, TokenStream, TokenStream),
) -> TokenStream {
    let is_tuple = fields.iter().any(|f| f.ident.is_none());

    let (expressions, get_fields): (Vec<(TokenStream, TokenStream)>, Vec<TokenStream>) = fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field.ident.clone().map(|i| i.to_token_stream()).unwrap_or(
                LitInt::new(format!("{i}").as_str(), Span::mixed_site()).to_token_stream(),
            );

            if is_nested(field.attrs.as_slice()) {
                (
                    (
                        quote! {
                            #trait_path::#getter(
                                &#trait_path::#getter(
                                    &self.#ident
                                )
                            ),
                        },
                        quote! {
                            #trait_path::#try_getter(
                                &self.#ident
                            ).map(|v| #trait_path::#try_getter(&v).is_some()).unwrap_or(false),
                        },
                    ),
                    ident,
                )
            } else if is_iterable(field.attrs.as_slice()) {
                let ty = field.ty.clone();

                (
                    (
                        quote! {
                            self.#ident.iter().map(|v| {
                                #trait_path::#getter(v)
                            }).collect::<#ty>(),
                        },
                        quote! {
                            {
                                self.#ident
                                    .iter()
                                    .any(|v| #trait_path::#try_getter(v).is_some())
                            },
                        },
                    ),
                    ident,
                )
            } else {
                (
                    (
                        quote! {
                            #trait_path::#getter(
                                &self.#ident
                            ),
                        },
                        quote! {
                            #trait_path::#try_getter(
                                &self.#ident
                            ).is_some(),
                        },
                    ),
                    ident,
                )
            }
        })
        .unzip();

    let (get_expressions, try_get_tests): (Vec<TokenStream>, Vec<TokenStream>) =
        expressions.into_iter().unzip();

    let (collect_get_fields, collect_try_get_fields) = if is_tuple {
        let try_get_expressions = get_expressions.clone();

        (
            quote! {
                #ident (
                    #(#get_expressions)*
                )
            },
            quote! {
                let fields = vec![#(#try_get_tests)*];

                if fields.iter().any(|f| !f) {
                    None
                }
                else {
                    Some(#ident (
                        #(#try_get_expressions)*
                    ))
                }
            },
        )
    } else {
        let named_fields = get_expressions
            .iter()
            .zip(get_fields.clone())
            .map(|(e, f)| quote! {#f: #e});

        let try_named_fields = named_fields.clone();

        (
            quote! {
                #ident  {
                    #(#named_fields)*
                }
            },
            quote! {
                let fields = vec![#(#try_get_tests)*];

                if fields.iter().any(|f| !f) {
                    None
                }
                else {
                    Some(#ident {
                        #(#try_named_fields)*
                    })
                }
            },
        )
    };

    quote! {
        impl #generics #trait_path for #struct_name #generics {
            type Value = #ident;

            fn #getter(&self) -> Self::Value {
                #collect_get_fields
            }

            fn #try_getter(&self) -> Option<Self::Value> {
                #collect_try_get_fields
            }
        }
    }
}
