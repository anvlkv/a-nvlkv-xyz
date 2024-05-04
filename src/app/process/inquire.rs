use leptos::*;
use leptos_meta::*;

use crate::app::components::ContactFormView;

/// step 7
#[component]
pub fn InquireView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {}", t!("contact.title"), t!("name"))}/>
        <section class="grow w-full p-8 my-8 bg-stone-200 dark:bg-stone-800 rounded-lg">
            <ContactFormView/>
        </section>
    }
}
