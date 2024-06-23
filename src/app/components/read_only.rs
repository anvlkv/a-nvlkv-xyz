use leptos::*;
use leptos_router::*;

#[component]
pub fn ReadOnlyView(
    #[prop(into, optional)] fallback_href: MaybeSignal<Option<String>>,
    #[prop(into, optional)] fallback_title: MaybeSignal<String>,
    #[prop(into, optional)] label: MaybeSignal<String>,
    #[prop(into, optional)] empty: MaybeSignal<bool>,
    children: ChildrenFn,
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

    let not_empty = move || !empty.get();

    let label = Signal::derive(move || label.get());

    view! {
        <blockquote class="max-w-prose whitespace-pre-line w-full my-2 mx-auto p-4 text-lg rounded border border-slate-300 dark:border-slate-700">
            <Show
                when={not_empty}
                fallback
            >
                <Show when=move || {label.get().len() > 0} >
                    <span class="font-thin">
                        {label}{": "}
                    </span>
                </Show>
                {children()}
            </Show>
        </blockquote>
    }
}

#[component]
pub fn ReadOnlyListView(
    #[prop(into)] value: Signal<Vec<String>>,
    #[prop(into, optional)] label: MaybeSignal<String>,
) -> impl IntoView {
    let empty = Signal::derive(move || value.get().len() == 0);
    view! {
        <div role="readonly" class="flex flex-col gap-4 ">
            <ReadOnlyView label={label} empty={empty}>
                {move || if value.get().len() > 0 {
                    view!{
                        <ul class="list-disc pl-4">
                            <For
                                each=move || value.get()
                                key=|state| state.clone()
                                let:child
                            >
                                <li>
                                    {child}
                                </li>
                            </For>
                        </ul>
                    }.into_view()
                } else {
                    ().into_view()
                }}
            </ReadOnlyView>
        </div>
    }
}
