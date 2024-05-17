use std::collections::HashMap;

use leptos::*;
use leptos_router::*;
use leptos_use::{
    signal_throttled,
    storage::{use_storage, use_storage_with_options, UseStorageOptions},
    utils::JsonCodec,
};

use crate::app::state::{use_store, State, StorageMode, WorkSheets, WorkSheetsFormState};

#[derive(PartialEq, Clone)]
pub struct Tab {
    pub title: String,
    pub href: String,
}

#[derive(Clone)]
pub struct WorksheetState {
    pub form: Resource<State, WorkSheetsFormState>,
    pub description_hidden: Signal<bool>,
    pub toggle_description_hidden: Callback<()>,
    pub set_tabs: WriteSignal<Vec<Tab>>,
    pub set_current_description: WriteSignal<String>,
    pub set_title: WriteSignal<String>,
}

pub const WK_STORAGE: &str = "worksheet_storage";
pub const WK_DESCRIPTION_HIDDEN_STORAGE: &str = "worksheet_description_hidden";

pub fn use_wk_state() -> Signal<WorkSheetsFormState> {
    let ctx = use_context::<WorksheetState>().unwrap();
    Signal::derive(move || ctx.form.get().unwrap_or_default())
}

#[component]
pub fn WorksheetView(
    #[prop(into)] storage_type: StorageMode,
    children: ChildrenFn,
) -> impl IntoView {
    let (title, set_title) = create_signal(String::default());
    let (current_description, set_current_description) = create_signal(String::default());
    let (tabs, set_tabs) = create_signal::<Vec<Tab>>(vec![]);

    let (_, set_wk_storage, del_wk_storage) =
        use_storage_with_options::<Option<WorkSheets>, JsonCodec>(
            (&storage_type).into(),
            WK_STORAGE,
            UseStorageOptions::default().listen_to_storage_changes(false),
        );

    let (hidden_stored, set_hidden, del_hidden) = use_storage::<HashMap<String, bool>, JsonCodec>(
        (&storage_type).into(),
        WK_DESCRIPTION_HIDDEN_STORAGE,
    );

    let state = use_store();
    let wk_state =
        create_local_resource(move || state.get(), |state| async move { state.wk.clone() });

    let description_hidden = create_local_resource(
        move || hidden_stored.get(),
        |hidden| async move { hidden.clone() },
    );
    let description_hidden = Signal::derive(move || {
        let current = current_description.get();
        *description_hidden
            .get()
            .unwrap_or_default()
            .get(&current)
            .unwrap_or(&false)
    });

    let wk_data_throttled = signal_throttled(
        Signal::derive(move || {
            wk_state
                .try_get()
                .flatten()
                .map(|wk| wk.try_get())
                .flatten()
        }),
        750.0,
    );

    create_effect(move |_| match state.get().storage_preference.get() {
        Some(StorageMode::Local) => {
            if let Some(wk) = wk_data_throttled.get() {
                set_wk_storage.update(|w| *w = Some(wk))
            }
        }
        None => {
            if wk_data_throttled.with(|wk| {
                wk.as_ref()
                    .map(|wk| wk != &WorkSheets::default())
                    .unwrap_or(false)
            }) {
                state.get().show_privacy_prompt.set(true);
            }
        }
        _ => {
            del_wk_storage();
            del_hidden();
        }
    });

    let on_toggle_hidden = Callback::new(move |_| {
        let current = current_description.get();
        if !current.is_empty() {
            set_hidden.update(move |s| {
                let val = !*s.get(&current).unwrap_or(&false);
                s.insert(current, val);
            })
        }
    });

    provide_context(WorksheetState {
        form: wk_state,
        description_hidden,
        toggle_description_hidden: on_toggle_hidden,
        set_tabs,
        set_current_description,
        set_title,
    });

    view! {
        <div class="flex flex-col">
            <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                <h2 class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3">{title}</h2>
                    <div class="flex justify-end grow items-end h-full">
                        <div class="border-b-2 px-2 border-slate-400 grow rounded-t-lg after:content-[' ']">
                            <Show when={move || description_hidden.get()}>
                                <button on:click={move |_| on_toggle_hidden.call(())} title=t!("util.info") class="text-2xl -mb-0.5 text-sky-800 dark:text-sky-200">{"ⓘ"}</button>
                            </Show>
                        </div>
                        <For each=move || tabs.get()
                            key=|state| state.href.clone()
                            let:child>
                            <A href={child.href} exact={true} class="worksheet-tab block rounded-t px-4 pt-3 pb-1 ml-0 mr-px border border-slate-400 border-b-2 hover:text-purple-800 hover:border-purple-800 active:text-purple-950" active_class="pointer-events-none -mb-px border-b-transparent">
                            {child.title}
                            </A>
                        </For>
                    </div>
            </div>
            <div class="grow w-full">
                {children}
            </div>
        </div>
    }
}
