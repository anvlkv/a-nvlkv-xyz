use std::collections::BTreeSet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use uuid::Uuid;
use web_time::Instant;

use form_signal::FormState;

use crate::app::{
    components::{
        use_wk_ctx, use_wk_state, DescriptionView, HistoryEntry, ListInputView, StringInputView,
        UndoRemove, WorksheetHeader,
    },
    state::ProcessStep,
    tabs_signal, use_lang,
};

/// step 2
#[component]
pub fn ProblemView() -> impl IntoView {
    let wk_state = use_wk_state();
    let wk_ctx = use_wk_ctx();

    let problem_statement = Signal::derive(move || {
        wk_state
            .get()
            .problem
            .try_get()
            .map(|v| v.problem_statement)
            .unwrap_or_default()
    });
    let problem_delete_history = create_rw_signal(vec![]);
    let stakeholder_delete_history = create_rw_signal(vec![]);

    let problems_data = Signal::derive(move || {
        wk_state
            .get()
            .problem
            .try_get()
            .map(|v| v.problems.clone())
            .unwrap_or_default()
    });
    let problems_value_add = move |(next, index): (String, Option<usize>)| {
        let next = FormState::new(next);
        let id = next.id;
        wk_state.get().problem.update(move |p| {
            p.problems.insert(index.unwrap_or(p.problems.len()), next);
        });
        id
    };
    let problems_value_remove = move |id: Uuid| {
        wk_state.get().problem.update(move |p| {
            let i = p.problems.iter().position(|v| v.id == id).unwrap();
            let removed = p.problems.remove(i).get_untracked();
            problem_delete_history.update(|h| h.push((removed, i, Instant::now())));
        })
    };
    let problem_restore = move |(val, at, _): HistoryEntry<String>| {
        wk_state.get().problem.update(move |p| {
            if p.problems.len() >= at {
                p.problems.insert(at, FormState::new(val));
            } else {
                p.problems.push(FormState::new(val));
            }
        })
    };

    let stakeholders_data = Signal::derive(move || {
        wk_state
            .get()
            .problem
            .try_get()
            .map(|v| v.stakeholders.clone())
            .unwrap_or_default()
    });
    let stakeholders_value_add = move |(next, index): (String, Option<usize>)| {
        let next = FormState::new(next);
        let id = next.id;
        wk_state.get().problem.update(move |p| {
            p.stakeholders
                .insert(index.unwrap_or(p.stakeholders.len()), next);
        });
        id
    };
    let stakeholders_value_remove = move |id: Uuid| {
        wk_state.get().problem.update(move |p| {
            let i = p.stakeholders.iter().position(|v| v.id == id).unwrap();
            let removed = p.stakeholders.remove(i).get_untracked();
            stakeholder_delete_history.update(|h| h.push((removed, i, Instant::now())));
        })
    };
    let stakeholder_restore = move |(val, at, _): HistoryEntry<String>| {
        wk_state.get().problem.update(move |p| {
            if p.stakeholders.len() >= at {
                p.stakeholders.insert(at, FormState::new(val));
            } else {
                p.stakeholders.push(FormState::new(val));
            }
        })
    };
    let stakeholders_autocomplete = Signal::derive(move || {
        BTreeSet::from_iter(
            wk_state
                .get()
                .problem
                .get()
                .stakeholders
                .into_iter()
                .map(|s| s.get()),
        )
        .into_iter()
        .collect::<Vec<_>>()
    });

    let tabs = tabs_signal(ProcessStep::Problem);

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.problem.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.problem.title").to_string()}
            description_id="problem"
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_ctx.description_hidden
                toggle_hidden=wk_ctx.toggle_description_hidden
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
                            input_type="text"
                            data=problems_data
                            add_value=problems_value_add
                            remove_value=problems_value_remove
                            add_entry_text={t!("worksheets.problem.add_problem").to_string()}
                            placeholder={t!("worksheets.problem.placeholder_problem").to_string()}
                    />
                    </div>
                    <div>
                        <h4 class="text-xl mb-4">
                            {t!("worksheets.problem.label_stakeholders")}
                        </h4>
                        <ListInputView
                            input_type="text"
                            data=stakeholders_data
                            add_value=stakeholders_value_add
                            remove_value=stakeholders_value_remove
                            add_entry_text={t!("worksheets.problem.add_stakeholder").to_string()}
                            placeholder={t!("worksheets.problem.placeholder_stakeholders").to_string()}
                            autocomplete=stakeholders_autocomplete
                        />
                    </div>
                </div>
                <hr class="border-t border-slate-400 mt-4 mb-8"/>
                <div class="max-w-prose mb-4 whitespace-pre-line">
                    <p>{t!("worksheets.problem.instruction_2")}</p>
                </div>
                <StringInputView
                    input_type="textarea"
                    value=problem_statement
                    placeholder={t!("worksheets.problem.placeholder_2").to_string()}/>
            </form>
        </div>
        <UndoRemove
            history=problem_delete_history
            on_restore=problem_restore
        />
        <UndoRemove
            history=stakeholder_delete_history
            on_restore=stakeholder_restore
        />
    }
}

#[component]
pub fn FixedProblemStatement() -> impl IntoView {
    let state = use_wk_state();
    let lang = use_lang();

    let problem_statement = Signal::derive(move || {
        state
            .get()
            .problem
            .try_get()
            .map(|v| v.problem_statement.get())
            .unwrap_or_default()
    });

    view! {
        <div class="max-w-prose my-2 mx-auto p-4 text-lg rounded border border-slate-300 dark:border-slate-700">
            <Show
                when={move || !problem_statement.get().is_empty()}
                fallback=move || {
                    let href = format!("/{}/process/1", lang.get());
                    view!{
                        <p class="text-sm opacity-80">
                            {t!("util.empty")}
                            {" "}
                            <A href
                                class="underline text-purple-800 dark:text-purple-200"
                            >
                                {t!("worksheets.problem.empty")}
                            </A>
                        </p>
                    }
                }
            >
                <p>{problem_statement}</p>
            </Show>
        </div>
    }
}
