use crate::app::{
    components::{ButtonSize, ButtonView, IconView, RvArtboardView, WorksheetHeader},
    state::{use_store, Completenes, ProcessStep},
    use_lang,
};

use leptos::*;
use leptos_meta::*;

/// step 1
#[component]
pub fn AboutView() -> impl IntoView {
    let lang = use_lang();
    let state = use_store();

    let show_privacy_choice =
        create_read_slice(state, move |s| s.storage_preference.get().is_some());

    let has_data = Signal::derive(move || !state.get().wk.get().is_empty());

    let clear_wk = move |_| state.update(|s| s.wk = Default::default());

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
            <div class="grid lg:grid-cols-2 gap-6 content-stretch">
                <p class="max-w-prose pb-4 col-start-1 whitespace-pre-line">
                    {t!("about.description_1")}
                </p>
                <p class="max-w-prose pb-4 col-start-1 whitespace-pre-line">
                    {t!("about.description_2")}
                </p>
                <ol class="max-w-prose row-span-3 lg:col-start-2 lg:row-start-1">
                    {steps}
                </ol>
                <div class="max-w-prose col-start-1 lg:row-start-3 flex mb-3 mt-auto items-center">
                    <Show when=move || show_privacy_choice.get()>
                        <button
                            on:click={move |_| state.get().show_privacy_prompt.set(true)}
                            title={t!("privacy.short")}
                        >
                            <RvArtboardView
                                attr:class="grow-0 shrink-0 h-14 aspect-square mr-4"
                                state_machine="Privacy State Machine"
                                name="Privacy"
                            />
                        </button>
                    </Show>
                    <ButtonView
                        link={Signal::derive(move || format!("/{}/process/1", lang.get()))}
                        size=ButtonSize::Lg
                        cta=2
                        attr:class="shrink-0 grow"
                    >
                        {t!("about.cta")}
                        <IconView icon="Next"/>
                    </ButtonView>
                </div>
                <Show when=move || has_data.get()>
                    <ButtonView
                        on:click={clear_wk}
                        size=ButtonSize::Lg
                    >
                        <IconView icon="Restart"/>
                        {t!("worksheets.inquire.cta_2")}
                    </ButtonView>
                </Show>
            </div>
        </div>
    }
}
