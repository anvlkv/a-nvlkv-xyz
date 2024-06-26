use form_signal::FormSignal;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use uuid::Uuid;
use web_time::Instant;

use crate::app::{
    components::{
        use_example_ctx, use_wk_ctx, ButtonSize, ButtonView, DescriptionView, DragListCtx,
        HistoryEntry, ListInputView, ReadOnlyListView, ReadOnlyView, StringInputView, UndoRemove,
        WorksheetHeader,
    },
    state::{use_store, Completenes, ProcessStep},
    tabs_signal, use_lang,
};

/// step 2
#[component]
pub fn ProblemView() -> impl IntoView {
    let wk_state = use_wk_ctx();
    let state = use_store();
    let lang = use_lang();
    let link = Signal::derive(move || format!("/{}/process/2", lang.get()));

    log::debug!("render ProblemView");
    let problem_statement = FormSignal::new(
        wk_state.wk_data,
        |s| s.problem.problem_statement,
        |s, p| s.problem.problem_statement = p,
    );

    // let problem_delete_history = create_rw_signal(vec![]);
    // let stakeholder_delete_history = create_rw_signal(vec![]);

    let problems_data = FormSignal::new(
        wk_state.wk_data,
        |s| s.problem.problems,
        |s, p| s.problem.problems = p,
    );

    //     Signal::derive(move || {
    //     wk_state
    //         .get()
    //         .problem
    //         .try_get()
    //         .map(|v| v.problems.clone())
    //         .unwrap_or_default()
    // });
    // let problems_value_add = move |(next, index): (String, Option<usize>)| {
    //     let next = FormState::new(next);
    //     let id = next.id;
    //     wk_state.get().problem.update(move |p| {
    //         p.problems.insert(index.unwrap_or(p.problems.len()), next);
    //     });
    //     id
    // };
    // let problems_value_remove = move |id: Uuid| {
    //     wk_state.get().problem.update(move |p| {
    //         let i = p.problems.iter().position(|v| v.id == id).unwrap();
    //         let removed = p.problems.remove(i).get_untracked();
    //         problem_delete_history.update(|h| h.push((removed, i, Instant::now())));
    //     })
    // };
    // let problem_restore = move |(val, at, _): HistoryEntry<String>| {
    //     wk_state.get().problem.update(move |p| {
    //         if p.problems.len() >= at {
    //             p.problems.insert(at, FormState::new(val));
    //         } else {
    //             p.problems.push(FormState::new(val));
    //         }
    //     })
    // };

    let stakeholders_data = FormSignal::new(
        wk_state.wk_data,
        |s| s.problem.stakeholders,
        |s, d| s.problem.stakeholders = d,
    );

    //     Signal::derive(move || {
    //     wk_state
    //         .get()
    //         .problem
    //         .try_get()
    //         .map(|v| v.stakeholders.clone())
    //         .unwrap_or_default()
    // });
    // let stakeholders_value_add = move |(next, index): (String, Option<usize>)| {
    //     let next = FormState::new(next);
    //     let id = next.id;
    //     wk_state.get().problem.update(move |p| {
    //         p.stakeholders
    //             .insert(index.unwrap_or(p.stakeholders.len()), next);
    //     });
    //     id
    // };
    // let stakeholders_value_remove = move |id: Uuid| {
    //     wk_state.get().problem.update(move |p| {
    //         let i = p.stakeholders.iter().position(|v| v.id == id).unwrap();
    //         let removed = p.stakeholders.remove(i).get_untracked();
    //         stakeholder_delete_history.update(|h| h.push((removed, i, Instant::now())));
    //     })
    // };
    // let stakeholder_restore = move |(val, at, _): HistoryEntry<String>| {
    //     wk_state.get().problem.update(move |p| {
    //         if p.stakeholders.len() >= at {
    //             p.stakeholders.insert(at, FormState::new(val));
    //         } else {
    //             p.stakeholders.push(FormState::new(val));
    //         }
    //     })
    // };
    let stakeholders_autocomplete =
        Signal::derive(move || wk_state.wk_data.get().problem.unique_stakeholders());

    let tabs = tabs_signal(ProcessStep::Problem);

    let disable_statement = Signal::derive(move || {
        let data = wk_state.wk_data.get().problem;
        data.problems
            .iter()
            .filter(|e| !e.is_empty())
            .next()
            .is_none()
            || data
                .stakeholders
                .iter()
                .filter(|e| !e.is_empty())
                .next()
                .is_none()
    });

    let disable_cta = Signal::derive(move || !wk_state.wk_data.get().problem.is_complete());

    DragListCtx::provide(Callback::new(
        move |(entry, list_name, insert_after): (String, String, usize)| {
            wk_state.wk_data.update(|wk| {
                match list_name.as_str() {
                    "problems" => {
                        let pos = if insert_after + 1 <= wk.problem.problems.len() {
                            insert_after + 1
                        } else {
                            wk.problem.problems.len()
                        };
                        wk.problem.problems.insert(pos, entry);
                    }
                    "stakeholders" => {
                        let pos = if insert_after + 1 <= wk.problem.stakeholders.len() {
                            insert_after + 1
                        } else {
                            wk.problem.stakeholders.len()
                        };
                        wk.problem.stakeholders.insert(pos, entry);
                    }
                    _ => {
                        log::warn!("unknown list name");
                    }
                };
            });
        },
    ));

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.problem.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.problem.title").to_string()}
            description_id="problem"
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_state.description_hidden
                toggle_hidden=wk_state.toggle_description_hidden
            >
                <p class="whitespace-pre-line">
                    {t!("worksheets.problem.description")}
                </p>
            </DescriptionView>
            <form>
                <div class="max-w-prose mb-4 whitespace-pre-line">
                    <p>{t!("worksheets.problem.instruction_1")}</p>
                </div>
                <div class="grid lg:grid-cols-2 text-center mb-4 gap-6">
                    <div>
                        <h4 class="text-xl mb-4">
                            {t!("worksheets.problem.label_problems")}
                        </h4>
                        <ListInputView
                            input_type="textarea"
                            value=problems_data
                            // add_value=problems_value_add
                            // remove_value=problems_value_remove
                            add_entry_text={t!("worksheets.problem.add_problem").to_string()}
                            placeholder={t!("worksheets.problem.placeholder_problem").to_string()}
                            drop_target_name="problems"
                    />
                    </div>
                    <div>
                        <h4 class="text-xl mb-4">
                            {t!("worksheets.problem.label_stakeholders")}
                        </h4>
                        <ListInputView
                            input_type="textarea"
                            value=stakeholders_data
                            // add_value=stakeholders_value_add
                            // remove_value=stakeholders_value_remove
                            add_entry_text={t!("worksheets.problem.add_stakeholder").to_string()}
                            placeholder={t!("worksheets.problem.placeholder_stakeholders").to_string()}
                            autocomplete=stakeholders_autocomplete
                            drop_target_name="stakeholders"
                        />
                    </div>
                </div>
                <hr class="border-t border-slate-400 mt-4 mb-8"/>
                <label>
                    <div class="max-w-prose mb-2 whitespace-pre-line">
                        <p>{t!("worksheets.problem.instruction_2")}</p>
                    </div>
                    <StringInputView
                        input_type="textarea"
                        disabled={disable_statement}
                        value=problem_statement
                        placeholder={t!("worksheets.problem.placeholder_2").to_string()}
                    />
                </label>
            </form>
            <div class="flex w-full mt-8 justify-center">
                <ButtonView
                    cta=2
                    size=ButtonSize::Lg
                    disabled={disable_cta}
                    link
                >
                    {t!("worksheets.problem.cta")}
                </ButtonView>
            </div>
        </div>
        // <UndoRemove
        //     history=problem_delete_history
        //     on_restore=problem_restore
        // />
        // <UndoRemove
        //     history=stakeholder_delete_history
        //     on_restore=stakeholder_restore
        // />
    }
}

