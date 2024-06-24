use std::collections::HashMap;

use leptos::*;
use leptos_use::{
    signal_throttled,
    storage::{use_storage, use_storage_with_options, UseStorageOptions},
    utils::JsonCodec,
};

use crate::app::state::{use_store, AppState, StorageMode, WorkSheets, WorkSheetsFormState};

#[derive(PartialEq, Clone)]
pub struct Tab {
    pub title: String,
    pub href: String,
    pub is_example: bool,
}

#[derive(Clone)]
pub struct WorksheetState {
    pub form: Resource<AppState, WorkSheetsFormState>,
    pub description_hidden: Signal<bool>,
    pub toggle_description_hidden: Callback<()>,
    pub set_current_description: WriteSignal<String>,
    pub toggle_fullscreen: Callback<()>,
    pub is_fullscreen: Signal<bool>,
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

#[cfg_attr(feature = "ssr", allow(unused))]
#[component]
pub fn WorksheetView(
    #[prop(into)] storage_type: StorageMode,
    #[prop(into, optional)] fs_element: Option<NodeRef<html::Div>>,
    children: ChildrenFn,
) -> impl IntoView {
    let (current_description, set_current_description) = create_signal(String::default());

    let (wk_storage, set_wk_storage, del_wk_storage) =
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
        Signal::derive(move || wk_state.get().map(|wk| wk.get())),
        750.0,
    );

    // create_render_effect(move |_| {
    //     let data = wk_storage.get_untracked();
    //     if let Some(wk) = data {
    //         state.update(|s| {
    //             s.wk = WorkSheetsFormState::new(wk);
    //         });
    //     }
    // });

    create_effect(move |_| {
        let wk = wk_data_throttled.get();
        if let Some(pref) = state.try_get().map(|s| s.storage_preference.try_get()) {
            match pref.flatten() {
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

    #[cfg_attr(feature = "ssr", allow(unused))]
    let (is_fullscreen, set_is_fullscreen) = create_signal(false);
    let on_tooggle_fullscreen = Callback::new(move |_| {
        if let Some(fs_element) = fs_element {
            #[cfg(feature = "client")]
            {
                let doc = document();
                if let Some(el) = doc.fullscreen_element() {
                    el.class_list().remove_1("full-screen").unwrap();
                    doc.exit_fullscreen();
                    set_is_fullscreen.set(false);
                } else if let Some(el) = fs_element.get().as_deref() {
                    match el.request_fullscreen() {
                        Ok(_) => {
                            el.class_list().add_1("full-screen").unwrap();
                            set_is_fullscreen.set(true);
                        }
                        Err(e) => {
                            log::error!("fullscreen error: {:?}", js_sys::Error::from(e));
                            set_is_fullscreen.set(false);
                        }
                    }
                } else {
                    log::warn!("not fullscreen; fullscreen element not provided");
                }
            }
        }
    });

    provide_context(WorksheetState {
        form: wk_state,
        description_hidden,
        toggle_description_hidden: on_toggle_hidden,
        toggle_fullscreen: on_tooggle_fullscreen,
        set_current_description,
        is_fullscreen: is_fullscreen.into(),
    });

    view! {
        <div class="flex flex-col">
            {children}
        </div>
    }
}
