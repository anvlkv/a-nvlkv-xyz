use std::collections::HashMap;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::{
    components::ErrorView, projects::get_projects, state::ProjectData, use_lang,
    util::transform_xata_image,
};

#[component]
pub fn ProjectsGridView() -> impl IntoView {
    let lang = use_lang();
    let (count, _set_count) = create_query_signal::<usize>("count");
    let (offset, _set_offset) = create_query_signal::<usize>("offset");

    let projects = create_resource(
        move || {
            (
                lang.get(),
                count.get().unwrap_or(6),
                offset.get().unwrap_or(0),
            )
        },
        |(lang, count, offset)| async move { get_projects(lang, count, offset, false).await },
    );

    let projects_grid = move || match projects.get() {
        Some(projects) => {
            let (projects, estimate) = projects.map_err(|e| ServerFnErrorErr::from(e))?;
            let page_size = count.get().unwrap_or(6);
            let max_page = (estimate / page_size as f32).ceil() as usize;
            let lang = lang.get();
            let pages = (0..max_page)
                .map(|i| {
                    let href = format!(
                        "/{lang}/projects?count={page_size}&offset={}",
                        i * page_size
                    );
                    view! {
                        <A
                            href
                            exact=true
                            class="w-10 h-10 flex items-center justify-center rounded-full bg-stone-300 dark:bg-stone-950 hover:bg-stone-200 dark:hover:bg-stone-800 active:bg-stone-300 dark:active:bg-stone:700 border-2 border-solid border-slate-50 drop-shadow-sm"
                            active_class="border pointer-events-none font-bold"
                        >
                            <span>{i + 1}</span>
                        </A>
                    }
                })
                .collect_view();

            leptos::error::Result::<View>::Ok(
                view! {
                    <ul
                        class="w-full grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6"
                    >
                        <For
                            each=move || projects.clone()
                            key=|state| state.id.clone()
                            let:child
                        >
                            <li class="contents">
                                <ProjectItem data=child/>
                            </li>
                        </For>
                    </ul>
                    <Show when={move || max_page > 1}>
                        <div class="flex flex-wrap mt-6">
                            {pages.clone()}
                        </div>
                    </Show>
                }
                .into_view(),
            )
        }
        None => leptos::error::Result::<View>::Ok(view! {<ProjectsGridDummy/>}.into_view()),
    };

    view! {
        <Title text={move || format!("{} | {}", t!("projects.title"), t!("name"))}/>
        <div class="mx-auto w-full max-w-screen-2xl my-6 px-6 md:px-8 lg:px-16 min-h-full flex flex-col justify-center items-center">
            <h2 class="text-2xl md:text-3xl xl:text-4xl mb-6">
                {t!("projects.title")}
            </h2>
            <Transition fallback=|| view!{<ProjectsGridDummy/>}>
                <ErrorBoundary fallback=|err| view! { <ErrorView errors=err/>}>
                    {projects_grid}
                </ErrorBoundary>
            </Transition>
        </div>
    }
}

#[component]
pub fn ProjectItem(#[prop(into)] data: MaybeSignal<ProjectData>) -> impl IntoView {
    let lang = use_lang();

    let href = {
        let data = data.clone();
        move || {
            let lang = lang.get();
            let id = data.get().id;
            format!("/{lang}/projects/{id}")
        }
    };

    let img = Signal::derive({
        let data = data.clone();
        move || {
            data.get().main_image_url.map(|u| {
                transform_xata_image(
                    u.as_str(),
                    HashMap::from_iter(vec![
                        ("width", "460"),
                        ("height", "460"),
                        ("fit", "cover"),
                        ("format", "webp"),
                    ]),
                )
            })
        }
    });

    let title = Signal::derive({
        let data = data.clone();
        move || data.get().title
    });

    let description = Signal::derive({
        let data = data.clone();
        move || data.get().description
    });

    view! {
        <A
            href
            class="flex flex-col rounded-md shadow-sm bg-stone-200 dark:bg-stone-800 overflow-hidden hover:shadow-lg hover:scale-105"
        >
            <Show when=move || img.get().is_some()>
                <img class="w-40 aspect-square" alt={title} src={move || img.get().unwrap()}/>
            </Show>
            <h5 class="mx-4 mt-4 mb-2 font-bold text-lg">
                {title}
            </h5>
            <p class="mx-4 mb-4">
                {description}
            </p>
        </A>
    }
}

#[component]
pub fn ProjectsGridDummy() -> impl IntoView {
    let entry = || {
        view! {
            <div class="flex flex-col rounded-md shadow-sm bg-stone-200 dark:bg-stone-800 overflow-hidden">
                <div class="dummy-line w-full aspect-square bg-stone-400 dark:bg-stone-600 after:content-[' ']"></div>
                <div class="dummy-line w-40 rounded-sm mx-4 mt-4 mb-2 bg-stone-300 dark:bg-stone-700 h-6 after:content-[' ']"></div>
                <div class="dummy-line w-40 rounded-sm mx-4 mb-4 bg-stone-300 dark:bg-stone-700 h-3 after:content-[' ']"></div>
                <div class="dummy-line w-40 rounded-sm mx-4 mb-4 bg-stone-300 dark:bg-stone-700 h-3 after:content-[' ']"></div>
            </div>
        }
    };

    view! {
        <div class="w-full grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6">
            {(0..6).map(|_| entry()).collect_view()}
        </div>
    }
}
