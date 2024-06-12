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
            let main_img = match data.main_image_url.as_ref() {
                Some(src) => {
                    let alt = data.main_image_alt.unwrap_or(alt.clone());

                    view! {
                        <div class="row-start-1 row-span-4 lg:row-span-5 lg:row-start-1 col-span-full">
                            <PictureView
                                src=src.clone()
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
                .chain(data.main_image_url.as_ref())
                .map(|src| {
                    view! {
                        <PictureModalView
                            src=src.as_str()
                            alt=alt.as_str()
                            thumbnail_size=400
                            thumbnail_class="row-auto m-4 overflow-hidden rounded shadow"
                        />
                    }
                })
                .collect_view();

            let gallery_span_rows = (data.images.len() as f32 / 2.0).ceil() as usize;
            let article_span_rows = data.article.len();

            let max_span_rows = usize::max(article_span_rows, gallery_span_rows);

            leptos::error::Result::<View>::Ok(
                view! {
                    <Title text={page_title}/>
                    <Style>
                        {
                            format!(r#"
                            .case-grid {{
                                grid-template-rows: repeat(5, minmax(0, 15dvh)) repeat({}, 1fr);
                            }}

                            .article-grid {{
                                grid-row-start: 5;
                                grid-row-end: {};
                            }}

                            .aside-grid {{
                                grid-row-start: {};
                                grid-row-end: {};
                            }}

                            @media (min-width: 1024px) {{
                                .case-grid {{
                                    grid-template-rows: repeat(5, 19.5dvh) repeat({max_span_rows}, 1fr);
                                }}

                                .article-grid {{
                                    grid-row-start: 5;
                                    grid-row-end: {};
                                }}

                                .aside-grid {{
                                    grid-row-start: 6;
                                    grid-row-end: {};
                                }}
                            }}
                                "#,
                                gallery_span_rows + article_span_rows,
                                article_span_rows + 1 + 5,
                                article_span_rows + 1 + 5,
                                gallery_span_rows + article_span_rows + 1 + 5,
                                5 + max_span_rows + 1,
                                6 + max_span_rows
                            )
                        }
                    </Style>
                    <div class="case-grid grid grid-cols-1 lg:grid-cols-2">
                        {main_img}
                        <article
                            class="article-grid grid grid-rows-subgrid lg:col-start-2 col-span-1 p-8 bg-stone-200 dark:bg-stone-800 shadow lg:rounded-tl-xl"
                        >
                            <div class="row-span-1">
                                <h3 class="text-xl">{data.title}</h3>
                                <h3 class="text-md">{data.description}</h3>
                            </div>
                            {
                                data.article.iter().map(|d| view!{
                                    <p>{d}</p>
                                }).collect_view()
                            }
                        </article>
                        <aside
                            class="aside-grid grid grid-rows-subgrid grid-cols-2 bg-stone-800 dark:bg-stone-200 shadow"
                        >
                            {extra_img}
                        </aside>
                    </div>
                }
                .into_view(),
            )
        }
        None => leptos::error::Result::<View>::Ok(CaseDummy.into_view()),
    };

    view! {
        <section class="w-screen">
            <Transition fallback={CaseDummy}>
                <ErrorBoundary fallback=|err| view! { <ErrorView errors=err/>}>
                    {project_view}
                </ErrorBoundary>
            </Transition>
        </section>
    }
}

#[component]
fn CaseDummy() -> impl IntoView {
    view! {
        <div class="case-grid grid grid-cols-1 grid-rows-[repeat(5,_19.5dvh)_repeat(3,_1fr)] lg:grid-cols-2">
            <div class="row-start-1 row-end-6 col-start-1 lg:col-end-3">
                <div class="dummy-line w-full h-full bg-stone-400 dark:bg-stone-600 after:content-[' ']"></div>
            </div>
            <article
                class="article-grid z-[1] row-span-4 row-start-5 grid grid-rows-subgrid lg:col-start-2 col-span-1 p-8 bg-stone-200 dark:bg-stone-800 shadow lg:rounded-tl-xl"
            >
                <div class="row-span-1">
                    <div class="dummy-line w-40 rounded-sm mx-4 mb-4 bg-stone-300 dark:bg-stone-700 h-3 after:content-[' ']"></div>
                    <div class="dummy-line w-40 rounded-sm mx-4 mb-4 bg-stone-300 dark:bg-stone-700 h-3 after:content-[' ']"></div>
                </div>
                <div class="row-span-1">
                    <div class="dummy-line w-40 rounded-sm mx-4 mb-4 bg-stone-300 dark:bg-stone-700 h-3 after:content-[' ']"></div>
                    <div class="dummy-line w-40 rounded-sm mx-4 mb-4 bg-stone-300 dark:bg-stone-700 h-3 after:content-[' ']"></div>
                </div>
                <div class="row-span-1">
                    <div class="dummy-line w-40 rounded-sm mx-4 mb-4 bg-stone-300 dark:bg-stone-700 h-3 after:content-[' ']"></div>
                    <div class="dummy-line w-40 rounded-sm mx-4 mb-4 bg-stone-300 dark:bg-stone-700 h-3 after:content-[' ']"></div>
                </div>
            </article>
            <aside
                class="aside-grid row-span-3 row-start-6 grid grid-rows-subgrid grid-cols-2 bg-stone-800 dark:bg-stone-200 shadow"
            >
                <div class="dummy-line row-auto m-4 overflow-hidden rounded shadow aspect-square bg-stone-400 dark:bg-stone-600 after:content-[' ']"></div>
                <div class="dummy-line row-auto m-4 overflow-hidden rounded shadow aspect-square bg-stone-400 dark:bg-stone-600 after:content-[' ']"></div>
                <div class="dummy-line row-auto m-4 overflow-hidden rounded shadow aspect-square bg-stone-400 dark:bg-stone-600 after:content-[' ']"></div>
                <div class="dummy-line row-auto m-4 overflow-hidden rounded shadow aspect-square bg-stone-400 dark:bg-stone-600 after:content-[' ']"></div>
                <div class="dummy-line row-auto m-4 overflow-hidden rounded shadow aspect-square bg-stone-400 dark:bg-stone-600 after:content-[' ']"></div>
            </aside>
        </div>
    }
}
