use leptos::*;
use leptos_router::*;

#[derive(PartialEq, Clone)]
pub struct Tab {
    pub title: String,
    pub href: String,
}

#[component]
pub fn WorksheetView(
    #[prop(into)] title: String,
    #[prop(into, optional)] tabs: Option<Signal<Vec<Tab>>>,
    children: Children,
) -> impl IntoView {
    let tabs_view = move || {
        tabs.unwrap()
            .get()
            .into_iter()
            .map(|t| {
                view! {
                    <A href={t.href} exact={true} class="worksheet-tab block rounded-t-lg px-4 pt-3 pb-1 ml-0 mr-px border border-slate-400 border-b-2 hover:text-purple-800 hover:border-purple-800 active:text-purple-950" active_class="pointer-events-none -mb-px border-b-transparent">
                        {t.title}
                    </A>
                }
            })
            .collect_view()
    };

    view! {
        <div class="flex flex-col">
            <div class="grow-0 flex items-end flex-wrap w-full mb-3">
                <h2 class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3">{title}</h2>
                <Show when={move || tabs.map(|t| t.get().len() > 0).unwrap_or(false)}>
                    <div class="flex justify-end grow items-end">
                        <div class="border-b-2 border-slate-400 grow rounded-t-lg after:content-[' ']"></div>
                        {tabs_view}
                    </div>
                </Show>
            </div>
            <div class="grow w-full">
                {children()}
            </div>
        </div>
    }
}
