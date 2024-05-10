use leptos::*;
use leptos_meta::*;
use uuid::Uuid;

use crate::app::{
    components::ListInputView,
    form::FormState,
    state::{use_store, State},
};

/// step 3
#[component]
pub fn SolutionView() -> impl IntoView {
    let state = use_store();

    let problem_statement = create_read_slice(state, |s| {
        s.problem.value.get().problem_statement.value.get()
    });

    let solutions_getter = |s: &State| s.solutions.value.get().solutions.clone();
    let solutions_value_add = move |(next, index): (String, Option<usize>)| {
        let next = FormState::new(next);
        let id = next.id;
        state.get().solutions.update(move |p| {
            p.solutions.insert(index.unwrap_or(p.solutions.len()), next);
        });
        id
    };
    let solutions_value_remove = move |id: Uuid| {
        state.get().solutions.update(move |p| {
            p.solutions.retain(|v| v.id != id);
        })
    };

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.solutions.title"), t!("process.title"), t!("name"))}/>
        <form>
            <div class="max-w-prose">
                <p>{t!("worksheets.solutions.instruction")}</p>
            </div>
            <div class="max-w-prose my-2 mx-auto p-4 text-lg rounded border border-slate-300 dark:border-slate-700">
                <Show
                    when={move || !problem_statement.get().is_empty()}
                    fallback={|| view!{
                        <p class="text-sm opacity-80">{t!("util.empty")}</p>
                        }
                    }
                >
                    <p>{problem_statement}</p>
                </Show>
            </div>
            <div class="grid">
                <h4 class="col-span-full text-center text-lg mb-4">
                    {t!("worksheets.solutions.label_solutions")}
                </h4>
                <ListInputView input_type="textarea"
                    signal=state
                    getter=solutions_getter
                    add_value=solutions_value_add
                    remove_value=solutions_value_remove
                    add_entry_text={t!("worksheets.solutions.add_solution").to_string()}
                    placeholder={t!("worksheets.solutions.placeholder_solution").to_string()}
                />
            </div>
        </form>
    }
}
