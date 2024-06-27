use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use strum::VariantArray;

use crate::app::{
    components::{ErrorView, Tab, WorksheetDummy, WorksheetView},
    process::*,
    projects::get_projects,
    state::{use_store, ProcessStep, StorageMode},
    use_lang,
};

pub const FULL_SCREEN_MAIN: &str = "full-screen-main";

#[component]
pub fn ProcessView() -> impl IntoView {
    let store = use_store();
    let lang = use_lang();

    let storage_type = create_memo(move |_| {
        store
            .get()
            .storage_preference
            .get()
            .unwrap_or(StorageMode::None)
    });

    let examples = create_resource(
        move || lang.get(),
        |lang| async move { get_projects(lang, 3, 0, true).await.map(|(d, _)| d) },
    );

    let fullscreen_root = create_node_ref::<html::Div>();

    view! {
        <Title text={move || format!("{} | {}", t!("process.title"), t!("name"))}/>
        <div
            class="grow mx-auto w-full max-w-screen-2xl px-6 md:px-8 lg:px-16"
            node_ref={fullscreen_root}
        >
            <div class="contents" id={FULL_SCREEN_MAIN}>
                <noscript>
                    <section class="grow lg:w-full p-8 my-6 lg:my-8 flex items-start mb-3 rounded-lg max-w-prose p-4 bg-amber-200 dark:bg-amber-950 border border-amber-400 dark:brder-amber-800 text-sky-800 dark:text-sky-200 text-lg rounded-xl shadow">
                        <div class="flex flex-col">
                            <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                                <h2 class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3 text-wrap whitespace-break-spaces w-full">{t!("util.js")}</h2>
                            </div>
                        </div>
                    </section>
                </noscript>
                <div class="flex relative flex-col xl:flex-row-reverse items-stretch">
                    <section class="grow p-8 my-6 lg:my-8 lg:mb-20 bg-stone-200 dark:bg-stone-800 rounded-xl shadow">
                        <Transition fallback={WorksheetDummy}>
                            <ErrorBoundary fallback=|err| view! { <ErrorView errors=err/>}>
                                {move || {
                                    if let Some(d) = examples.get() {
                                        _ = d.map_err(ServerFnErrorErr::from)?;
                                    }
                                    leptos::error::Result::<View>::Ok(().into_view())
                                }}
                                {move || {
                                    let storage_type = storage_type.get();
                                    view! {
                                        <WorksheetView
                                            storage_type=storage_type
                                            fs_element=fullscreen_root
                                            examples=examples
                                        >
                                            <Outlet/>
                                        </WorksheetView>
                                    }
                                }}
                            </ErrorBoundary>
                        </Transition>
                    </section>
                    <Suspense>
                        <div class="contents xl:block xl:w-64 fixed-aside min-h-svh shrink-0">
                            <StepperView/>
                        </div>
                    </Suspense>
                </div>
            </div>
        </div>
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
