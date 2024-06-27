use form_signal::FormSignal;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use uuid::Uuid;
use web_time::Instant;

use crate::app::{
    components::{
        use_example_ctx, use_wk_ctx, ButtonSize, ButtonView, DescriptionView, HistoryEntry,
        ListInputView, ReadOnlyListView, ReadOnlyView, UndoRemove, WorksheetHeader,
    },
    process::FixedProblemStatement,
    state::{use_store, Completenes, ProcessStep},
    tabs_signal, use_lang,
};

/// step 3
#[component]
pub fn SolutionView() -> impl IntoView {
    let state = use_store();
    let wk_state = use_wk_ctx();
    let lang = use_lang();
    let link = Signal::derive(move || format!("/{}/process/3", lang.get()));

    // let solution_delete_history = create_rw_signal(vec![]);

    let solutions_data = FormSignal::new(
        wk_state.wk_data,
        |s| s.solutions.solutions,
        |s, d| s.solutions.solutions = d,
    );
    // let solutions_value_add = move |(next, index): (String, Option<usize>)| {
    //     let next = FormState::new(next);
    //     let id = next.id;
    //     wk_state.get().solutions.update(move |p| {
    //         p.solutions.insert(index.unwrap_or(p.solutions.len()), next);
    //     });
    //     id
    // };
    // let solutions_value_remove = move |id: Uuid| {
    //     wk_state.get().solutions.update(move |p| {
    //         let i = p.solutions.iter().position(|v| v.id == id).unwrap();
    //         let removed = p.solutions.remove(i).get_untracked();
    //         solution_delete_history.update(|h| h.push((removed, i, Instant::now())));
    //     })
    // };
    // let solution_restore = move |(val, at, _): HistoryEntry<String>| {
    //     wk_state.get().solutions.update(move |p| {
    //         if p.solutions.len() >= at {
    //             p.solutions.insert(at, FormState::new(val));
    //         } else {
    //             p.solutions.push(FormState::new(val));
    //         }
    //     })
    // };

    let tabs = tabs_signal(ProcessStep::Solution);

    let disable_cta = Signal::derive(move || !wk_state.wk_data.get().solutions.is_complete());

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.solutions.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.solutions.title").to_string()}
            description_id="solutions"
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_state.description_hidden
                toggle_hidden=wk_state.toggle_description_hidden
            >
                <p class="whitespace-pre-line">
                    {t!("worksheets.solutions.description")}
                </p>
            </DescriptionView>
            <form>
                <div class="max-w-prose mb-4 whitespace-pre-line">
                    <p>{t!("worksheets.solutions.instruction")}</p>
                </div>
                <FixedProblemStatement/>
                <div class="grid">
                    <h4 class="text-center text-xl mb-4">
                        {t!("worksheets.solutions.label_solutions")}
                    </h4>
                    <ListInputView
                        input_type="textarea"
                        value=solutions_data
                        // add_value=solutions_value_add
                        // remove_value=solutions_value_remove
                        add_entry_text={t!("worksheets.solutions.add_solution").to_string()}
                        placeholder={t!("worksheets.solutions.placeholder_solution").to_string()}
                    />
                </div>
            </form>
            <div class="flex w-full mt-8 justify-center">
                <ButtonView
                    cta=2
                    size=ButtonSize::Lg
                    disabled={disable_cta}
                    link
                >
                    {t!("worksheets.solutions.cta")}
                </ButtonView>
            </div>
        </div>
        // <UndoRemove
        //     history=solution_delete_history
        //     on_restore=solution_restore
        // />
    }
}

#[component]
pub fn ExampleSolutionView() -> impl IntoView {
    let lang = use_lang();
    let (wk, example) = use_example_ctx();
    let wk_ctx = use_wk_ctx();

    let tabs = tabs_signal(ProcessStep::Solution);

    let problem_statement = Signal::derive(move || wk.get().problem.problem_statement);

    let solutions_data = Signal::derive(move || wk.get().solutions.solutions);

    let title = Signal::derive(move || {
        t!(
            "worksheets.solutions.example_title",
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
        <Title text={move || format!("{} | {} | {} | {}", example.get().title, t!("worksheets.solutions.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title
            description_id=example_id
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_ctx.description_hidden
                toggle_hidden=wk_ctx.toggle_description_hidden
                alternative=1
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
                    <p>{t!("worksheets.solutions.instruction")}</p>
                </div>
                <ReadOnlyView
                    label=t!("worksheets.problem.label_statement").to_string()
                >
                    {problem_statement}
                </ReadOnlyView>
                <div class="grid">
                    <h4 class="text-center text-xl mb-4">
                        {t!("worksheets.solutions.label_solutions")}
                    </h4>
                    <ReadOnlyListView
                        value=solutions_data
                    />
                </div>
            </div>
        </div>
    }
}
