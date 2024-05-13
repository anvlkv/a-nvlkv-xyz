use leptos::*;
use leptos_meta::*;
use uuid::Uuid;
use web_time::Instant;

use form_signal::FormState;

use crate::app::components::{
    use_wk_state, DescriptionView, HistoryEntry, ListInputView, UndoRemove, WorksheetHeader,
};

/// step 3
#[component]
pub fn SolutionView() -> impl IntoView {
    let state = use_wk_state();

    let problem_statement = Signal::derive(move || {
        state
            .get()
            .problem
            .try_get()
            .map(|v| v.problem_statement.get())
            .unwrap_or_default()
    });
    let solution_delete_history = create_rw_signal(vec![]);

    let solutions_data = Signal::derive(move || {
        state
            .get()
            .solutions
            .try_get()
            .map(|v| v.solutions.clone())
            .unwrap_or_default()
    });
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
            let i = p.solutions.iter().position(|v| v.id == id).unwrap();
            let removed = p.solutions.remove(i).get_untracked();
            solution_delete_history.update(|h| h.push((removed, i, Instant::now())));
        })
    };
    let solution_restore = move |(val, at, _): HistoryEntry<String>| {
        state.get().solutions.update(move |p| {
            if p.solutions.len() >= at {
                p.solutions.insert(at, FormState::new(val));
            } else {
                p.solutions.push(FormState::new(val));
            }
        })
    };

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.solutions.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.solutions.title").to_string()}
            description_id="solutions"
            let:child
        >
            <DescriptionView
                hidden=child.hidden
                toggle_hidden=child.toggle_hidden
            >
                <p>{t!("worksheets.solutions.description")}</p>
            </DescriptionView>
        </WorksheetHeader>
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
                <h4 class="text-center text-xl mb-4">
                    {t!("worksheets.solutions.label_solutions")}
                </h4>
                <ListInputView
                    input_type="textarea"
                    data=solutions_data
                    add_value=solutions_value_add
                    remove_value=solutions_value_remove
                    add_entry_text={t!("worksheets.solutions.add_solution").to_string()}
                    placeholder={t!("worksheets.solutions.placeholder_solution").to_string()}
                />
            </div>
        </form>
        <UndoRemove
            history=solution_delete_history
            on_restore=solution_restore
        />
    }
}
