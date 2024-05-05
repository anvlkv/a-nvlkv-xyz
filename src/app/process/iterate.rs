use leptos::*;
use leptos_meta::*;

/// step 6
#[component]
pub fn IterateView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.iterate.title"), t!("process.title"), t!("name"))}/>
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
    }
}
