use proc_macro2::Span;
use syn::{parse_quote, spanned::Spanned, Attribute, Fields, Ident, Type};

pub fn make_form_state_fields(fields: &Fields) -> Fields {
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

pub fn is_nested(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path()
            .get_ident()
            .map(|i| i.to_string().as_str() == "nested")
            .unwrap_or_default()
    })
}

pub fn is_iterable(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path()
            .get_ident()
            .map(|i| i.to_string().as_str() == "iterable")
            .unwrap_or_default()
    })
}

pub fn form_state_generic(ty: &Type) -> Type {
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

pub fn form_state_type(ty: &Type) -> Type {
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
