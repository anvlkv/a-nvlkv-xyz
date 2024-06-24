use leptos::*;

use leptos_use::{
    storage::{use_local_storage_with_options, UseStorageOptions},
    utils::JsonCodec,
};

#[cfg(feature = "ssr")]
use spin_sdk::sqlite::{Connection, Value};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use crate::server::safe_error;

use super::state::{use_store, StorageMode};

#[server(NewSession, "/api")]
pub async fn new_tracking_session(ua: Option<String>) -> Result<Uuid, ServerFnError<String>> {
    if let Some(ua) = ua {
        let new_id = Uuid::new_v4();
        let conn = Connection::open("default").map_err(safe_error)?;

        let sql = r#"
            INSERT INTO tracking
            (id, user_agent,
            created_date, updated_date,
            restored_session, inferrence, personal_inquery, wk_download)
            VALUES (?, ?, unixepoch(), unixepoch(), NULL, NULL, NULL, NULL)
    "#;
        _ = conn
            .execute(
                sql,
                &[Value::Blob(Vec::from(new_id.as_bytes())), Value::Text(ua)],
            )
            .map_err(safe_error)?;

        println!("started session {new_id}");

        Ok(new_id)
    } else {
        Err(ServerFnError::MissingArg("No user agent".to_string()))
    }
}

#[server(RestoreSession, "/api")]
pub async fn restore_tracking_session(
    init_id: Uuid,
    restore_id: Uuid,
) -> Result<(), ServerFnError<String>> {
    let conn = Connection::open("default").map_err(safe_error)?;

    let sql = r#"
        UPDATE tracking
        SET restored_session = ?, updated_date = unixepoch()
        WHERE id = ?;
"#;
    _ = conn
        .execute(
            sql,
            &[
                Value::Blob(Vec::from(restore_id.as_bytes())),
                Value::Blob(Vec::from(init_id.as_bytes())),
            ],
        )
        .map_err(safe_error)?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub fn complete_inferrence(id: Uuid, result: String) -> Result<(), ServerFnError<String>> {
    let conn = Connection::open("default").map_err(safe_error)?;

    let sql = r#"
        UPDATE tracking
        SET inferrence = ?, updated_date = unixepoch()
        WHERE id = ?;
"#;
    _ = conn
        .execute(
            sql,
            &[Value::Text(result), Value::Blob(Vec::from(id.as_bytes()))],
        )
        .map_err(safe_error)?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub fn complete_personal(id: Uuid, data_id: String) -> Result<(), ServerFnError<String>> {
    let conn = Connection::open("default").map_err(safe_error)?;

    let sql = r#"
        UPDATE tracking
        SET personal_inquery = ?, updated_date = unixepoch()
        WHERE id = ?;
"#;
    _ = conn
        .execute(
            sql,
            &[Value::Text(data_id), Value::Blob(Vec::from(id.as_bytes()))],
        )
        .map_err(safe_error)?;

    Ok(())
}

#[server(WkDownloadSession, "/api")]
pub async fn complete_wk_download(id: Uuid) -> Result<(), ServerFnError<String>> {
    let conn = Connection::open("default").map_err(safe_error)?;

    let sql = r#"
        UPDATE tracking
        SET wk_download = 1, updated_date = unixepoch()
        WHERE id = ?;
"#;
    _ = conn
        .execute(sql, &[Value::Blob(Vec::from(id.as_bytes()))])
        .map_err(safe_error)?;

    Ok(())
}

#[derive(Clone, PartialEq, Eq)]
pub struct SessionId(pub ReadSignal<Option<Uuid>>);

pub fn session_id_resource() -> Resource<Option<String>, Option<Uuid>> {
    #[cfg(feature = "ssr")]
    let req = use_context::<leptos_spin::RequestParts>();

    create_resource(
        move || {
            cfg_if::cfg_if! {
                if #[cfg(feature = "ssr")] {
                    req.as_ref().map(|r| {
                        r.headers()
                            .iter()
                            .find_map(|(name, ua)| {
                                if &http::header::USER_AGENT.to_string() == name {
                                    std::str::from_utf8(ua.as_slice()).ok().map(|d| d.to_string())
                                } else {
                                    None
                                }
                            })
                    }).flatten()
                }
                else {
                    Option::<String>::None
                }
            }
        },
        |ua: Option<String>| async move {
            match new_tracking_session(ua).await {
                Ok(id) => Some(id),
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
            }
        },
    )
}

#[component]
pub fn SessionIdProvider(
    #[prop(into)] init_id: Resource<Option<String>, Option<Uuid>>,
    children: ChildrenFn,
) -> impl IntoView {
    let id_rw = create_rw_signal::<Option<Uuid>>(None);
    let (remembered_session_id, set_remembered_session_id, del_session_id) =
        use_local_storage_with_options::<Option<Uuid>, JsonCodec>(
            "session_id",
            UseStorageOptions::default().listen_to_storage_changes(false),
        );
    let store = use_store();

    let restore_session_id = create_action(|ids: &(Uuid, Uuid)| {
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
