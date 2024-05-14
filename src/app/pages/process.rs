use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use strum::VariantArray;

use crate::app::{
    components::{PrivacyNoticeView, WorksheetDummy, WorksheetView},
    process::*,
    state::{use_store, Example, ProcessStep, SeqStep, StorageMode},
    use_lang, Language,
};

#[component]
pub fn ProcessView() -> impl IntoView {
    let store = use_store();
    let lang = use_lang();

    let storage_type = create_read_slice(store, |s| {
        s.storage_preference.get().unwrap_or(StorageMode::None)
    });

    let examples = create_resource(
        move || lang.get(),
        |lang| async move {
            println!("get with {lang}");
            log::info!("getting examples");
            get_examples(lang, 3, 0).await
        },
    );

    create_isomorphic_effect(move |old| {
        let state = store.get();
        if state.sequence.is_empty() || old.map(|o| o != state.lang).unwrap_or(false) {
            store.update(|s| {
                s.sequence = ProcessStep::VARIANTS
                    .iter()
                    .enumerate()
                    .map(|(i, step)| SeqStep {
                        href: format!("/{}/process/{}", state.lang, i),
                        process_step: *step,
                    })
                    .collect();
            });
        }
        state.lang
    });

    create_isomorphic_effect(move |_| {
        if let Some(data) = examples.get() {
            match data {
                Ok(examples) => {
                    let lang = store.get_untracked().lang;

                    store.update(|s| {
                        s.sequence = ProcessStep::VARIANTS.iter().enumerate().fold(
                            vec![],
                            |mut acc, (i, step)| {
                                acc.push(SeqStep {
                                    href: format!("/{}/process/{}", lang, i),
                                    process_step: *step,
                                });

                                acc.extend(examples.iter().map(|ex| SeqStep {
                                    href: format!("/{}/process/{}/{}", lang, i, ex.id),
                                    process_step: *step,
                                }));

                                acc
                            },
                        );
                    });
                }
                Err(err) => {
                    log::error!("{err}");
                    println!("{err}");
                }
            }
        }
    });

    view! {
        <Title text={move || format!("{} | {}", t!("process.title"), t!("name"))}/>
        <noscript>
            <section class="grow lg:w-full p-8 my-6 lg:my-8 flex items-start mb-3 rounded-lg max-w-prose p-4 bg-amber-200 dark:bg-amber-950 border border-amber-400 dark:brder-amber-800 text-sky-800 dark:text-sky-200 text-lg rounded-xl shadow">
                <div class="flex flex-col">
                    <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                        <h2 class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3 text-wrap whitespace-break-spaces w-full">{t!("util.js")}</h2>
                    </div>
                </div>
            </section>
        </noscript>
        <section class="grow lg:w-full p-8 my-6 lg:my-8 bg-stone-200 dark:bg-stone-800 rounded-xl shadow">
            {move || {
                let storage_type = storage_type.get();
                view! {
                    <Transition fallback={WorksheetDummy}>
                        <WorksheetView
                            storage_type=storage_type
                        >
                            <Outlet/>
                        </WorksheetView>
                    </Transition>
                }
            }}
        </section>
        <StepperView/>
        <PrivacyNoticeView/>
    }
}

#[server(GetExamples, "/api")]
pub async fn get_examples(
    lang: Language,
    count: i64,
    offset: i64,
) -> Result<Vec<Example>, ServerFnError<String>> {
    use crate::{
        app::state::WorkSheets,
        server::{get_db_conn, xata_rest_builder},
    };
    use spin_sdk::{
        http::{send, Method, Response},
        pg::{Decode, ParameterValue},
    };

    log::debug!("Getting examples {lang:?} {count} : {offset}");

    let conn = get_db_conn().map_err(|e| e.to_string())?;

    fn coalesce_translations(table: &str, lang: &Language) -> String {
        let available_langs = rust_i18n::available_locales!();

        format!(
            r#"
            CASE
                WHEN {table}.{lang} IS NULL THEN TRUE
                ELSE FALSE
            END AS {table}_translation_fallback,
            COALESCE({table}.{lang}{})
                "#,
            available_langs
                .into_iter()
                .fold(String::default(), |acc, ln| format!("{acc},{table}.{ln}"))
        )
    }

    let sql = format!(
        r#"
    SELECT projects.xata_id as id,
    {} AS title,
    {} AS description,
    {} #>> '{{}}' AS wk,
            FROM "projects"
            LEFT JOIN "localized_text" AS lt_title ON lt_title.xata_id = projects.title
            LEFT JOIN "localized_text" AS lt_description ON lt_description.xata_id = projects.description
            LEFT JOIN "localized_json" AS lj_worksheets ON lj_worksheets.xata_id = projects.worksheets
            ORDER BY projects.weight ASC
            LIMIT $1 OFFSET $2;
        "#,
        coalesce_translations("lt_title", &lang),
        coalesce_translations("lt_description", &lang),
        coalesce_translations("lj_worksheets", &lang),
    );

    let params = vec![ParameterValue::Int64(count), ParameterValue::Int64(offset)];

    let data = conn
        .query(sql.as_str(), params.as_slice())
        .map_err(|e| e.to_string())?;

    println!("data: {data:#?}");

    let examples: Vec<_> = data
        .rows
        .into_iter()
        .try_fold(vec![], |mut acc, row| {
            let id = String::decode(&row[0])?;
            let title = String::decode(&row[2])?;
            let description = String::decode(&row[4])?;
            let wk = String::decode(&row[6])
                .map(|s| serde_json::from_str::<WorkSheets>(s.as_str()).ok())
                .ok()
                .flatten();
            let translation_warning =
                bool::decode(&row[1])? || bool::decode(&row[3])? || bool::decode(&row[5])?;
            acc.push(Example {
                id,
                title,
                description,
                wk,
                translation_warning,
            });
            Ok(acc)
        })
        .map_err(|e: anyhow::Error| e.to_string())?;

    // let project_ids = examples.iter().fold(String::default(), |acc, (id, ..)| {
    //     format!(r#"{acc}{}"{id}""#, if acc.is_empty() { "" } else { "," })
    // });

    // let mut images_req =
    //     xata_rest_builder("tables/projects/query").map_err(|e: anyhow::Error| e.to_string())?;

    // images_req.method(Method::Post).body(format!(
    //     r#"{{
    //     "columns": ["main_image.signedUrl", "xata_id"],
    //     "filter": {{ "xata_id" : {{ "$any": [{project_ids}] }} }}
    //     }}"#
    // ));

    // async move {
    // let images: Response = send(images_req).await.unwrap();

    //     println!("images: {images:#?}");
    // }
    // .await;

    Ok(examples)
}
