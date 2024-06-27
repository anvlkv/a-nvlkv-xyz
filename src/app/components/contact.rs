use form_signal::{AllSignalTraits, FormSignal};
use leptos::*;

use crate::app::{components::StringInputView, state::Contact};

#[component]
pub fn ContactForm<T, Rw, R, W>(value: FormSignal<T, Contact, Rw, R, W>) -> impl IntoView
where
    Rw: AllSignalTraits<T>,
    T: std::fmt::Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> Contact + Clone + 'static,
    W: Fn(&mut T, Contact) + Clone + 'static,
{
    let contact_name = value.derive(|v| v.name.clone(), |v, name| v.name = name);
    let contact_email = value.derive(|v| v.email.clone(), |v, email| v.email = email);
    let contact_message = value.derive(|v| v.message.clone(), |v, message| v.message = message);

    view! {
        <label class="block my-2">
            <p class="mb-1">{t!("contact.name.label")}</p>
            <StringInputView
                attr:required=true
                attr:autocomplete="given-name"
                attr:name="name"
                input_type="text"
                value=contact_name
                placeholder=t!("contact.name.placeholder").to_string()
            />
        </label>
        <label class="block my-2">
            <p class="mb-1">{t!("contact.email.label")}</p>
            <StringInputView
                attr:required=true
                attr:autocomplete="email"
                attr:name="email"
                input_type="email"
                value=contact_email
                placeholder=t!("contact.email.placeholder").to_string()
            />
        </label>
        <label class="block my-2">
            <p class="mb-1">{t!("contact.message.label")}</p>
            <StringInputView
                attr:required=true
                attr:name="message"
                input_type="textarea"
                value=contact_message
                placeholder=t!("contact.message.placeholder").to_string()
            />
        </label>
    }
}
