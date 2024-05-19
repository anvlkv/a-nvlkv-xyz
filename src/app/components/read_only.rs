use leptos::*;
use leptos_router::*;

#[component]
pub fn ReadOnlyView(
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(into, optional)] fallback_href: MaybeSignal<Option<String>>,
    #[prop(into, optional)] fallback_title: MaybeSignal<String>,
) -> impl IntoView {
    let fallback = move || {
        let href = fallback_href.get();
        let title = fallback_title.get();

        view! {
            <p class="text-sm opacity-80">
                {t!("util.empty")}
                {
                    href.map(|href| view!{
                        {" "}
                        <A href
                            class="underline text-purple-800 dark:text-purple-200"
                        >
                            {title}
                        </A>
                    })
                }

            </p>
        }
    };

    let value = Signal::derive(move || value.get());

    view! {
        <div class="max-w-prose my-2 mx-auto p-4 text-lg rounded border border-slate-300 dark:border-slate-700">
            <Show
                when={move || !value.get().is_empty()}
                fallback
            >
                <p role="readonly">{value}</p>
            </Show>
        </div>
    }
}

#[component]
pub fn ReadOnlyListView(#[prop(into)] value: MaybeSignal<Vec<String>>) -> impl IntoView {
    let entries = Signal::derive(move || value.get());

    view! {
        <div role="readonly" class="flex flex-col gap-4 ">
            <For
                each=move || entries.get()
                key=|state| state.clone()
                let:child
            >
                <ReadOnlyView value=child.clone()/>
            </For>
        </div>
    }
}
