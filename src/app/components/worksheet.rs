use std::{collections::VecDeque, rc::Rc};

use leptos::*;
use leptos_router::*;
use leptos_use::{
    signal_throttled,
    storage::{use_storage, use_storage_with_options, UseStorageOptions},
    use_event_listener,
    utils::JsonCodec,
};

use crate::app::{
    components::DescriptionView,
    state::{use_store, StorageMode, WorkSheets},
};

#[derive(PartialEq, Clone)]
pub struct Tab {
    pub title: String,
    pub href: String,
}

pub const WK_STORAGE: &str = "worksheet_storage";

#[component]
pub fn WorksheetView<F, IV>(
    #[prop(into)] title: String,
    #[prop(into, optional)] tabs: Option<Signal<Vec<Tab>>>,
    #[prop(into)] description: Rc<F>,
    #[prop(into)] description_id: String,
    storage_type: StorageMode,
    children: ChildrenFn,
) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let (_, set_wk_storage, del_wk_storage) =
        use_storage_with_options::<Option<WorkSheets>, JsonCodec>(
            (&storage_type).into(),
            WK_STORAGE,
            UseStorageOptions::default().listen_to_storage_changes(false),
        );
    let (hidden_stored, set_hidden, del_hidden) = use_storage::<Option<bool>, JsonCodec>(
        (&storage_type).into(),
        format!("description_{description_id}_hidden"),
    );

    let state = use_store();
    let worksheet = signal_throttled(Signal::derive(move || state.get().wk.get()), 750.0);

    create_effect(move |_| match state.get().storage_preference.get() {
        Some(StorageMode::Local) => {
            log::debug!("update wk");
            let wk = worksheet.get();
            set_wk_storage.update(|w| *w = Some(wk))
        }
        None => {
            if worksheet.with(|wk| *wk != WorkSheets::default()) {
                state.get().show_privacy_prompt.set(true);
            }
        }
        _ => {
            del_wk_storage();
            del_hidden();
        }
    });

    let description_hidden = create_local_resource(
        move || hidden_stored.get(),
        |hidden| async move { hidden.unwrap_or(false) },
    );

    let toggle_hidden = Callback::new(move |_| {
        let next = description_hidden.get().map(|h| !h).unwrap_or(false);
        set_hidden.set(Some(next));
    });

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

    view! {
        <div class="flex flex-col">
            <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                <h2 class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3">{title}</h2>
                    <div class="flex justify-end grow items-end h-full">
                        <div class="border-b-2 px-2 border-slate-400 grow rounded-t-lg after:content-[' ']">
                            <Show when={move || description_hidden.get().unwrap_or(false)}>
                                <button on:click={move |_| toggle_hidden.call(())} title=t!("util.info") class="text-2xl -mb-0.5 text-sky-800 dark:text-sky-200">{"ⓘ"}</button>
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
