mod examples;
mod types;
mod worksheets;

use form_signal::FormState;

pub use examples::*;
pub use types::*;
pub use worksheets::*;

use leptos::*;
use leptos_router::Outlet;
use leptos_use::{
    storage::{use_local_storage, use_local_storage_with_options, UseStorageOptions},
    utils::JsonCodec,
};

use crate::app::tracking::SessionIdProvider;

use super::{components::WK_STORAGE, tracking::new_tracking_session};

#[derive(Clone)]
struct Store(RwSignal<AppState>);

#[component]
pub fn StoreProvider() -> impl IntoView {
    let session_id = create_resource(
        || (),
        |_| async move {
            match new_tracking_session().await {
                Ok(id) => Some(id),
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
            }
        },
    );

    let (remembered_storage_preference, set_remembered_storage_preference, del_storage_preference) =
        use_local_storage::<Option<StorageMode>, JsonCodec>("storage_preference");

    let (wk_storage, _, del_wk_storage) =
        use_local_storage_with_options::<Option<WorkSheets>, JsonCodec>(
            WK_STORAGE,
            UseStorageOptions::default().listen_to_storage_changes(false),
        );

    let state = create_rw_signal({
        let mut state = AppState::default();

        if let Some(storage_prefernce) = remembered_storage_preference.get_untracked() {
            state.storage_preference = FormState::new(Some(storage_prefernce));
        }

        if let Some(wk) = wk_storage.get_untracked() {
            state.wk = WorkSheetsFormState::from(wk);
            log::info!("write wk state");
        }

        state
    });
    provide_context(Store(state));

    create_effect(move |_| {
        let preference = state.get().storage_preference.get();
        if let Some(StorageMode::Local) = preference {
            set_remembered_storage_preference.update(|o| *o = Some(StorageMode::Local))
        } else {
            del_storage_preference();
            del_wk_storage();
        }
    });

    view! {
        <Transition>
            <SessionIdProvider
                init_id=session_id
            >
                <Outlet/>
            </SessionIdProvider>
        </Transition>
    }
}

pub fn use_store() -> RwSignal<AppState> {
    let ctx = use_context::<Store>().expect("State not provided");
    ctx.0
}
