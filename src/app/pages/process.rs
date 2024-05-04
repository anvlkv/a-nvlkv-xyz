use leptos::*;
use leptos_router::*;
use strum::VariantArray;
use uuid::Uuid;

use crate::app::{
    process::*,
    state::{use_store, ProcessStep, SeqStep},
    Language,
};

#[derive(Params, PartialEq, Clone)]
struct ProcessParams {
    step: Option<usize>,
    example_id: Option<Uuid>,
}

#[component]
pub fn ProcessView() -> impl IntoView {
    let params = use_params::<ProcessParams>();
    let store = use_store();
    let lang = use_context::<Signal<Language>>().unwrap();

    let step = move || {
        params
            .get()
            .map(|p| p.step)
            .ok()
            .flatten()
            .unwrap_or_default()
    };

    create_effect(move |_| {
        let step = step();
        if store.get().sequence.is_empty() && step > 0 {
            let lang = lang.get().0;

            store.update(|s| {
                s.sequence = ProcessStep::VARIANTS
                    .iter()
                    .enumerate()
                    .map(|(i, step)| SeqStep {
                        href: format!("/{lang}/{}", i + 1),
                        process_step: *step,
                    })
                    .collect();
            })
        }
    });

    let step_view = move || {
        let step = step();
        match step {
            0 => view! {<LandingView/>}.into_view(),
            1 => view! {<AboutView/>}.into_view(),
            2 => view! {<ProblemView/>}.into_view(),
            3 => view! {<SolutionView/>}.into_view(),
            4 => view! {<CompromiseView/>}.into_view(),
            5 => view! {<ImplementView/>}.into_view(),
            6 => view! {<IterateView/>}.into_view(),
            _ => view! {<InquireView/>}.into_view(),
        }
    };

    view! {
        <div class="mx-auto max-w-screen-2xl px-6 md:px-8 lg:px-16 min-h-full flex flex-col justify-stretch items-center">
            {step_view}
            <Show when={move || step() > 0}>
                <StepperView/>
            </Show>
        </div>
    }
}
