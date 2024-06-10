use leptos::*;

use crate::app::{components::StringInputView, state::ContactFormState};

#[component]
pub fn ContactForm(value: Signal<ContactFormState>) -> impl IntoView {
    let contact_name = Signal::derive(move || value.get().name);
    let contact_email = Signal::derive(move || value.get().email);
    let contact_message = Signal::derive(move || value.get().message);

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
