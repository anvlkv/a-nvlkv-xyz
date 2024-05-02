use leptos::*;

use super::StepperView;

/// step 4
#[component]
pub fn CompromiseView() -> impl IntoView {
    view! {
        <section class="self-stretch w-full flex flex-col justify-between">
            <div class="p-8 my-8 dark:bg-stone-800 rounded-lg">
                <h3 class="w-full text-center text-2xl md:text-3x xl:text-4xl pb-6">{t!("worksheets.compromise.title")}</h3>
                <div class="max-w-prose">
                    <p class="pb-4">{t!("worksheets.compromise.description")}</p>
                    <p>{t!("worksheets.compromise.instruction_1")}</p>
                </div>
                <div class="grid grid-cols-2 text-center">
                    <h4>{t!("worksheets.compromise.label_solutions")}</h4>
                    <h4>{t!("worksheets.compromise.label_stakeholders")}</h4>
                </div>
                <div class="max-w-prose">
                    <p>{t!("worksheets.compromise.instruction_2")}</p>
                </div>
                <label>
                    <p>{t!("worksheets.compromise.label_question")}</p>
                    <textarea/>
                </label>
            </div>
            <StepperView/>
        </section>
    }
}
