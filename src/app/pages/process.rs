use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use strum::VariantArray;
use uuid::Uuid;

use crate::app::{
    components::{PrivacyNoticeView, Tab, WorksheetDummy, WorksheetView},
    process::*,
    state::{use_store, ProcessStep, SeqStep, StorageMode},
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

    let storage_type = create_read_slice(store, |s| {
        s.storage_preference.get().unwrap_or(StorageMode::None)
    });

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

    let section_class =
        "grow lg:w-full p-8 my-6 lg:my-8 bg-stone-200 dark:bg-stone-800 rounded-xl shadow";

    let storage_type = move || storage_type.get();
    let step_view = move || {
        let step = step();
        let storage_type = storage_type();

        match step {
            0 => view! {<LandingView/>}.into_view(),
            1 => view! {
                <section class={section_class}>
                    <div class="flex flex-col">
                        <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                            <h2 class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3">{t!("about.title")}</h2>
                        </div>
                        <div class="grow w-full">
                            <AboutView/>
                        </div>
                    </div>
                </section>
            }
            .into_view(),
            2 => view! {
                <section class={section_class}>
                    <Transition fallback={WorksheetDummy}>
                        <WorksheetView
                            title={t!("worksheets.problem.title")}
                            description_id="problem"
                            description={move || view!{
                                    <p>{t!("worksheets.problem.description")}</p>
                            }}
                            tabs={mock_tabs}
                            storage_type=storage_type
                        >
                                <ProblemView/>
                        </WorksheetView>
                    </Transition>
                </section>
            }
            .into_view(),
            3 => view! {
                <section class={section_class}>
                    <Transition fallback={WorksheetDummy}>
                        <WorksheetView
                            title={t!("worksheets.solutions.title")}
                            description_id="solutions"
                            description={move || view! {
                                <p>{t!("worksheets.solutions.description")}</p>
                            }}
                            storage_type=storage_type
                        >
                                <SolutionView/>
                        </WorksheetView>
                    </Transition>
                </section>
            }
            .into_view(),
            4 => view! {
                <section class={section_class}>
                    <Transition fallback={WorksheetDummy}>
                        <WorksheetView
                            title={t!("worksheets.compromise.title")}
                            description_id="compromise"
                            description={move || view! {
                                    <p>{t!("worksheets.compromise.description")}</p>
                            }}
                            storage_type=storage_type
                        >
                                <CompromiseView/>
                        </WorksheetView>
                    </Transition>
                </section>
            }
            .into_view(),
            5 => view! {
                <section class={section_class}>
                    <Transition fallback={WorksheetDummy}>
                        <WorksheetView
                            title={t!("worksheets.implement.title")}
                            description_id="implement"
                            description={move || view! {
                                <p class="pb-4">{t!("worksheets.implement.description_1")}</p>
                                <p>{t!("worksheets.implement.description_2")}</p>
                            }}
                            storage_type=storage_type
                        >
                                <ImplementView/>
                        </WorksheetView>
                    </Transition>
                </section>
            }
            .into_view(),
            6 => view! {
                <section class={section_class}>
                    <Transition fallback={WorksheetDummy}>
                        <WorksheetView
                            title={t!("worksheets.iterate.title")}
                            description_id="iterate"
                            description={move || view!{
                                    <p>{t!("worksheets.iterate.description")}</p>
                            }}
                            storage_type=storage_type
                        >
                                <IterateView/>
                        </WorksheetView>
                    </Transition>
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
        <Title text={move || format!("{} | {}", t!("process.title"), t!("name"))}/>
        <div class="grow mx-auto w-full max-w-screen-2xl px-6 md:px-8 lg:px-16 flex flex-col sm:flex-row-reverse lg:flex-col lg:justify-stretch lg:items-stretch">
            <Show when={move || step() > 0}>
                <noscript>
                    <section class="grow lg:w-full p-8 my-6 lg:my-8 flex items-start mb-3 rounded-lg max-w-prose p-4 bg-amber-200 dark:bg-amber-950 border border-amber-400 dark:brder-amber-800 text-sky-800 dark:text-sky-200 text-lg rounded-xl shadow">
                        <div class="flex flex-col">
                            <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                                <h2 class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3 text-wrap whitespace-break-spaces w-full">{t!("util.js")}</h2>
                            </div>
                        </div>
                    </section>
                </noscript>
            </Show>
            {step_view}
            <Show when={move || step() > 0}>
                <StepperView/>
            </Show>
            <PrivacyNoticeView/>
        </div>
    }
}
