use std::collections::BTreeSet;

use leptos::*;
use leptos_meta::*;
use uuid::Uuid;

use crate::app::{
    components::{ListInputView, StringInputView},
    form::FormState,
    state::{use_store, State},
};

/// step 2
#[component]
pub fn ProblemView() -> impl IntoView {
    let state = use_store();
    let problem_statement =
        create_read_slice(state, |s| s.problem.value.get().problem_statement.clone());

    let problems_getter = |s: &State| s.problem.value.get().problems.clone();
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
            p.problems.retain(|v| v.id != id);
        })
    };

    let stakeholders_getter = |s: &State| s.problem.value.get().stakeholders.clone();
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
            p.stakeholders.retain(|v| v.id != id);
        })
    };
    let stakeholders_autocomplete = Signal::derive(move || {
        BTreeSet::from_iter(
            state
                .get()
                .problem
                .value
                .get()
                .stakeholders
                .into_iter()
                .map(|s| s.value.get()),
        )
        .into_iter()
        .collect::<Vec<_>>()
    });

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.problem.title"), t!("process.title"), t!("name"))}/>

        <form >
            <div class="max-w-prose mb-4">
                <p>{t!("worksheets.problem.instruction_1")}</p>
            </div>
            <div class="grid lg:grid-cols-2 text-center mb-4 gap-6">
                <div>
                    <h4 class="text-lg mb-4">
                        {t!("worksheets.problem.label_problems")}
                    </h4>
                    <ListInputView input_type="text"
                       signal=state
                       getter=problems_getter
                       add_value=problems_value_add
                       remove_value=problems_value_remove
                       add_entry_text={t!("worksheets.problem.add_problem").to_string()}
                       placeholder={t!("worksheets.problem.placeholder_problem").to_string()}/>
                </div>
                <div>
                    <h4 class="text-xl mb-4">
                        {t!("worksheets.problem.label_stakeholders")}
                    </h4>
                    <ListInputView input_type="text"
                       signal=state
                       getter=stakeholders_getter
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
    }
}
