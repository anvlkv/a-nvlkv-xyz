use form_signal::FormState;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use uuid::Uuid;
use web_time::Instant;

use crate::app::{
    components::{
        use_example_ctx, use_wk_ctx, use_wk_state, DescriptionView, HistoryEntry, ListInputView,
        ReadOnlyListView, ReadOnlyView, UndoRemove, WorksheetHeader,
    },
    process::{FixedAssumptionStatement, FixedProblemStatement},
    state::ProcessStep,
    tabs_signal, use_lang,
};

/// step 5
#[component]
pub fn ImplementView() -> impl IntoView {
    let state = use_wk_state();
    let wk_ctx = use_wk_ctx();

    let now_delete_history = create_rw_signal(vec![]);
    let nows_data = Signal::derive(move || {
        state
            .get()
            .implement
            .try_get()
            .map(|v| v.now.clone())
            .unwrap_or_default()
    });
    let nows_value_add = move |(next, index): (String, Option<usize>)| {
        let next = FormState::new(next);
        let id = next.id;
        state.get().implement.update(move |p| {
            p.now.insert(index.unwrap_or(p.now.len()), next);
        });
        id
    };
    let nows_value_remove = move |id: Uuid| {
        state.get().implement.update(move |p| {
            let i = p.now.iter().position(|v| v.id == id).unwrap();
            let removed = p.now.remove(i).get_untracked();
            now_delete_history.update(|h| h.push((removed, i, Instant::now())));
        })
    };
    let now_restore = move |(val, at, _): HistoryEntry<String>| {
        state.get().implement.update(move |p| {
            if p.now.len() >= at {
                p.now.insert(at, FormState::new(val));
            } else {
                p.now.push(FormState::new(val));
            }
        })
    };

    let best_delete_history = create_rw_signal(vec![]);
    let bests_data = Signal::derive(move || {
        state
            .get()
            .implement
            .try_get()
            .map(|v| v.best.clone())
            .unwrap_or_default()
    });
    let bests_value_add = move |(next, index): (String, Option<usize>)| {
        let next = FormState::new(next);
        let id = next.id;
        state.get().implement.update(move |p| {
            p.best.insert(index.unwrap_or(p.best.len()), next);
        });
        id
    };
    let bests_value_remove = move |id: Uuid| {
        state.get().implement.update(move |p| {
            let i = p.best.iter().position(|v| v.id == id).unwrap();
            let removed = p.best.remove(i).get_untracked();
            now_delete_history.update(|h| h.push((removed, i, Instant::now())));
        })
    };
    let best_restore = move |(val, at, _): HistoryEntry<String>| {
        state.get().implement.update(move |p| {
            if p.best.len() >= at {
                p.best.insert(at, FormState::new(val));
            } else {
                p.best.push(FormState::new(val));
            }
        })
    };

    let tabs = tabs_signal(ProcessStep::Implement);

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.implement.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.implement.title").to_string()}
            description_id="implement"
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_ctx.description_hidden
                toggle_hidden=wk_ctx.toggle_description_hidden
            >
                <p class="pb-4 whitespace-pre-line">
                    {t!("worksheets.implement.description_1")}
                </p>
                <p class="whitespace-pre-line">
                    {t!("worksheets.implement.description_2")}
                </p>
            </DescriptionView>
            <form>
                <div class="max-w-prose mb-4 whitespace-pre-line">
                    <p>{t!("worksheets.implement.instruction")}</p>
                </div>
                <FixedProblemStatement/>
                <FixedAssumptionStatement/>
                <div class="grid lg:grid-cols-2 text-center mb-4 gap-6">
                    <div>
                        <h4 class="text-xl mb-2">
                            {t!("worksheets.implement.label_col_1")}
                        </h4>
                        <p class="mb-4">{t!("worksheets.implement.hint_col_1")}</p>
                        <ListInputView
                            input_type="text"
                            data=nows_data
                            add_value=nows_value_add
                            remove_value=nows_value_remove
                            add_entry_text={t!("worksheets.implement.add_now").to_string()}
                            placeholder={t!("worksheets.implement.placeholder_nows").to_string()}
                        />
                    </div>
                    <div>
                        <h4 class="text-xl mb-2">
                            {t!("worksheets.implement.label_col_2")}
                        </h4>
                        <p class="mb-4">{t!("worksheets.implement.hint_col_2")}</p>
                        <ListInputView
                            input_type="text"
                            data=bests_data
                            add_value=bests_value_add
                            remove_value=bests_value_remove
                            add_entry_text={t!("worksheets.implement.add_best").to_string()}
                            placeholder={t!("worksheets.implement.placeholder_bests").to_string()}
                        />
                    </div>
                </div>
            </form>
        </div>
        <UndoRemove
            history=now_delete_history
            on_restore=now_restore
        />
        <UndoRemove
            history=best_delete_history
            on_restore=best_restore
        />
    }
}

#[component]
pub fn ExampleImplementView() -> impl IntoView {
    let lang = use_lang();
    let (wk, example) = use_example_ctx();
    let wk_ctx = use_wk_ctx();

    let tabs = tabs_signal(ProcessStep::Implement);

    let assumption_statement = Signal::derive(move || wk.get().compromise.assumption);
    let problem_statement = Signal::derive(move || wk.get().problem.problem_statement);

    let nows_data = Signal::derive(move || wk.get().implement.now);
    let bests_data = Signal::derive(move || wk.get().implement.best);

    let title = Signal::derive(move || {
        t!(
            "worksheets.implement.example_title",
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
        <Title text={move || format!("{} | {} | {} | {}", example.get().title, t!("worksheets.implement.title"), t!("process.title"), t!("name"))}/>
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
                    <p>{t!("worksheets.implement.instruction")}</p>
                </div>
                <ReadOnlyView
                    value=problem_statement
                />
                <ReadOnlyView
                    value=assumption_statement
                />
                <div class="grid lg:grid-cols-2 text-center mb-4 gap-6">
                    <div>
                        <h4 class="text-xl mb-2">
                            {t!("worksheets.implement.label_col_1")}
                        </h4>
                        <p class="mb-4 italic">{t!("worksheets.implement.hint_col_1")}</p>
                        <ReadOnlyListView
                            value=nows_data
                        />
                    </div>
                    <div>
                        <h4 class="text-xl mb-2">
                            {t!("worksheets.implement.label_col_2")}
                        </h4>
                        <p class="mb-4 italic">{t!("worksheets.implement.hint_col_2")}</p>
                        <ReadOnlyListView
                            value=bests_data
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
