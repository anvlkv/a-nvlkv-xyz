use leptos::*;
use leptos_meta::*;

/// step 2
#[component]
pub fn ProblemView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.problem.title"), t!("process.title"), t!("name"))}/>
        <section class="grow w-full p-8 my-8 bg-stone-200 dark:bg-stone-800 rounded-lg">
            <h3 class="w-full text-center text-2xl md:text-3x xl:text-4xl pb-6">{t!("worksheets.problem.title")}</h3>
            <div class="max-w-prose">
                <p class="pb-4">{t!("worksheets.problem.description")}</p>
                <p>{t!("worksheets.problem.instruction_1")}</p>
            </div>
            <div class="grid grid-cols-2 text-center">
                <h4>{t!("worksheets.problem.label_problems")}</h4>
                <h4>{t!("worksheets.problem.label_stakeholders")}</h4>
            </div>
            <div class="max-w-prose">
                <p>{t!("worksheets.problem.instruction_2")}</p>
            </div>
            <textarea/>
        </section>
    }
}
