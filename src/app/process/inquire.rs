use leptos::*;
use leptos_meta::*;

use crate::app::components::ContactFormView;

/// step 7
#[component]
pub fn InquireView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {}", t!("contact.title"), t!("name"))}/>
        <ContactFormView/>
    }
}
