use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::{
    components::{use_wk_state, DescriptionView, StringInputView, WorksheetHeader},
    process::FixedProblemStatement,
    Language,
};

/// step 4
#[component]
pub fn CompromiseView() -> impl IntoView {
    let state = use_wk_state();

    let assumption_statement = Signal::derive(move || {
        state
            .get()
            .compromise
            .try_get()
            .map(|v| v.assumption)
            .unwrap_or_default()
    });

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.compromise.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.compromise.title").to_string()}
            description_id="compromise"
            let:child
        >
            <DescriptionView
                hidden=child.hidden
                toggle_hidden=child.toggle_hidden
            >
                <p class="whitespace-pre-line">
                    {t!("worksheets.compromise.description")}
                </p>
            </DescriptionView>
        </WorksheetHeader>
        <form>
            <div class="max-w-prose mb-4 whitespace-pre-line">
                <p>{t!("worksheets.compromise.instruction_1")}</p>
            </div>
            <FixedProblemStatement/>
            <div class="grid grid-cols-2 text-center">
                <h4 class="text-xl mb-4">
                    {t!("worksheets.compromise.label_solutions")}
                </h4>
                <h4 class="text-xl mb-4">
                    {t!("worksheets.compromise.label_stakeholders")}
                </h4>
            </div>
            <hr class="border-t border-slate-400 mt-4 mb-8"/>
            <div class="max-w-prose mb-4 whitespace-pre-line">
                <p>{t!("worksheets.compromise.instruction_2")}</p>
            </div>
            <label>
                <p class="mb-2">{t!("worksheets.compromise.label_question")}</p>
                <StringInputView
                    input_type="textarea"
                    value=assumption_statement
                    placeholder={t!("worksheets.compromise.placeholder").to_string()}/>
            </label>
        </form>
    }
}

#[component]
pub fn FixedAssumptionStatement() -> impl IntoView {
    let state = use_wk_state();
    let lang = use_context::<Signal<Language>>().unwrap();

    let assumption = Signal::derive(move || {
        state
            .get()
            .compromise
            .try_get()
            .map(|v| v.assumption.get())
            .unwrap_or_default()
    });

    view! {
        <div class="max-w-prose my-2 mx-auto p-4 text-lg rounded border border-slate-300 dark:border-slate-700">
            <Show
                when={move || !assumption.get().is_empty()}
                fallback=move || {
                    let href = format!("/{}/process/3", lang.get().0);
                    view!{
                        <p class="text-sm opacity-80">
                            {t!("util.empty")}
                            {" "}
                            <A href
                                class="underline text-purple-800 dark:text-purple-200"
                            >
                                {t!("worksheets.compromise.empty")}
                            </A>
                        </p>
                    }
                }
            >
                <p>{assumption}</p>
            </Show>
        </div>
    }
}
