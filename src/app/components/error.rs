use leptos::*;

#[component]
pub fn ErrorTemplate(errors: RwSignal<Errors>) -> impl IntoView {
    view! {
        <For each=move || errors.get()
            key=|state| state.0.clone()
            let:child
        >
            <p>{format!("{:?}", child.1)}</p>
        </For>
    }
}

#[cfg(any(feature = "csr", feature = "hydrate"))]
mod rv_animation {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/app/components/error.mjs")]
    extern "C" {

        #[wasm_bindgen(js_name=errAnimation)]
        pub fn err_animation();

        #[wasm_bindgen(js_name=cleanUp)]
        pub fn clean_up();
    }
}

#[component]
pub fn ErrorView() -> impl IntoView {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        create_effect(move |_| {
            rv_animation::err_animation();
        });

        on_cleanup(move || {
            rv_animation::clean_up();
        });
    }

    view! {
        <div class="flex flex-col md:flex-row max-w-prose mx-auto my-6 lg:my-8 p-16 items-center bg-red-200 dark:bg-red-800 rounded-lg">
            <canvas id="err_animation" class="w-32 h-32 mr-8"/>
            <p class="text-xl whitespace-pre-line">{t!("util.err")}</p>
        </div>
    }
}
