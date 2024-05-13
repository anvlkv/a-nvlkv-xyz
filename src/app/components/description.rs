use leptos::*;

#[component]
pub fn DescriptionView(
    #[prop(into)] toggle_hidden: Callback<()>,
    #[prop(into)] hidden: Signal<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Show when={move || !hidden.get()}>
            <div class="flex items-start mb-3 rounded-lg max-w-prose p-4 bg-sky-200 dark:bg-sky-950 border border-sky-400 dark:brder-sky-800 text-sky-800 dark:text-sky-200 text-lg">
                <div>
                    {children()}
                </div>
                <button on:click={move |_| toggle_hidden.call(())} title=t!("util.close") class="ml-1 text-xl -mt-0.5">{"×"}</button>
            </div>
        </Show>
    }
}
