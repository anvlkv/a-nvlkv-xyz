use std::rc::Rc;

use leptos::*;
use leptos_router::*;
use leptos_use::{storage::use_local_storage, utils::JsonCodec};

use crate::app::components::DescriptionView;

#[derive(PartialEq, Clone)]
pub struct Tab {
    pub title: String,
    pub href: String,
}

#[component]
pub fn WorksheetView<F, IV>(
    #[prop(into)] title: String,
    #[prop(into, optional)] tabs: Option<Signal<Vec<Tab>>>,
    #[prop(into)] description: Rc<F>,
    #[prop(into)] description_id: String,
    children: ChildrenFn,
) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let tabs_view = move || {
        tabs.unwrap()
            .get()
            .into_iter()
            .map(|t| {
                view! {
                    <A href={t.href} exact={true} class="worksheet-tab block rounded-t px-4 pt-3 pb-1 ml-0 mr-px border border-slate-400 border-b-2 hover:text-purple-800 hover:border-purple-800 active:text-purple-950" active_class="pointer-events-none -mb-px border-b-transparent">
                        {t.title}
                    </A>
                }
            })
            .collect_view()
    };

    let (hidden_stored, set_hidden, _) = use_local_storage::<Option<bool>, JsonCodec>(format!(
        "description_{description_id}_hidden"
    ));

    let description_hidden = create_local_resource(
        move || hidden_stored.get(),
        |hidden| async move { hidden.unwrap_or(false) },
    );

    let toggle_hidden = Callback::new(move |_| {
        let next = description_hidden.get().map(|h| !h).unwrap_or(false);
        set_hidden.set(Some(next));
    });

    view! {
        <div class="flex flex-col">
            <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                <h2 class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3">{title}</h2>
                    <div class="flex justify-end grow items-end h-full">
                        <div class="border-b-2 px-2 border-slate-400 grow rounded-t-lg after:content-[' ']">
                            <Show when={move || description_hidden.get().unwrap_or(false)}>
                                <button on:click={move |_| toggle_hidden.call(())} title=t!("util.info") class="text-2xl text-sky-800 dark:text-sky-200">{"ⓘ"}</button>
                            </Show>
                        </div>
                        <Show when={move || tabs.map(|t| t.get().len() > 0).unwrap_or(false)}>
                            {tabs_view}
                        </Show>
                    </div>
            </div>
            <div class="grow w-full">
                <DescriptionView hidden={Signal::derive(move || description_hidden.get())} toggle_hidden={toggle_hidden}>
                    {description()}
                </DescriptionView>
                {children}
            </div>
        </div>
    }
}

#[component]
pub fn WorksheetDummy() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                <div class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3">
                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-5 md:h-7 after:content-[' ']"></div>
                </div>
                    <div class="flex justify-end grow items-end h-full">
                        <div class="border-b-2 px-2 border-slate-400 grow after:content-[' ']">
                        </div>
                        <div class="worksheet-tab block rounded-t px-4 pt-3 pb-1 ml-0 mr-px border border-slate-400 border-b-2 -mb-px border-b-transparent">
                            <div class="dummy-line rounded-sm w-24 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                        </div>
                        <div class="worksheet-tab block rounded-t px-4 pt-3 pb-1 ml-0 mr-px border border-slate-400 border-b-2">
                            <div class="dummy-line rounded-sm w-24 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                        </div>
                        <div class="worksheet-tab block rounded-t px-4 pt-3 pb-1 ml-0 mr-px border border-slate-400 border-b-2">
                            <div class="dummy-line rounded-sm w-24 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                        </div>
                    </div>
            </div>
            <div class="grow w-full">
                <div class="dummy-line rounded-sm w-96 max-w-full mb-2 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                <div class="dummy-line rounded-sm w-96 max-w-full mb-2 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                <div class="dummy-line rounded-sm w-96 max-w-full mb-2 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                <div class="dummy-line rounded-sm w-64 max-w-full mb-2 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
            </div>
        </div>
    }
}
