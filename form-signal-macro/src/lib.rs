use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Attribute, DataStruct, DeriveInput, Fields,
    Generics, Ident, LitInt, Type,
};

/// Given a `struct StructName { value: String }`
/// generates a corresponding `struct StructNameFormState { value: FormState<String> }`
/// along with required `.into()` and `.from()` implementations.
///
/// The derived `FormState` also implements `leptos::SignalGet`
///
/// use `#[nested]` field attribute for fields which represent nested forms
/// turning `Nested` into `NestedFormState` where `Nested` must derive `FormState`
///
/// use `#[iterable]` field attribute for fields with iterable values
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

    let impl_signal_get = make_signal_get(&fields, &ident, &generics, &struct_name);

    let expanded = quote! {
        #[derive(Default, Clone, Debug, PartialEq, Eq)]
        #vis #struct_token #struct_name #generics #form_state_fields #semi_token

        #impl_from

        #impl_into

        #impl_signal_get
    };

    proc_macro::TokenStream::from(expanded)
}

fn make_signal_get(
    fields: &Fields,
    ident: &Ident,
    generics: &Generics,
    struct_name: &Ident,
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
                            leptos::SignalGet::get(
                                &self.#ident
                            ).get(),
                        },
                        quote! {
                            leptos::SignalGet::try_get(
                                &self.#ident
                            ).map(|v| v.try_get().is_some()).unwrap_or(false),
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
                                leptos::SignalGet::get(v)
                            }).collect::<#ty>(),
                        },
                        quote! {
                            {
                                self.#ident
                                    .iter()
                                    .any(|v| leptos::SignalGet::try_get(v).is_some())
                            },
                        },
                    ),
                    ident,
                )
            } else {
                (
                    (
                        quote! {
                            leptos::SignalGet::get(
                                &self.#ident
                            ),
                        },
                        quote! {
                            leptos::SignalGet::try_get(
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
        impl #generics leptos::SignalGet for #struct_name #generics {
            type Value = #ident;

            fn get(&self) -> Self::Value {
                #collect_get_fields
            }

            fn try_get(&self) -> Option<Self::Value> {
                #collect_try_get_fields
            }
        }
    }
}

fn make_form_state_fields(fields: &Fields) -> Fields {
    let mut form_state_fields = fields.clone();
    form_state_fields.iter_mut().for_each(|field| {
        let ty = field.ty.clone();
        let attrs = field.attrs.clone();

        let replace: Type = if is_nested(attrs.as_slice()) {
            let ty = form_state_type(&ty);
            parse_quote! { leptos::RwSignal<#ty> }
        } else if is_iterable(attrs.as_slice()) {
            form_state_generic(&ty)
        } else {
            parse_quote! { form_signal::FormState<#ty> }
        };

        field.ty = replace;

        field.attrs = vec![];
    });

    form_state_fields
}

fn make_impl_from(
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

fn make_impl_into(
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

fn is_nested(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path()
            .get_ident()
            .map(|i| i.to_string().as_str() == "nested")
            .unwrap_or_default()
    })
}

fn is_iterable(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path()
            .get_ident()
            .map(|i| i.to_string().as_str() == "iterable")
            .unwrap_or_default()
    })
}

fn form_state_generic(ty: &Type) -> Type {
    match ty {
        Type::Path(p) => {
            let mut p = p.clone();
            p.path.segments.last_mut().map(|s| match &mut s.arguments {
                syn::PathArguments::AngleBracketed(a) => a.args.iter_mut().for_each(|a| match a {
                    syn::GenericArgument::Type(ty) => {
                        let replace: Type = parse_quote! { form_signal::FormState<#ty> };
                        *ty = replace;
                    }
                    _ => {}
                }),
                _ => unreachable!(),
            });

            Type::Path(p)
        }
        Type::Tuple(t) => {
            let mut t = t.clone();
            t.elems.iter_mut().for_each(|ty| {
                *ty = form_state_generic(ty);
            });
            Type::Tuple(t)
        }
        Type::Array(a) => {
            let mut a = a.clone();
            let t = form_state_generic(&a.elem.as_ref());
            a.elem = Box::new(t);
            Type::Array(a)
        }
        _ => {
            proc_macro_error::emit_error!(ty.span(), "field types must be Path, Tuple or Array");
            ty.clone()
        }
    }
}

fn form_state_type(ty: &Type) -> Type {
    match ty.clone() {
        Type::Path(mut p) => {
            let last = p.path.segments.last_mut().unwrap();

            let replace = Ident::new(
                format!("{}FormState", last.ident).as_str(),
                Span::mixed_site(),
            );

            last.ident = replace;

            Type::Path(p)
        }
        Type::Tuple(mut t) => {
            t.elems.iter_mut().for_each(|ty| *ty = form_state_type(&ty));

            Type::Tuple(t)
        }
        Type::Array(mut a) => {
            a.elem = Box::new(form_state_type(a.elem.as_ref()));
            Type::Array(a)
        }
        _ => {
            proc_macro_error::emit_error!(ty.span(), "field types must be Path, Tuple or Array");
            ty.clone()
        }
    }
}
