use leptos::*;
use leptos_router::*;

use crate::app::components::ContactFormView;

#[component]
pub fn ContactView() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-screen-2xl px-6 md:px-8 lg:px-16 min-h-full flex justify-center items-center">
            <ContactFormView/>
        </div>
    }
}
