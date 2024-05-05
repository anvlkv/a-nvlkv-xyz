use leptos::*;
use leptos_meta::*;

/// step 4
#[component]
pub fn CompromiseView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.compromise.title"), t!("process.title"), t!("name"))}/>
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
    }
}
