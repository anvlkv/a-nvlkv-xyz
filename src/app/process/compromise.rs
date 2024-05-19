use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::{
    components::{
        use_example_ctx, use_wk_ctx, use_wk_state, DescriptionView, ReadOnlyListView, ReadOnlyView,
        StringInputView, WorksheetHeader,
    },
    process::FixedProblemStatement,
    state::ProcessStep,
    tabs_signal, use_lang,
};

/// step 4
#[component]
pub fn CompromiseView() -> impl IntoView {
    let state = use_wk_state();
    let wk_ctx = use_wk_ctx();

    let assumption_statement = Signal::derive(move || {
        state
            .get()
            .compromise
            .try_get()
            .map(|v| v.assumption)
            .unwrap_or_default()
    });

    let tabs = tabs_signal(ProcessStep::Compromise);

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.compromise.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.compromise.title").to_string()}
            description_id="compromise"
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_ctx.description_hidden
                toggle_hidden=wk_ctx.toggle_description_hidden
            >
                <p class="whitespace-pre-line">
                    {t!("worksheets.compromise.description")}
                </p>
            </DescriptionView>
            <form>
                <div class="max-w-prose mb-4 whitespace-pre-line">
                    <p>{t!("worksheets.compromise.instruction_1")}</p>
                </div>
                <FixedProblemStatement/>
                <div class="grid grid-cols-2">
                    <div>
                        <h4 class="text-xl mb-4 w-full text-center">
                            {t!("worksheets.compromise.label_solutions")}
                        </h4>

                    </div>
                    <div>
                        <h4 class="text-xl mb-4 w-full text-center">
                            {t!("worksheets.compromise.label_stakeholders")}
                        </h4>

                    </div>
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
        </div>
    }
}

#[component]
pub fn FixedAssumptionStatement() -> impl IntoView {
    let state = use_wk_state();
    let lang = use_lang();

    let assumption = Signal::derive(move || {
        state
            .get()
            .compromise
            .try_get()
            .map(|v| v.assumption.get())
            .unwrap_or_default()
    });

    let href = Signal::derive(move || Some(format!("/{}/process/3", lang.get())));

    view! {
        <ReadOnlyView
            value=assumption
            fallback_title=t!("worksheets.compromise.empty").to_string()
            fallback_href=href
        />
    }
}

#[component]
pub fn ExampleCompromiseView() -> impl IntoView {
    let lang = use_lang();
    let (wk, example) = use_example_ctx();
    let wk_ctx = use_wk_ctx();

    let tabs = tabs_signal(ProcessStep::Compromise);

    let assumption_statement = Signal::derive(move || wk.get().compromise.assumption);
    let problem_statement = Signal::derive(move || wk.get().problem.problem_statement);

    let solutions_data = Signal::derive(move || wk.get().solutions.solutions);
    let stakeholders_data = Signal::derive(move || wk.get().problem.stakeholders);

    let title = Signal::derive(move || {
        t!(
            "worksheets.compromise.example_title",
            title = example.get().title
        )
        .to_string()
    });
    let example_id = Signal::derive(move || example.get().id);

    let case_href = move || {
        let id = example_id.get();
        let lang = lang.get();
        format!("/{lang}/projects/{id}")
    };

    view! {
        <Title text={move || format!("{} | {} | {} | {}", example.get().title, t!("worksheets.compromise.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title
            description_id=example_id
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_ctx.description_hidden
                toggle_hidden=wk_ctx.toggle_description_hidden
                alternative=true
            >
                <p class="whitespace-pre-line">
                    {move || example.get().description}
                </p>
                <A href=case_href attr:class="underline">
                    {move || t!("worksheets.view_example", title=example.get().title)}
                </A>
            </DescriptionView>
            <div role="form">
                <div class="max-w-prose mb-4 whitespace-pre-line italic">
                    <p>{t!("worksheets.compromise.instruction_1")}</p>
                </div>
                <ReadOnlyView
                    value=problem_statement
                />
                <div class="grid grid-cols-2">
                    <div>
                        <h4 class="text-xl mb-4 w-full text-center">
                            {t!("worksheets.compromise.label_solutions")}
                        </h4>
                        <ReadOnlyListView
                            value=solutions_data
                        />
                    </div>
                    <div>
                        <h4 class="text-xl mb-4 w-full text-center">
                            {t!("worksheets.compromise.label_stakeholders")}
                        </h4>
                        <ReadOnlyListView
                            value=stakeholders_data
                        />
                    </div>
                </div>
                <hr class="border-t border-slate-400 mt-4 mb-8"/>
                <div class="max-w-prose mb-4 whitespace-pre-line italic">
                    <p>{t!("worksheets.compromise.instruction_2")}</p>
                </div>
                <ReadOnlyView
                    value=assumption_statement
                />
            </div>
        </div>
    }
}
