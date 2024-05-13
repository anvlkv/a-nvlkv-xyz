use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use strum::VariantArray;

use crate::app::{
    components::{PrivacyNoticeView, WorksheetDummy, WorksheetView},
    process::*,
    state::{use_store, ProcessStep, SeqStep, StorageMode},
    Language,
};

#[component]
pub fn ProcessView() -> impl IntoView {
    let store = use_store();
    let lang = use_context::<Signal<Language>>().unwrap();

    let storage_type = create_read_slice(store, |s| {
        s.storage_preference.get().unwrap_or(StorageMode::None)
    });

    create_effect(move |_| {
        if store.get().sequence.is_empty() {
            let lang = lang.get().0;

            store.update(|s| {
                s.sequence = ProcessStep::VARIANTS
                    .iter()
                    .enumerate()
                    .map(|(i, step)| SeqStep {
                        href: format!("/{lang}/process/{}", i),
                        process_step: *step,
                    })
                    .collect();
            })
        }
    });

    view! {
        <Title text={move || format!("{} | {}", t!("process.title"), t!("name"))}/>
        <noscript>
            <section class="grow lg:w-full p-8 my-6 lg:my-8 flex items-start mb-3 rounded-lg max-w-prose p-4 bg-amber-200 dark:bg-amber-950 border border-amber-400 dark:brder-amber-800 text-sky-800 dark:text-sky-200 text-lg rounded-xl shadow">
                <div class="flex flex-col">
                    <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                        <h2 class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3 text-wrap whitespace-break-spaces w-full">{t!("util.js")}</h2>
                    </div>
                </div>
            </section>
        </noscript>
        <section class="grow lg:w-full p-8 my-6 lg:my-8 bg-stone-200 dark:bg-stone-800 rounded-xl shadow">
            {move || {
                let storage_type = storage_type.get();
                view! {
                    <Transition fallback={WorksheetDummy}>
                        <WorksheetView
                            storage_type=storage_type
                        >
                            <Outlet/>
                        </WorksheetView>
                    </Transition>
                }
            }}
        </section>
        <StepperView/>
        <PrivacyNoticeView/>
    }
}
