use leptos::*;

use super::StepperView;

/// step 5
#[component]
pub fn ImplementView() -> impl IntoView {
    view! {
        <section class="self-stretch w-full flex flex-col justify-between">
            <div class="p-8 my-8 dark:bg-stone-800 rounded-lg">
                <h3 class="w-full text-center text-2xl md:text-3x xl:text-4xl pb-6">{t!("worksheets.implement.title")}</h3>
                <div class="max-w-prose">
                    <p class="pb-4">{t!("worksheets.implement.description_1")}</p>
                    <p class="pb-4">{t!("worksheets.implement.description_2")}</p>
                    <p>{t!("worksheets.implement.instruction")}</p>
                </div>
                <div class="grid grid-cols-2 text-center">
                    <div>
                        <h4>{t!("worksheets.implement.label_col_1")}</h4>
                        <p>{t!("worksheets.implement.hint_col_1")}</p>
                    </div>
                    <div>
                        <h4>{t!("worksheets.implement.label_col_2")}</h4>
                        <p>{t!("worksheets.implement.hint_col_2")}</p>
                    </div>
                </div>
            </div>
            <StepperView/>
        </section>
    }
}
