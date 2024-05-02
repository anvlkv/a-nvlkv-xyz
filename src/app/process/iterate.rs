use leptos::*;

use super::StepperView;

/// step 6
#[component]
pub fn IterateView() -> impl IntoView {
    view! {
        <section class="self-stretch w-full flex flex-col justify-between">
            <div class="p-8 my-8 dark:bg-stone-800 rounded-lg">
                <h3 class="w-full text-center text-2xl md:text-3x xl:text-4xl pb-6">{t!("worksheets.iterate.title")}</h3>
                <div class="max-w-prose">
                    <p class="pb-4">{t!("worksheets.iterate.description")}</p>
                </div>

                <label>
                    <p>{t!("worksheets.iterate.instruction_1")}</p>
                    <input type="checkbox"/>
                </label>
                <label>
                    <p>{t!("worksheets.iterate.instruction_2")}</p>
                    <input type="checkbox"/>
                </label>
                <label>
                    <p>{t!("worksheets.iterate.instruction_3")}</p>
                    <input type="checkbox"/>
                </label>
                <label>
                    <p>{t!("worksheets.iterate.instruction_4")}</p>
                    <input type="checkbox"/>
                </label>
            </div>
            <StepperView/>
        </section>
    }
}