#[component]
pub fn FixedProblemStatement() -> impl IntoView {
    let state = use_wk_ctx();
    let lang = use_lang();

    let problem_statement = Signal::derive(move || state.wk_data.get().problem.problem_statement);
    let href = Signal::derive(move || Some(format!("/{}/process/1", lang.get())));

    let empty = Signal::derive(move || problem_statement.get().is_empty());

    view! {
        <ReadOnlyView
            fallback_title=t!("worksheets.problem.empty").to_string()
            fallback_href=href
            label=t!("worksheets.problem.label_statement").to_string()
            empty={empty}
        >
            {problem_statement}
        </ReadOnlyView>
    }
}

#[component]
pub fn ExampleProblemView() -> impl IntoView {
    let lang = use_lang();
    let (wk, example) = use_example_ctx();
    let wk_ctx = use_wk_ctx();

    let tabs = tabs_signal(ProcessStep::Problem);

    let problem_statement = Signal::derive(move || wk.get().problem.problem_statement);

    let problems_data = Signal::derive(move || wk.get().problem.problems);

    let stakeholders_data = Signal::derive(move || wk.get().problem.stakeholders);

    let title = Signal::derive(move || {
        t!(
            "worksheets.problem.example_title",
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
        <Title text={move || format!("{} | {} | {} | {}", example.get().title, t!("worksheets.problem.title"), t!("process.title"), t!("name"))}/>
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
            <form role="form">
                <div class="max-w-prose mb-4 whitespace-pre-line italic">
                    <p>{t!("worksheets.problem.instruction_1")}</p>
                </div>
                <div class="grid lg:grid-cols-2 text-center mb-4 gap-6">
                    <div>
                        <h4 class="text-xl mb-4">
                            {t!("worksheets.problem.label_problems")}
                        </h4>
                        <ReadOnlyListView
                            value=problems_data
                        />
                    </div>
                    <div>
                        <h4 class="text-xl mb-4">
                            {t!("worksheets.problem.label_stakeholders")}
                        </h4>
                        <ReadOnlyListView
                            value=stakeholders_data
                        />
                    </div>
                </div>
                <hr class="border-t border-slate-400 mt-4 mb-8"/>
                <div class="max-w-prose mb-4 whitespace-pre-line italic">
                    <p>{t!("worksheets.problem.instruction_2")}</p>
                </div>
                <ReadOnlyView
                    fallback_title=t!("worksheets.problem.empty").to_string()
                    label=t!("worksheets.problem.label_statement").to_string()
                >
                    {problem_statement}
                </ReadOnlyView>
            </form>
        </div>
    }
}
