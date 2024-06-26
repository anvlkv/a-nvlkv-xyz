use form_signal::FormSignal;
use leptos::*;
use leptos_meta::*;
use web_time::Instant;

use crate::app::{
    components::{
        use_wk_ctx, ButtonSize, ButtonView, HistoryEntry, IconView, RvArtboardView, UndoRemove,
        WorksheetHeader,
    },
    state::{use_store, Completenes, ProcessStep, WorkSheets},
    use_lang,
};

/// step 1
#[component]
pub fn AboutView() -> impl IntoView {
    let lang = use_lang();
    let state = use_store();

    let show_privacy_choice =
        Signal::derive(move || state.get().storage_preference.get().is_some());
    let wk_state = use_wk_ctx();
    let wk = FormSignal::new(wk_state.wk_data, |s| s, |s, w| *s = w);

    let has_data = Signal::derive({
        let wk = wk.clone();
        move || !wk.get().is_empty()
    });

    let wk_clear_history = create_rw_signal(vec![]);

    let clear_wk = {
        let wk = wk.clone();
        move |_| {
            wk_clear_history.set(vec![(wk.get(), 0, Instant::now())]);
            wk.clear();
        }
    };

    let wk_restore = {
        let wk = wk.clone();
        move |(wk_data, _, _): HistoryEntry<WorkSheets>| {
            wk.set(wk_data);
        }
    };

    let steps = vec![
        ProcessStep::Problem,
        ProcessStep::Solution,
        ProcessStep::Compromise,
        ProcessStep::Implement,
        ProcessStep::Iterate,
    ]
    .into_iter()
    .enumerate()
    .map(|(i, name)| {
        let i = i + 1;
        view! {
            <li class="pb-3 flex items-start">
                <RvArtboardView
                    attr:class="w-16 h-16 xl:w-20 xl:h-20"
                    state_machine={format!("{name} State Machine")}
                    name={format!("{name}")}
                />
                <div class="pl-3">
                    <h5 class="pb-1 font-bold">
                        {t!(format!("about.s_{i}.title").as_str()).to_string()}
                    </h5>
                    <p class="whitespace-pre-line">
                        {t!(format!("about.s_{i}.description").as_str()).to_string()}
                    </p>
                </div>
            </li>
        }
    })
    .collect_view();

    view! {
        <Title text={move || format!("{} | {}", t!("process.title"), t!("name"))}/>

        <WorksheetHeader
            title={t!("about.title").to_string()}
        />
        <div class="grow w-full">
            <div class="grid lg:grid-cols-2 gap-6 content-stretch pb-3">
                <p class="max-w-prose pb-4 col-start-1 whitespace-pre-line">
                    {t!("about.description_1")}
                </p>
                <p class="max-w-prose pb-4 col-start-1 whitespace-pre-line">
                    {t!("about.description_2")}
                </p>
                <ol class="max-w-prose row-span-3 lg:col-start-2 lg:row-start-1">
                    {steps}
                </ol>
                <Show when=move || show_privacy_choice.get()>
                    <ButtonView
                        on:click={move |_| state.get().show_privacy_prompt.set(true)}
                        attr:class="lg:col-start-2"
                    >
                        <RvArtboardView
                            attr:class="grow-0 shrink-0 h-8 aspect-square inline-block"
                            state_machine="Privacy State Machine"
                            name="Privacy"
                        />
                        {t!("privacy.short")}
                    </ButtonView>
                </Show>
                <Show when=move || has_data.get()>
                    <ButtonView
                        on:click={clear_wk.clone()}
                        cta=-1
                        attr:class="lg:col-start-2"
                    >
                        <IconView icon="Restart"/>
                        {t!("worksheets.inquire.cta_2")}
                    </ButtonView>
                </Show>
                <div class="max-w-prose col-start-1 lg:row-start-3 flex mt-auto items-center">
                    <ButtonView
                        link={Signal::derive(move || format!("/{}/process/1", lang.get()))}
                        size=ButtonSize::Lg
                        cta=2
                        attr:class="shrink-0 grow"
                    >
                        {move || if has_data.get() {
                            t!("about.cta_alt").to_string()
                        } else {
                            t!("about.cta").to_string()
                        }}
                        <IconView icon="Next"/>
                    </ButtonView>
                </div>
            </div>
        </div>
        <UndoRemove
            history=wk_clear_history
            on_restore=wk_restore
        />
    }
}
