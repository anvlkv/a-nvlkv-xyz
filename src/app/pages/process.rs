use leptos::*;
use leptos_router::*;
use strum::VariantArray;
use uuid::Uuid;

use crate::app::{
    components::{Tab, WorksheetView},
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

    let mock_tabs = Signal::derive(move || {
        vec![
            Tab {
                title: "title 1".to_string(),
                href: "/en/2".to_string(),
            },
            Tab {
                title: "somewhat long title 1".to_string(),
                href: "/1".to_string(),
            },
        ]
    });

    let section_class = "grow w-full p-8 my-8 bg-stone-200 dark:bg-stone-800 rounded-xl shadow";

    let step_view = move || {
        let step = step();
        match step {
            0 => view! {<LandingView/>}.into_view(),
            1 => view! {
                <section class={section_class}>
                    <WorksheetView title={t!("about.title")} >
                        <AboutView/>
                    </WorksheetView>
                </section>
            }
            .into_view(),
            2 => view! {
                <section class={section_class}>
                    <WorksheetView title={t!("worksheets.problem.title")} tabs={mock_tabs}>
                        <ProblemView/>
                    </WorksheetView>
                </section>
            }
            .into_view(),
            3 => view! {
                <section class={section_class}>
                    <WorksheetView title={t!("worksheets.solutions.title")} >
                        <SolutionView/>
                    </WorksheetView>
                </section>
            }
            .into_view(),
            4 => view! {
                <section class={section_class}>
                    <WorksheetView title={t!("worksheets.compromise.title")} >
                        <CompromiseView/>
                    </WorksheetView>
                </section>
            }
            .into_view(),
            5 => view! {
                <section class={section_class}>
                    <WorksheetView title={t!("worksheets.implement.title")} >
                        <ImplementView/>
                    </WorksheetView>
                </section>
            }
            .into_view(),
            6 => view! {
                <section class={section_class}>
                    <WorksheetView title={t!("worksheets.iterate.title")} >
                        <IterateView/>
                    </WorksheetView>
                </section>
            }
            .into_view(),
            _ => view! {
                <section class={section_class}>
                    <InquireView/>
                </section>
            }
            .into_view(),
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
