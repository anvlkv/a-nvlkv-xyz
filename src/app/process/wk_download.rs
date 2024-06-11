use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use web_time::Duration;

use crate::app::{
    components::{use_wk_state, Localized, RvArtboardView, WorksheetView},
    state::{use_store, StorageMode, WorkSheets},
};

#[component]
pub fn WorksheetsDownload() -> impl IntoView {
    let store = use_store();
    let storage_type = create_read_slice(store, |s| {
        s.storage_preference.get().unwrap_or(StorageMode::None)
    });

    view! {
        <Link
            rel="stylesheet"
            href="https://cdnjs.cloudflare.com/ajax/libs/paper-css/0.4.1/paper.css"
            integrity="sha384-Velkkr4y29T3b+5t49UmQaVHkJrr1GJRHHq1BG3nSpmQrdf5Dv525IDQRdqkxZpd"
            crossorigin="anonymous"
        />
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
    let now = move || data.get().implement.now;
    let best = move || data.get().implement.best;

    let (rv_loaded, set_rv_loaded) =
        create_signal(HashSet::<&'static str>::from_iter(vec!["Inquire"]));

    create_effect(move |_| {
        let body = document().get_elements_by_tag_name("body").item(0).unwrap();
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
                Duration::from_millis(100),
            )
        }
    });

    view! {
        <Title text={move || format!("{}_{}_{}", iteration_title(), t!("about.title"), t!("name")).replace(&['.', '|', '/', '\\', '>', '<', '!', '?', '*'], "-")}/>
        <div
            class="font-sans bg-white text-black"
        >
            <section class="sheet padding-10mm">
                <header class="flex w-full justify-start items-center border-b border-black">
                    <RvArtboardView
                        name="Inquire"
                        state_machine="Inquire State Machine"
                        attr:class="w-16 h-16"
                        on_loaded={move |_| set_rv_loaded.update(|v|{ v.remove("Inquire");})}
                    />
                    <h1 class="text-2xl">
                        {t!("about.title")}
                    </h1>
                    <div class="h-full text-sm pl-4 ml-auto mr-0 border-l border-black">
                        <p>{t!("worksheets.download.signature")}</p>
                        <a href="https://a.nvlkv.xyz" class="underline">{"https://a.nvlkv.xyz"}</a>
                    </div>
                </header>

                <h2>
                    {t!("worksheets.problem.label_statement")}
                </h2>
                <p>
                    {problem_statement}
                </p>
            </section>
            <section class="sheet padding-10mm">
                <div class="grid grid-cols-2">
                    <div>
                        <h3>
                            {t!("worksheets.compromise.label_solutions")}
                        </h3>
                        <ul>
                            {move || solutions().iter().map(|s| view!{<li>{s}</li>}).collect_view()}
                        </ul>
                    </div>
                    <div>
                        <h3>
                            {t!("worksheets.compromise.label_stakeholders")}
                        </h3>
                        <ul>
                            {move || stakeholders().iter().map(|s| view!{<li>{s}</li>}).collect_view()}
                        </ul>
                    </div>
                </div>
                <h2>
                    {t!("worksheets.compromise.label_question")}
                </h2>
                <p>
                    {question}
                </p>
                <h2>
                    {t!("worksheets.implement.title")}
                </h2>
                <div class="grid grid-cols-2">
                    <div>
                        <h3>
                            {t!("worksheets.implement.label_now")}
                        </h3>
                        <ul>
                            {move || now().iter().map(|s| view!{<li>{s}</li>}).collect_view()}
                        </ul>
                    </div>
                    <div>
                        <h3>
                            {t!("worksheets.implement.label_best")}
                        </h3>
                        <ul>
                            {move || best().iter().map(|s| view!{<li>{s}</li>}).collect_view()}
                        </ul>
                    </div>
                </div>
            </section>
        </div>
    }
}
