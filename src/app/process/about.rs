use crate::app::state::ProcessStep;
use crate::app::Language;

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
    let lang = use_context::<Signal<Language>>().unwrap();

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
                        <div class="pt-3 pl-3">
                        <h5 class="pb-1 font-bold">
                            {t!(format!("about.s_{i}.title").as_str()).to_string()}
                        </h5>
                        <p>
                            {t!(format!("about.s_{i}.description").as_str()).to_string()}
                        </p>
                    </div>
                </li>
            }
        })
        .collect_view();

    view! {
        <Title text={move || format!("{} | {}", t!("process.title"), t!("name"))}/>
        <div class="grid lg:grid-cols-2 gap-10 content-stretch">
            <div class="max-w-prose flex flex-col self-stretch">
                <p class="pb-4">{t!("about.description_1")}</p>
                <p class="pb-4">{t!("about.description_2")}</p>
                <A attr:type="button" href={move || format!("/{}/2", lang.get().0)} class="mb-3 mt-auto w-full py-3 rounded-full bg-purple-900 hover:bg-purple-800 active:bg-purple-950 text-stone-100 border-2 border-solid border-slate-50 drop-shadow-md text-center">{t!("about.cta")}</A>
            </div>
            <ol class="max-w-prose">
                {steps}
            </ol>
        </div>
    }
}
