use leptos::*;
use leptos_router::*;

use crate::app::state::{use_store, Example, ExampleParams, WorkSheets};

#[derive(Clone, PartialEq)]
struct ExampleCtx {
    wk: Signal<WorkSheets>,
    example: Signal<Example>,
}

pub fn use_example_ctx() -> (Signal<WorkSheets>, Signal<Example>) {
    let ctx = use_context::<ExampleCtx>().unwrap();

    (
        Signal::derive(move || ctx.wk.try_get().unwrap_or_default()),
        Signal::derive(move || ctx.example.try_get().unwrap_or_default()),
    )
}

#[component]
pub fn ExampleView(children: ChildrenFn) -> impl IntoView {
    let example_id = use_params::<ExampleParams>();
    let state = use_store();

    let example = Signal::derive(move || {
        let examples = state.get().examples;
        example_id
            .get()
            .map(|ex| {
                let found = ex
                    .example
                    .map(move |ex_id| examples.into_iter().find(|d| d.id == ex_id));

                found
            })
            .ok()
            .flatten()
            .flatten()
            .unwrap_or_default()
    });
    let wk = Signal::derive(move || example.get().wk.unwrap_or_default());

    provide_context(ExampleCtx { example, wk });

    children.into_view()
}
