use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use web_time::Duration;

use crate::app::{
    components::{use_wk_state, ButtonView, Localized, RvArtboardView, WorksheetView},
    state::{use_store, StorageMode, WorkSheets},
};

#[component]
pub fn WorksheetsDownload() -> impl IntoView {
    let store = use_store();
    let storage_type = create_read_slice(store, |s| {
        s.storage_preference.get().unwrap_or(StorageMode::None)
    });

    view! {
        <Localized>
        {
            move || {
                let storage_type = storage_type.get();

                view!{
                    <WorksheetView
                        storage_type=storage_type
                    >
                        <PrintView/>
                    </WorksheetView>
                }
            }
        }
        </Localized>
    }
}

#[component]
fn PrintView() -> impl IntoView {
    let ctx = use_wk_state();
    let data = Signal::derive(move || {
        let wk: WorkSheets = ctx.get().get();
        wk
    });
    let problem_statement = move || data.get().problem.problem_statement;
    let solutions = move || data.get().compromise.solution_choices;
    let stakeholders = move || data.get().compromise.stakeholder_choices;
    let question = move || data.get().compromise.question;
    let iteration_title = move || data.get().iterate.title;
    let iteration_start_date = move || data.get().iterate.start_date;
    let iteration_end_date = move || data.get().iterate.end_date;
    let iteration_resources = move || data.get().iterate.resources;
    let iteration_external_resources = move || data.get().iterate.external_resources;
    let now = move || data.get().implement.now;
    let best = move || data.get().implement.best;

    let (rv_loaded, set_rv_loaded) = create_signal(HashSet::<&'static str>::from_iter(vec![
        "Inquire",
        "Problem",
        "Compromise",
        "Iterate",
        "Implement",
    ]));

    create_effect(move |_| {
        let body = document().body().unwrap();
        body.class_list().add_2("A4", "landscape").unwrap();
    });

    create_effect(move |_| {
        if rv_loaded.get().is_empty() {
            // TODO: emit event from rive artboards once played the stale state
            set_timeout(
                || {
                    let win = window();
                    _ = win.print();
                },
                Duration::from_millis(200),
            )
        }
    });

    let on_rv_loaded = Callback::new(move |name: String| {
        set_rv_loaded.update(|v| {
            v.remove(name.as_str());
        })
    });

    view! {
        <Title text={move || format!("{}_{}_{}", iteration_title(), t!("about.title"), t!("name")).replace(&['.', '|', '/', '\\', '>', '<', '!', '?', '*'], "-")}/>
        <ButtonView
            cta=2
            on:click={move |_| {
                let win = window();
                _ = win.print();
            }}
            disabled={Signal::derive(move || !rv_loaded.get().is_empty())}
            attr:class="z-10 fixed print:hidden top-10 right-10"
        >
            {t!("util.print")}
        </ButtonView>
        <div
        class="font-sans text-black -mt-40 -ml-40  print:mt-0 print:ml-0 scale-50 print:scale-100"
        >
            <div class="sheet padding-10mm">
                <header class="flex gap-2 w-full justify-start items-center border-b border-gray-500">
                    <RvArtboardView
                        name="Inquire"
                        state_machine="Inquire State Machine"
                        attr:class="w-16 h-16"
                        on_loaded={on_rv_loaded}
                    />
                    <h1 class="text-2xl">
                        {t!("about.title")}
                    </h1>
                    <div class="h-full text-sm pl-4 ml-auto mr-0 border-l border-gray-500">
                        <p>{t!("worksheets.download.signature")}</p>
                        <a href="https://a.nvlkv.xyz" class="underline">{"https://a.nvlkv.xyz"}</a>
                    </div>
                </header>

                <main class="contents">
                    <section class="flex gap-2 w-full py-4 border-b border-gray-500">
                        <RvArtboardView
                            name="Iterate"
                            state_machine="Iterate State Machine"
                            attr:class="w-16 h-16"
                            on_loaded={on_rv_loaded}
                        />
                        <div class="grow">
                            <h2 class="text-xl">
                                {t!("worksheets.download.iteration")}{": "} {iteration_title}
                            </h2>
                            <div class="grid grid-cols-2 w-full">
                                <p>
                                    {t!("worksheets.iterate.label_date_1")}{": "}{iteration_start_date}
                                </p>
                                <p>
                                    {t!("worksheets.iterate.label_date_2")}{": "}{iteration_end_date}
                                </p>
                            </div>
                        </div>
                    </section>
                    <section class="flex gap-2 w-full py-4 border-b border-gray-500">
                        <RvArtboardView
                            name="Problem"
                            state_machine="Problem State Machine"
                            attr:class="w-16 h-16"
                            on_loaded={on_rv_loaded}
                        />
                        <div class="grow flex gap-2 items-baseline">
                            <h2 class="text-xl">
                                {t!("worksheets.download.problem")}{": "}
                            </h2>
                            <p>
                                {problem_statement}
                            </p>
                        </div>
                    </section>
                    <section class="flex gap-2 w-full py-4 border-b border-gray-500">
                        <RvArtboardView
                            name="Compromise"
                            state_machine="Compromise State Machine"
                            attr:class="w-16 h-16"
                            on_loaded={on_rv_loaded}
                        />
                        <div class="grow grid grid-cols-2 gap-2">
                            <h2 class="text-xl col-span-full">
                                {t!("worksheets.download.compromise")}{": "}
                            </h2>
                            <div>
                                <h3 class="text-lg text-center w-full">
                                    {t!("worksheets.compromise.label_solutions")}
                                </h3>
                                <ul class="list-disc pl-6">
                                    {move || solutions().iter().map(|s| view!{
                                        <li class="mb-1">{s}</li>
                                    }).collect_view()}
                                </ul>
                            </div>
                            <div>
                                <h3 class="text-lg text-center w-full">
                                    {t!("worksheets.compromise.label_stakeholders")}
                                </h3>
                                <ul class="list-disc pl-6">
                                    {move || stakeholders().iter().map(|s| view!{
                                        <li class="mb-1">{s}</li>
                                    }).collect_view()}
                                </ul>
                            </div>
                            <div class="col-span-full flex gap-2 items-baseline">
                                <h3 class="text-lg">
                                    {t!("worksheets.download.research")}{": "}
                                </h3>
                                <p>
                                    {question}
                                </p>
                            </div>
                        </div>
                    </section>
                    <section class="flex gap-2 w-full py-4 border-b border-gray-500">
                        <RvArtboardView
                            name="Implement"
                            state_machine="Implement State Machine"
                            attr:class="w-16 h-16"
                            on_loaded={on_rv_loaded}
                        />
                        <div class="grow grid grid-cols-2 gap-2">
                            <h2 class="text-xl col-span-full">
                                {t!("worksheets.implement.title")}{": "}
                            </h2>
                            <div>
                                <h3 class="text-lg text-center w-full">
                                    {t!("worksheets.implement.label_now")}
                                </h3>
                                <h4 class="text-lg font-thin text-center w-full">
                                    {t!("worksheets.implement.hint_now")}
                                </h4>
                                <ul class="list-disc pl-6">
                                    {move || now().iter().map(|s| view!{
                                        <li class="mb-1">{s}</li>
                                    }).collect_view()}
                                </ul>
                            </div>
                            <div>
                                <h3 class="text-lg text-center w-full">
                                    {t!("worksheets.implement.label_best")}
                                </h3>
                                <h4 class="text-lg font-thin text-center w-full">
                                    {t!("worksheets.implement.hint_best")}
                                </h4>
                                <ul class="list-disc pl-6">
                                    {move || best().iter().map(|s| view!{
                                        <li class="mb-1">{s}</li>
                                    }).collect_view()}
                                </ul>
                            </div>
                            <hr class="col-span-full my-2 border-t border-slate-400"/>
                            <div>
                                <h3 class="text-lg text-center w-full">
                                    {t!("worksheets.iterate.label_resources")}
                                </h3>
                                <ul class="list-disc pl-6">
                                    {move || iteration_resources().iter().map(|s| view!{
                                        <li class="mb-1">{s}</li>
                                    }).collect_view()}
                                </ul>
                            </div>
                            <div>
                                <h3 class="text-lg text-center w-full">
                                    {t!("worksheets.iterate.label_externals")}
                                </h3>
                                <ul class="list-disc pl-6">
                                    {move || iteration_external_resources().iter().map(|s| view!{
                                        <li class="mb-1">{s}</li>
                                    }).collect_view()}
                                </ul>
                            </div>
                        </div>
                    </section>
                </main>
            </div>
        </div>
    }
}
