use std::collections::HashMap;

use leptos::*;
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
    pub is_example: bool,
}

#[derive(Clone)]
pub struct WorksheetState {
    pub form: Resource<State, WorkSheetsFormState>,
    pub description_hidden: Signal<bool>,
    pub toggle_description_hidden: Callback<()>,
    pub set_current_description: WriteSignal<String>,
}

pub const WK_STORAGE: &str = "worksheet_storage";
pub const WK_DESCRIPTION_HIDDEN_STORAGE: &str = "worksheet_description_hidden";

pub fn use_wk_ctx() -> WorksheetState {
    use_context::<WorksheetState>().unwrap()
}

pub fn use_wk_state() -> Signal<WorkSheetsFormState> {
    let ctx = use_wk_ctx();
    Signal::derive(move || ctx.form.get().unwrap_or_default())
}

#[component]
pub fn WorksheetView(
    #[prop(into)] storage_type: StorageMode,
    children: ChildrenFn,
) -> impl IntoView {
    let (current_description, set_current_description) = create_signal(String::default());

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
                .get()
                // .flatten()
                .map(|wk| wk.get())
        }),
        750.0,
    );

    create_effect(move |_| {
        let wk = wk_data_throttled.get();
        match state.get().storage_preference.get() {
            Some(StorageMode::Local) => {
                if let Some(wk) = wk {
                    set_wk_storage.update(|w| *w = Some(wk))
                }
            }
            None => {
                log::debug!("wk: {wk:#?}");

                if wk.map(|d| d != WorkSheets::default()).unwrap_or(false) {
                    state.get().show_privacy_prompt.set(true);
                }
            }
            _ => {
                del_wk_storage();
                del_hidden();
            }
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
        set_current_description,
    });

    view! {
        <div class="flex flex-col">
            {children}
        </div>
    }
}
