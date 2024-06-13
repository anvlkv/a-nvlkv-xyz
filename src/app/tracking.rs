use leptos::*;

use leptos_use::{
    storage::{use_local_storage_with_options, UseStorageOptions},
    utils::JsonCodec,
};

#[cfg(feature = "ssr")]
use spin_sdk::pg::{Decode, ParameterValue};

#[cfg(feature = "ssr")]
use crate::server::{get_db_conn, safe_error};

use super::state::{use_store, StorageMode};

#[server(NewSession, "/api")]
pub async fn new_tracking_session() -> Result<String, ServerFnError<String>> {
    let conn = get_db_conn().map_err(safe_error)?;

    let sql = r#"
        INSERT INTO "tracking" (xata_id) VALUES (DEFAULT)
            RETURNING xata_id;
"#;
    let data = conn.query(sql, &[]).map_err(safe_error)?;

    let id = String::decode(&data.rows[0][0]).map_err(safe_error)?;

    println!("started session {id}");

    Ok(id)
}

#[server(RestoreSession, "/api")]
pub async fn restore_tracking_session(
    init_id: String,
    restore_id: String,
) -> Result<(), ServerFnError<String>> {
    let conn = get_db_conn().map_err(safe_error)?;

    let sql = r#"
        UPDATE "tracking"
        SET restored_session = $2
        WHERE xata_id = $1;
"#;
    _ = conn
        .execute(
            sql,
            &[
                ParameterValue::Str(init_id),
                ParameterValue::Str(restore_id),
            ],
        )
        .map_err(safe_error)?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub fn complete_inferrence(id: String) -> Result<(), ServerFnError<String>> {
    let conn = get_db_conn().map_err(safe_error)?;

    let sql = r#"
        UPDATE "tracking"
        SET inferrence = true
        WHERE xata_id = $1;
"#;
    _ = conn
        .execute(sql, &[ParameterValue::Str(id)])
        .map_err(safe_error)?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub fn complete_personal(id: String, data_id: String) -> Result<(), ServerFnError<String>> {
    let conn = get_db_conn().map_err(safe_error)?;

    let sql = r#"
        UPDATE "tracking"
        SET personal_inquery = $2
        WHERE xata_id = $1;
"#;
    _ = conn
        .execute(
            sql,
            &[ParameterValue::Str(id), ParameterValue::Str(data_id)],
        )
        .map_err(safe_error)?;

    Ok(())
}

#[server(WkDownloadSession, "/api")]
pub async fn complete_wk_download(id: String) -> Result<(), ServerFnError<String>> {
    let conn = get_db_conn().map_err(safe_error)?;

    let sql = r#"
        UPDATE "tracking"
        SET wk_download = true
        WHERE xata_id = $1;
"#;
    _ = conn
        .execute(sql, &[ParameterValue::Str(id)])
        .map_err(safe_error)?;

    Ok(())
}

#[derive(Clone, PartialEq, Eq)]
pub struct SessionId(pub ReadSignal<Option<String>>);

#[component]
pub fn SessionIdProvider(
    #[prop(into)] init_id: Resource<(), Option<String>>,
    children: ChildrenFn,
) -> impl IntoView {
    let id_rw = create_rw_signal::<Option<String>>(None);
    let (remembered_session_id, set_remembered_session_id, del_session_id) =
        use_local_storage_with_options::<Option<String>, JsonCodec>(
            "session_id",
            UseStorageOptions::default().listen_to_storage_changes(false),
        );
    let store = use_store();

    let restore_session_id = create_action(|ids: &(String, String)| {
        let (init_id, restore_id) = ids.clone();
        async move { restore_tracking_session(init_id, restore_id).await }
    });

    let storage_preference = Signal::derive(move || store.get().storage_preference.get());

    create_isomorphic_effect(move |_| {
        let init_id = init_id.get().flatten();
        // only happens on client if it has session id in LS
        if let Some(old_id) = remembered_session_id.get() {
            id_rw.set(Some(old_id.clone()));
            // only happens if there's an id for current session
            if let Some(init_id) = init_id {
                restore_session_id.dispatch((init_id, old_id));
            }
        } else if let Some(id) = init_id {
            id_rw.set(Some(id.clone()));
            // only happens on client if LS is allowed
            if Some(StorageMode::Local) == storage_preference.get() {
                set_remembered_session_id.set(Some(id));
            } else {
                del_session_id()
            }
        }
    });

    let restore_success = restore_session_id.value();
    create_render_effect(move |_| {
        // fallback to the new id if restore failed
        if let Some(Err(_)) = restore_success.get() {
            if let Some(id) = init_id.get().flatten() {
                id_rw.set(Some(id.clone()));
                if Some(StorageMode::Local) == storage_preference.get() {
                    set_remembered_session_id.set(Some(id));
                }
            }
        }
    });

    provide_context(SessionId(id_rw.read_only()));

    children.into_view()
}
