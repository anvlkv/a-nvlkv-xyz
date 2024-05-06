use leptos::*;
use leptos_meta::*;

/// step 3
#[component]
pub fn SolutionView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.solutions.title"), t!("process.title"), t!("name"))}/>

        <div class="max-w-prose">
            <p>{t!("worksheets.solutions.instruction")}</p>
        </div>
        <div class="grid text-center">
            <h4 class="col-span-full">{t!("worksheets.solutions.label_solutions")}</h4>
        </div>
    }
}
