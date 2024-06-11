use leptos::*;

use crate::app::{components::RvArtboardView, NotFound};

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

#[component]
pub fn ErrorView(#[prop(into, optional)] errors: MaybeSignal<Errors>) -> impl IntoView {
    let errors = Signal::derive(move || errors.get());
    let is_not_found = Signal::derive(move || {
        errors
            .get()
            .iter()
            .any(|e| e.1.to_string().ends_with("Not found"))
    });

    view! {
        <Show when=move || !is_not_found.get() fallback=move || view!{
            <div class="w-full">
                <NotFound/>
            </div>
        }>
            <div class="flex flex-col md:flex-row max-w-prose mx-auto my-6 lg:my-8 p-16 items-start bg-red-200 dark:bg-red-800 rounded-lg">
                <RvArtboardView
                    attr:class="w-32 h-32 mr-8"
                    state_machine="Err State Machine"
                    name="Err"
                />
                <div>
                    <p class="text-xl whitespace-pre-line">{t!("util.err")}</p>
                    <div class="text-sm p-2 mt-4 rounded bg-gray-900/25">
                        <code>
                            {move || errors.get().iter().enumerate().map(|(key, (_, err))| format!(r#"
        {}: {err}
                            "#, key + 1).into_view()).collect_view()}
                        </code>
                    </div>
                </div>
            </div>
        </Show>
    }
}
