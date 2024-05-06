use leptos::*;
use leptos_meta::*;

/// step 2
#[component]
pub fn ProblemView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.problem.title"), t!("process.title"), t!("name"))}/>

        <div class="max-w-prose">
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
    }
}
