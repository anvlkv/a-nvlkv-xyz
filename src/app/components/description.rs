use leptos::*;

#[component]
pub fn DescriptionView(
    #[prop(into)] toggle_hidden: Callback<()>,
    #[prop(into)] hidden: Signal<bool>,
    #[prop(into, optional)] alternative: MaybeSignal<bool>,

    children: ChildrenFn,
) -> impl IntoView {
    let class = move || {
        format!(
            "flex items-start mb-3 rounded-lg max-w-prose lg:w-fit p-4 {} text-lg",
            if alternative.get() {
                "bg-emerald-200 dark:bg-emerald-950 border border-emerald-400 dark:brder-emerald-800 text-emerald-800 dark:text-emerald-200"
            } else {
                "bg-sky-200 dark:bg-sky-950 border border-sky-400 dark:brder-sky-800 text-sky-800 dark:text-sky-200"
            }
        )
    };

    view! {
        <Show when={move || !hidden.get()}>
            <div class=class>
                <div>
                    {children()}
                </div>
                <button on:click={move |_| toggle_hidden.call(())} title=t!("util.close") class="ml-1 mr-0 text-xl -mt-0.5">{"×"}</button>
            </div>
        </Show>
    }
}
