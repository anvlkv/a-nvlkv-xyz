mod form_state;
mod form_state_self;
mod from_into;
mod signal_get;

use form_state::*;
use form_state_self::*;
use from_into::*;
use proc_macro2::Span;
use quote::quote;
use signal_get::*;
use syn::{parse_macro_input, DataStruct, DeriveInput};

/// Given a `struct StructName { value: String }`
/// generates a corresponding `struct StructNameFormState { value: FormState<String> }`
/// along with required `.into()` and `.from()` implementations.
///
/// The derived `FormState` also implements `leptos::{SignalGet, SignalGetUntracked}`
///
/// use `#[nested]` field attribute for fields which represent nested forms
/// turning `Nested` into `NestedFormState` where `Nested` must derive `FormState`
///
/// use `#[iterable]` field attribute for fields with iterable values
/// only iterables with single generic are supported aka `Vec<T>`
#[proc_macro_derive(FormState, attributes(nested, iterable))]
pub fn derive_form_state(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        vis,
        data,
        attrs: _,
        generics,
        ident,
    } = parse_macro_input!(item as DeriveInput);

    let DataStruct {
        struct_token,
        fields,
        semi_token,
    } = if let syn::Data::Struct(data) = data {
        data
    } else {
        proc_macro_error::emit_error!(ident.span(), "FormState can only be derived for structs");

        return proc_macro::TokenStream::new();
    };

    let struct_name = syn::Ident::new(format!("{}FormState", ident).as_str(), Span::mixed_site());

    let form_state_fields = make_form_state_fields(&fields);

    let impl_from = make_impl_from(&fields, &ident, &generics, &struct_name);

    let impl_into = make_impl_into(&fields, &ident, &generics, &struct_name);

    let impl_signal_get = make_signal_get(
        &fields,
        &ident,
        &generics,
        &struct_name,
        (
            quote! {
                leptos::SignalGet
            },
            quote! {
                get
            },
            quote! {
                try_get
            },
        ),
    );

    let impl_signal_get_untracked = make_signal_get(
        &fields,
        &ident,
        &generics,
        &struct_name,
        (
            quote! {
                leptos::SignalGetUntracked
            },
            quote! {
                get_untracked
            },
            quote! {
                try_get_untracked
            },
        ),
    );

    let impl_self = make_self(&ident, &generics, &struct_name);

    let expanded = quote! {
        #[derive(Clone, Debug, PartialEq, Eq)]
        #vis #struct_token #struct_name #generics #form_state_fields #semi_token

        #impl_from

        #impl_into

        #impl_signal_get

        #impl_signal_get_untracked

        #impl_self
    };

    proc_macro::TokenStream::from(expanded)
}
