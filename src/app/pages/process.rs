use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use strum::VariantArray;

use crate::app::{
    components::{ErrorView, PrivacyNoticeView, Tab, WorksheetDummy, WorksheetView},
    process::*,
    projects::get_projects,
    state::{use_store, ProcessStep, ProjectData, SeqStep, StorageMode},
    use_lang, Language,
};

#[component]
pub fn ProcessView() -> impl IntoView {
    let store = use_store();
    let lang = use_lang();

    let storage_type = create_read_slice(store, |s| {
        s.storage_preference.get().unwrap_or(StorageMode::None)
    });

    let examples = create_resource(
        move || lang.get(),
        |lang| async move { get_projects(lang, 3, 0, true).await.map(|(d, _)| d) },
    );

    let process_view_with_data = Signal::derive(move || {
        let lang = store.get().lang;
        if let Some(data) = examples.get() {
            let examples = data.map_err(|e| ServerFnErrorErr::from(e))?;
            store.update(|s| {
                s.examples = examples;
                s.sequence = vec![];
                make_sequence(&mut s.sequence, &s.examples, lang);
            });
        } else {
            store.update(|s| {
                s.sequence = ProcessStep::VARIANTS
                    .iter()
                    .enumerate()
                    .map(|(i, step)| SeqStep {
                        href: format!("/{}/process/{}", lang, i),
                        process_step: *step,
                        example: None,
                    })
                    .collect();
            });
        }
        let storage_type = storage_type.get();

        leptos::error::Result::<View>::Ok(view! {
            <WorksheetView
                storage_type=storage_type
            >
                <Outlet/>
            </WorksheetView>
        })
    });

    view! {
        <Title text={move || format!("{} | {}", t!("process.title"), t!("name"))}/>
        <div class="grow mx-auto w-full max-w-screen-2xl px-6 md:px-8 lg:px-16">
            <noscript>
                <section class="grow lg:w-full p-8 my-6 lg:my-8 flex items-start mb-3 rounded-lg max-w-prose p-4 bg-amber-200 dark:bg-amber-950 border border-amber-400 dark:brder-amber-800 text-sky-800 dark:text-sky-200 text-lg rounded-xl shadow">
                    <div class="flex flex-col">
                        <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                            <h2 class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3 text-wrap whitespace-break-spaces w-full">{t!("util.js")}</h2>
                        </div>
                    </div>
                </section>
            </noscript>
            <div class="flex flex-col xl:flex-row-reverse">
                <section class="grow lg:w-full p-8 my-6 lg:my-8 bg-stone-200 dark:bg-stone-800 rounded-xl shadow">
                    <Transition fallback={WorksheetDummy}>
                        <ErrorBoundary fallback=|err| view! { <ErrorView errors=err/>}>
                            {process_view_with_data}
                        </ErrorBoundary>
                    </Transition>
                </section>
                <Suspense>
                    <StepperView/>
                </Suspense>
            </div>
        </div>
        <PrivacyNoticeView/>
    }
}

pub fn tabs_signal(step: ProcessStep) -> Signal<Vec<Tab>> {
    let state = use_store();

    let step_num: usize = ProcessStep::VARIANTS
        .iter()
        .position(|s| *s == step)
        .unwrap();

    Signal::derive(move || {
        let s = state.get();
        if s.examples.len() == 0 {
            return vec![];
        }

        let mut tabs = vec![Tab {
            title: t!("worksheets.wk").to_string(),
            href: format!("/{}/process/{step_num}", s.lang),
            is_example: false,
        }];
        tabs.extend(s.examples.into_iter().map(|ex| Tab {
            title: t!("worksheets.example", title = ex.title).to_string(),
            href: format!("/{}/process/{step_num}/{}", s.lang, ex.id),
            is_example: true,
        }));
        tabs
    })
}

fn make_sequence(seq: &mut Vec<SeqStep>, examples: &Vec<ProjectData>, lang: Language) {
    // about
    seq.push(SeqStep {
        href: format!("/{}/process/{}", lang, 0),
        process_step: ProcessStep::About,
        example: None,
    });

    // all worksheets first example
    examples.first().iter().for_each(|ex| {
        seq.extend(
            ProcessStep::VARIANTS
                .iter()
                .enumerate()
                .filter_map(|(i, step)| {
                    if i > 0 && i < ProcessStep::VARIANTS.len() - 1 {
                        Some(SeqStep {
                            href: format!("/{}/process/{}/{}", lang, i, ex.id),
                            process_step: *step,
                            example: Some(ex.id.clone()),
                        })
                    } else {
                        None
                    }
                }),
        );
    });

    // each workshet examples
    seq.extend(
        ProcessStep::VARIANTS
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (i, step)| {
                if i > 0 && i < ProcessStep::VARIANTS.len() - 1 {
                    // example
                    acc.extend(examples.iter().skip(1).map(|ex| SeqStep {
                        href: format!("/{}/process/{}/{}", lang, i, ex.id),
                        process_step: *step,
                        example: Some(ex.id.clone()),
                    }));
                    // worksheet
                    acc.push(SeqStep {
                        href: format!("/{}/process/{}", lang, i),
                        process_step: *step,
                        example: None,
                    });
                }
                acc
            }),
    );

    // inquire
    seq.push(SeqStep {
        href: format!("/{}/process/{}", lang, 6),
        process_step: ProcessStep::Inquire,
        example: None,
    });
}
