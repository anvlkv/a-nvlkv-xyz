use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Fields, Generics, Ident, LitInt};

use super::{form_state_type, is_iterable, is_nested};

pub fn make_impl_from(
    fields: &Fields,
    ident: &Ident,
    generics: &Generics,
    struct_name: &Ident,
) -> TokenStream {
    let is_tuple = fields.iter().any(|f| f.ident.is_none());

    let (from_impl_expressions, from_impl_fields): (Vec<TokenStream>, Vec<TokenStream>) = fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field.ident.clone().map(|i| i.to_token_stream()).unwrap_or(
                LitInt::new(format!("{i}").as_str(), Span::mixed_site()).to_token_stream(),
            );
            if is_nested(field.attrs.as_slice()) {
                let nested_ty = form_state_type(&field.ty);
                (
                    quote! {
                        leptos::RwSignal::new(
                            #nested_ty::from(value.#ident)
                        ),
                    },
                    ident,
                )
            } else if is_iterable(field.attrs.as_slice()) {
                (
                    quote! {
                        value.#ident.into_iter().map(|v| {
                            form_signal::FormState::new(v)
                        }).collect(),
                    },
                    ident,
                )
            } else {
                (
                    quote! {
                        form_signal::FormState::new(value.#ident),
                    },
                    ident,
                )
            }
        })
        .unzip();

    let from_fields_collect = if is_tuple {
        quote! {
            Self(
                #(#from_impl_expressions)*
            )
        }
    } else {
        let named_fields = from_impl_expressions
            .into_iter()
            .zip(from_impl_fields)
            .map(|(e, f)| quote! {#f: #e});

        quote! {
            Self {
                #(#named_fields)*
            }
        }
    };

    quote! {
        impl #generics From<#ident #generics> for #struct_name #generics {
            fn from(value: #ident #generics) -> Self {
                #from_fields_collect
            }
        }
    }
}

pub fn make_impl_into(
    fields: &Fields,
    ident: &Ident,
    generics: &Generics,
    struct_name: &Ident,
) -> TokenStream {
    let is_tuple = fields.iter().any(|f| f.ident.is_none());

    let (into_impl_expressions, into_impl_fields): (Vec<TokenStream>, Vec<TokenStream>) = fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field.ident.clone().map(|i| i.to_token_stream()).unwrap_or(
                LitInt::new(format!("{i}").as_str(), Span::mixed_site()).to_token_stream(),
            );
            if is_nested(field.attrs.as_slice()) {
                let ty = field.ty.clone();
                (
                    quote! {
                        Into::<#ty>::into(
                            &leptos::SignalGetUntracked::get_untracked(&self.#ident)
                        ),
                    },
                    ident,
                )
            } else if is_iterable(field.attrs.as_slice()) {
                (
                    quote! {
                        self.#ident
                            .iter()
                            .map(|s| {
                                leptos::SignalGetUntracked::get_untracked(s)
                            })
                            .collect(),
                    },
                    ident,
                )
            } else {
                (
                    quote! {
                        leptos::SignalGetUntracked::get_untracked(&self.#ident),
                    },
                    ident,
                )
            }
        })
        .unzip();

    let into_fields_collect = if is_tuple {
        quote! {
            #ident(
                #(#into_impl_expressions)*
            )
        }
    } else {
        let named_fields = into_impl_expressions
            .into_iter()
            .zip(into_impl_fields)
            .map(|(e, f)| quote! {#f: #e});

        quote! {
            #ident {
                #(#named_fields)*
            }
        }
    };

    quote! {
        impl #generics Into<#ident #generics> for &#struct_name #generics {
            fn into(self) -> #ident #generics {
                #into_fields_collect
            }
        }
    }
}
