use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::{
    components::{
        use_example_ctx, use_wk_ctx, DescriptionView, ReadOnlyListView, ReadOnlyView,
        WorksheetHeader,
    },
    state::ProcessStep,
    tabs_signal, use_lang,
};

/// step 6
#[component]
pub fn IterateView() -> impl IntoView {
    let tabs = tabs_signal(ProcessStep::Iterate);
    let wk_ctx = use_wk_ctx();

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.iterate.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.iterate.title").to_string()}
            description_id="iterate"
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_ctx.description_hidden
                toggle_hidden=wk_ctx.toggle_description_hidden
            >
                <p class="whitespace-pre-line">
                    {t!("worksheets.iterate.description")}
                </p>
            </DescriptionView>
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
        </div>
    }
}

#[component]
pub fn ExampleIterateView() -> impl IntoView {
    let lang = use_lang();
    let (wk, example) = use_example_ctx();
    let wk_ctx = use_wk_ctx();

    let tabs = tabs_signal(ProcessStep::Iterate);

    // let assumption_statement = Signal::derive(move || wk.get().compromise.assumption);
    // let problem_statement = Signal::derive(move || wk.get().problem.problem_statement);

    // let nows_data = Signal::derive(move || wk.get().implement.now);
    // let bests_data = Signal::derive(move || wk.get().implement.best);

    let title = Signal::derive(move || {
        t!(
            "worksheets.iterate.example_title",
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
        <Title text={move || format!("{} | {} | {} | {}", example.get().title, t!("worksheets.iterate.title"), t!("process.title"), t!("name"))}/>
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
            </div>
        </div>
    }
}
