use leptos::*;

use super::StepperView;
use crate::app::components::ContactFormView;

/// step 7
#[component]
pub fn InquireView() -> impl IntoView {
    view! {
        <section class="self-stretch w-full flex flex-col justify-between">
            <div class="p-8 my-8 dark:bg-stone-800 rounded-lg">
                <ContactFormView/>
            </div>
            <StepperView/>
        </section>
    }
}
