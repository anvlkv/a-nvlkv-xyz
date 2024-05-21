use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::{components::ErrorView, projects::get_project_details, use_lang};

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
            let img = match data.main_image_url {
                Some(src) => view! {
                    <div class="basis-1/2 grow-0 shrink-0 overflow-visible">
                        <img src=src class="h-screen w-auto"/>
                    </div>
                }
                .into_view(),
                None => ().into_view(),
            };

            leptos::error::Result::<View>::Ok(
                view! {
                    <Title text={page_title}/>
                    <div class="flex flex-col lg:flex-row justify-center">
                        {img}
                        <article class="basis-1/2 grow-0 shrink-0 lg:mt-auto p-8 bg-stone-200 dark:bg-stone-800 shadow rounded-tl-xl">
                            <h3 class="text-lg">{data.title}</h3>
                            {
                                data.article.iter().map(|d| view!{
                                    <p>{d}</p>
                                }).collect_view()
                            }
                        </article>
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
