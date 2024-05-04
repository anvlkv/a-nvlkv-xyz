use leptos::*;
use leptos_meta::*;

/// step 6
#[component]
pub fn IterateView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.iterate.title"), t!("process.title"), t!("name"))}/>
        <section class="grow w-full p-8 my-8 bg-stone-200 dark:bg-stone-800 rounded-lg">
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
        </section>
    }
}
