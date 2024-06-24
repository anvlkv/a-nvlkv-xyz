use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};

use crate::app::{components::ErrorView, use_lang, Language};

#[component]
pub fn LinksView() -> impl IntoView {
    let lang = use_lang();
    let entries = create_resource(
        move || lang.get(),
        |lang| async move { get_links(lang).await },
    );

    view! {
        <Title text={move || format!("{} | {}", t!("links.title"), t!("name"))}/>
        <div class="mx-auto max-w-screen-xl px-6 md:px-8 lg:px-16 min-h-full flex flex-col justify-center items-center">
            <h2 class="text-2xl mt-8 mb-4 mx-auto text-center">{t!("links.title")}</h2>
            <Transition fallback=LinksDummyView>
                <ErrorBoundary fallback=|err| view! { <ErrorView errors=err/>}>
                    {move || match entries.get() {
                        Some(d) => {
                            let data = d.map_err(|e| ServerFnErrorErr::from(e))?;
                            Result::<View, ServerFnErrorErr<String>>::Ok(view!{
                                <LinksListView data/>
                            }.into_view())
                        }
                        None => {
                            Ok(
                                LinksDummyView.into_view()
                            )
                        }
                    }}
                </ErrorBoundary>
            </Transition>
        </div>
    }
}

#[component]
pub fn LinksDummyView() -> impl IntoView {
    view! {
        <ul class="grid lg:grid-cols-2 gap-4 auto-rows-min">
            {
                move || (0..4).map(|_| view!{
                    <li class="contents">
                        <div class="p-4 bg-stone-200 dark:bg-stone-800 rounded-lg shadow">
                            <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-5 md:h-7 mb-2 after:content-[' ']"></div>
                            <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-3 md:h-5 mb-2 after:content-[' ']"></div>
                        </div>
                    </li>
                }).collect_view()
            }
        </ul>
    }
}

#[component]
pub fn LinksListView(#[prop(into)] data: Vec<ExternalLink>) -> impl IntoView {
    view! {
        <ul class="grid lg:grid-cols-2 gap-4">
            {
                move || data.iter().map(|l| {
                    let l = l.clone();

                    view!{
                        <li class="contents">
                            <a
                                class="block p-4 bg-stone-200 dark:bg-stone-800 rounded-lg shadow hover:scale-105 hover:text-purple-800 dark:hover:text-purple-400 active:scale-95"
                                href={l.url}
                                target="_blank"
                            >
                                <h3 class="mb-2 text-lg text-bold">{l.title}</h3>
                                {l.description.map(|d| view!{<p>{d}</p>})}
                            </a>
                        </li>
                    }
                }).collect_view()
            }
        </ul>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExternalLink {
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub translation_warning: bool,
}

#[server(GetLinks, "/api")]
pub async fn get_links(lang: Language) -> Result<Vec<ExternalLink>, ServerFnError<String>> {
    use spin_sdk::pg::Decode;

    use crate::{
        app::util::coalesce_translations,
        server::{get_db_conn, safe_error},
    };

    println!("Getting links {lang:?}");

    let conn = get_db_conn().map_err(safe_error)?;

    let sql = format!(
        r#"
    SELECT links.xata_id as id,
    links.url as url,
    links.title as title,
    {} AS description
            FROM "links"
            LEFT JOIN "localized_text" AS lt_description ON lt_description.xata_id = links.description
            ORDER BY links.weight ASC;
        "#,
        coalesce_translations("lt_description", &lang),
    );

    let data = conn.query(sql.as_str(), &[]).map_err(safe_error)?;

    let entries_data: Vec<_> = data
        .rows
        .into_iter()
        .try_fold(vec![], |mut acc, row| {
            let url = String::decode(&row[1])?;
            let title = String::decode(&row[2])?;
            let description = Option::<String>::decode(&row[4])?;

            let translation_warning = bool::decode(&row[3])?;

            acc.push(ExternalLink {
                title,
                description,
                url,
                translation_warning,
            });
            Ok(acc)
        })
        .map_err(|e: anyhow::Error| e.to_string())?;

    Ok(entries_data)
}
