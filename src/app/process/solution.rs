use leptos::*;
use leptos_meta::*;

/// step 3
#[component]
pub fn SolutionView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.solutions.title"), t!("process.title"), t!("name"))}/>
        <section class="grow w-full p-8 my-8 bg-stone-200 dark:bg-stone-800 rounded-lg">
            <h3 class="w-full text-center text-2xl md:text-3x xl:text-4xl pb-6">{t!("worksheets.solutions.title")}</h3>
            <div class="max-w-prose">
                <p class="pb-4">{t!("worksheets.solutions.description")}</p>
                <p>{t!("worksheets.solutions.instruction")}</p>
            </div>
            <div class="grid text-center">
                <h4 class="col-span-full">{t!("worksheets.solutions.label_solutions")}</h4>
            </div>
        </section>
    }
}
