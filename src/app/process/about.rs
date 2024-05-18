use crate::app::{
    components::WorksheetHeader,
    state::{use_store, ProcessStep},
    use_lang,
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(any(feature = "csr", feature = "hydrate"))]
mod rv_animation {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/app/process/about.mjs")]
    extern "C" {

        #[wasm_bindgen(js_name=mountArtboards)]
        pub fn mount_artboards();

        #[wasm_bindgen(js_name=cleanUp)]
        pub fn clean_up();
    }
}

/// step 1
#[component]
pub fn AboutView() -> impl IntoView {
    let lang = use_lang();
    let state = use_store();

    let show_privacy_choice =
        create_read_slice(state, move |s| s.storage_preference.get().is_some());

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        create_effect(move |_| {
            rv_animation::mount_artboards();
        });

        on_cleanup(move || {
            rv_animation::clean_up();
        });
    }

    let steps = vec![
        ProcessStep::Problem,
        ProcessStep::Solution,
        ProcessStep::Compromise,
        ProcessStep::Implement,
        ProcessStep::Iterate
    ]
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            let i = i + 1;
            view! {
                <li class="pb-3 flex items-start">
                    <canvas id={move || format!("about_icon_{}", name)} class="w-16 h-16 xl:w-20 xl:h-20"/>
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
            <div class="grid lg:grid-cols-2 gap-10 content-stretch">
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
                    <button
                        on:click={move |_| state.get().show_privacy_prompt.set(true)}
                        class={move || if show_privacy_choice.get() { "contents" } else { "hidden" }}
                        title={t!("privacy.short")}
                    >
                        <canvas id="about_icon_Privacy" class="grow-0 shrink-0 h-14 aspect-square mr-4"/>
                    </button>
                    <A attr:type="button" href={move || format!("/{}/process/1", lang.get())} class="shrink-0 grow py-3 rounded-full bg-purple-900 hover:bg-purple-800 active:bg-purple-950 text-stone-100 border-2 border-solid border-slate-50 drop-shadow-md text-center">{t!("about.cta")}</A>
                </div>
            </div>
        </div>
    }
}
