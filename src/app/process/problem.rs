use std::collections::BTreeSet;

use leptos::*;
use leptos_meta::*;
use uuid::Uuid;
use web_time::Instant;

use form_signal::FormState;

use crate::app::components::{
    use_wk_state, DescriptionView, HistoryEntry, ListInputView, StringInputView, UndoRemove,
    WorksheetHeader,
};

/// step 2
#[component]
pub fn ProblemView() -> impl IntoView {
    let state = use_wk_state();

    let problem_statement = Signal::derive(move || {
        state
            .get()
            .problem
            .try_get()
            .map(|v| v.problem_statement)
            .unwrap_or_default()
    });
    let problem_delete_history = create_rw_signal(vec![]);
    let stakeholder_delete_history = create_rw_signal(vec![]);

    let problems_data = Signal::derive(move || {
        state
            .get()
            .problem
            .try_get()
            .map(|v| v.problems.clone())
            .unwrap_or_default()
    });
    let problems_value_add = move |(next, index): (String, Option<usize>)| {
        let next = FormState::new(next);
        let id = next.id;
        state.get().problem.update(move |p| {
            p.problems.insert(index.unwrap_or(p.problems.len()), next);
        });
        id
    };
    let problems_value_remove = move |id: Uuid| {
        state.get().problem.update(move |p| {
            let i = p.problems.iter().position(|v| v.id == id).unwrap();
            let removed = p.problems.remove(i).get_untracked();
            problem_delete_history.update(|h| h.push((removed, i, Instant::now())));
        })
    };
    let problem_restore = move |(val, at, _): HistoryEntry<String>| {
        state.get().problem.update(move |p| {
            if p.problems.len() >= at {
                p.problems.insert(at, FormState::new(val));
            } else {
                p.problems.push(FormState::new(val));
            }
        })
    };

    let stakeholders_data = Signal::derive(move || {
        state
            .get()
            .problem
            .try_get()
            .map(|v| v.stakeholders.clone())
            .unwrap_or_default()
    });
    let stakeholders_value_add = move |(next, index): (String, Option<usize>)| {
        let next = FormState::new(next);
        let id = next.id;
        state.get().problem.update(move |p| {
            p.stakeholders
                .insert(index.unwrap_or(p.stakeholders.len()), next);
        });
        id
    };
    let stakeholders_value_remove = move |id: Uuid| {
        state.get().problem.update(move |p| {
            let i = p.stakeholders.iter().position(|v| v.id == id).unwrap();
            let removed = p.stakeholders.remove(i).get_untracked();
            stakeholder_delete_history.update(|h| h.push((removed, i, Instant::now())));
        })
    };
    let stakeholder_restore = move |(val, at, _): HistoryEntry<String>| {
        state.get().problem.update(move |p| {
            if p.stakeholders.len() >= at {
                p.stakeholders.insert(at, FormState::new(val));
            } else {
                p.stakeholders.push(FormState::new(val));
            }
        })
    };
    let stakeholders_autocomplete = Signal::derive(move || {
        BTreeSet::from_iter(
            state
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

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.problem.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.problem.title").to_string()}
            description_id="problem"
            let:child
        >
            <DescriptionView
                hidden=child.hidden
                toggle_hidden=child.toggle_hidden
            >
                <p>{t!("worksheets.problem.description")}</p>
            </DescriptionView>
        </WorksheetHeader>
        <form>
            <div class="max-w-prose mb-4">
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
            <div class="max-w-prose mb-4">
                <p>{t!("worksheets.problem.instruction_2")}</p>
            </div>
            <StringInputView
                input_type="textarea"
                value=problem_statement
                placeholder={t!("worksheets.problem.placeholder_2").to_string()}/>
        </form>
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
