use std::{fmt::Display, str};

use leptos_spin::{render_best_match_to_stream, server_fn::register_explicit, RouteTable};
use spin_sdk::{
    http::{Fields, IncomingRequest, OutgoingResponse, ResponseOutparam},
    http_component,
    pg::Connection,
    variables,
};

const TEMPORARY_REDIRECT_CODE: u16 = 307;

#[http_component]
async fn handle_a_nvlkv_xyz(req: IncomingRequest, resp_out: ResponseOutparam) {
    let url = req.path_with_query().unwrap();
    println!("handling request: {:?} {}", req.method(), req.uri());

    let mut conf = leptos::get_configuration(None).await.unwrap();
    conf.leptos_options.output_name = "a_nvlkv_xyz".to_owned();

    // Register server functions
    register_explicit::<crate::app::projects::GetProjects>();
    register_explicit::<crate::app::projects::GetProjectDetails>();
    register_explicit::<crate::app::process::InquireInferrence>();
    register_explicit::<crate::app::process::InquirePersonal>();
    register_explicit::<crate::app::process::InquireContact>();
    register_explicit::<crate::app::resume::GetCvEntries>();
    register_explicit::<crate::app::pages::GetLinks>();
    register_explicit::<crate::app::tracking::NewSession>();
    register_explicit::<crate::app::tracking::RestoreSession>();
    register_explicit::<crate::app::tracking::WkDownloadSession>();

    let app = crate::app::App;

    let mut routes = RouteTable::build(app);
    routes.add_server_fn_prefix("/api").unwrap();

    // handle localization
    match (lang_code_or_redirect(&req), url.starts_with("/api")) {
        (Ok(_), _) | (_, true) => {
            // render localized
            render_best_match_to_stream(req, resp_out, &routes, app, &conf.leptos_options).await;
        }
        (Err(redirect), _) => {
            // redirect to language
            println!("redirecting: {url} to {redirect}");

            let res = OutgoingResponse::new(
                TEMPORARY_REDIRECT_CODE,
                &Fields::new(&[(http::header::LOCATION.to_string(), redirect.into_bytes())]),
            );

            resp_out.set(res);
        }
    }
}

fn lang_code_or_redirect(req: &IncomingRequest) -> Result<String, String> {
    let url = req.path_with_query().unwrap();

    let supported_languages = rust_i18n::available_locales!();

    let it = url.split("/").skip(1);

    let mut ln_it = it.clone();
    let selected_language = ln_it.next().and_then(|lc| {
        if supported_languages.contains(&lc) {
            Some(lc)
        } else {
            None
        }
    });

    if let Some(lang) = selected_language {
        Ok(lang.to_string())
    } else {
        let lang_h = req
            .headers()
            .get(&http::header::ACCEPT_LANGUAGE.to_string());

        let lang_h = lang_h
            .first()
            .map(|ln| str::from_utf8(ln).ok())
            .flatten()
            .unwrap_or("");

        let suggested_language = {
            let langs = accept_language::intersection(lang_h, supported_languages.as_slice());
            langs
                .first()
                .map(|s| s.as_str())
                .unwrap_or(supported_languages.first().unwrap())
                .to_string()
        };

        let route = [suggested_language.as_str()]
            .into_iter()
            .chain(it)
            .fold(String::default(), |acc, p| {
                format!("{acc}{}{p}", if p.is_empty() { "" } else { "/" })
            });

        Err(route)
    }
}

pub fn get_db_conn() -> anyhow::Result<Connection> {
    let xata_pg_url = variables::get("xata_pg_url")?;
    let conn = Connection::open(xata_pg_url.as_str())?;
    Ok(conn)
}

pub fn xata_rest_builder(path: &str) -> anyhow::Result<spin_sdk::http::RequestBuilder> {
    let xata_key = variables::get("xata_key")?;
    let xata_rest_url = variables::get("xata_rest_url")?;
    let db_name = variables::get("db_name")?;
    let db_branch = variables::get("db_branch")?;
    let mut req = spin_sdk::http::Request::builder();

    req.header("Authorization", format!("Bearer {xata_key}"))
        .header("Content-Type", "application/json")
        .uri(format!("{xata_rest_url}/db/{db_name}:{db_branch}/{path}"));

    Ok(req)
}

pub fn safe_error<T: Display>(err: T) -> String {
    eprintln!("{}", err.to_string());
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            err.to_string()
        } else {
            format!("Server error")
        }
    }
}
