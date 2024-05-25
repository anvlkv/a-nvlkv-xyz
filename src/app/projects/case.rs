use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::{
    components::{ErrorView, PictureModalView, PictureView},
    projects::get_project_details,
    use_lang,
};

#[derive(Params, PartialEq, Clone)]
pub struct CaseParams {
    pub id: Option<String>,
}

#[component]
pub fn CaseView() -> impl IntoView {
    let lang = use_lang();
    let params = use_params::<CaseParams>();

    let project_data = create_resource(
        move || (lang.get(), params.get()),
        |(lang, params)| async move {
            let CaseParams { id } =
                params.map_err(|_| ServerFnError::Request("Invalid param".to_string()))?;
            let id = id.ok_or(ServerFnError::Request("No id".to_string()))?;

            get_project_details(lang, id).await
        },
    );

    let project_view = move || match project_data.get() {
        Some(data) => {
            let data = data.map_err(|e| ServerFnErrorErr::from(e))?;

            let page_title = format!("{} | {} | {}", data.title, t!("case.title"), t!("name"));
            let alt = t!("util.alt_fallback", title = data.title.as_str()).to_string();
            let main_img = match data.main_image_url {
                Some(src) => {
                    let alt = data.main_image_alt.unwrap_or(alt.clone());

                    view! {
                        <div class="row-start-1 row-span-5 col-span-full">
                            <PictureView
                                src
                                alt
                                attr:class="mx-auto w-full max-w-screen-2xl"
                            />
                        </div>
                    }
                    .into_view()
                }
                None => ().into_view(),
            };

            let extra_img = data
                .images
                .iter()
                .map(|src| {
                    view! {
                        <div class="row-auto overflow-hidden">
                            <PictureModalView
                                src=src.as_str()
                                alt=alt.as_str()
                                thumbnail_size=400
                            />
                        </div>
                    }
                })
                .collect_view();

            let gallery_span_rows = (data.images.len() as f32 / 2.0).ceil() as usize;
            let article_span_rows = data.article.len();

            let (template_rows, gallery_span, article_span) = {
                let max_span_rows = usize::max(article_span_rows, gallery_span_rows);
                let rows = 5 + max_span_rows;

                (
                    format!("grid-template-rows: repeat({rows}, minmax(0, 20vh));"),
                    format!("grid-row-start: 6; grid-row-end: {};", 6 + max_span_rows),
                    format!(
                        "grid-row-start: 5; grid-row-end: {};",
                        5 + max_span_rows + 1
                    ),
                )
            };

            leptos::error::Result::<View>::Ok(
                view! {
                    <Title text={page_title}/>
                    <div class="grid lg:grid-cols-2" style={template_rows}>
                        {main_img}
                        <article
                            class="grid grid-rows-subgrid lg:col-start-2 col-span-1 p-8 bg-stone-200 dark:bg-stone-800 shadow rounded-tl-xl"
                            style={article_span}
                        >
                            <div class="rows-span-1">
                                <h3 class="text-lg">{data.title}</h3>
                                <h3 class="text-md">{data.description}</h3>
                            </div>
                            {
                                data.article.iter().map(|d| view!{
                                    <p>{d}</p>
                                }).collect_view()
                            }
                        </article>
                        <aside
                            class="grid grid-rows-subgrid grid-cols-2"
                            style={gallery_span}
                        >
                            {extra_img}
                        </aside>
                    </div>
                }
                .into_view(),
            )
        }
        None => leptos::error::Result::<View>::Ok(().into_view()),
    };

    view! {
        <section class="w-screen">
            <Transition>
                <ErrorBoundary fallback=|err| view! { <ErrorView errors=err/>}>
                    {project_view}
                </ErrorBoundary>
            </Transition>
        </section>
    }
}
