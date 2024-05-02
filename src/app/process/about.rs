use super::StepperView;

use leptos::*;

/// step 1
#[component]
pub fn AboutView() -> impl IntoView {
    let steps = (1..=5)
        .map(|i| {
            view! {
                <li class="pb-3">
                    <h5 class="pb-1 font-bold">
                        {t!(format!("process_intro.s_{i}.title").as_str()).to_string()}
                    </h5>
                    <p>
                        {t!(format!("process_intro.s_{i}.description").as_str()).to_string()}
                    </p>
                </li>
            }
        })
        .collect_view();

    view! {
        <section class="self-stretch w-full flex flex-col justify-between">
            <div class="p-8 my-8 dark:bg-stone-800 rounded-lg">
                <h2 class="w-full text-center text-2xl md:text-3x xl:text-4xl pb-6">{t!("process_intro.title")}</h2>
                <div class="grid lg:grid-cols-2 gap-10">
                    <div class="max-w-prose">
                        <p class="pb-4">{t!("process_intro.description_1")}</p>
                        <p>{t!("process_intro.description_2")}</p>
                    </div>
                    <ol class="row-span-2 max-w-prose">
                        {steps}
                    </ol>
                </div>
            </div>
            <StepperView/>
        </section>
    }
}
